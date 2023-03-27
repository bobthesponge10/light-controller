use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};
use std::process::Command;
use fs_extra;
use toml_edit::{Document, value};
use log::*;
use serde_json::{from_str, Value};

#[cfg(windows)]
use libloading::os::windows::*;
#[cfg(not(windows))]
use libloading::os::unix::*;


use crate::managers::light_manager::LightManager;
use crate::structs::profile::*;



#[derive(Debug)]
pub enum ProfileLoaderState{
    Unloaded,
    Exists,
    Compiled(String),
    FailedCompile(String),
    Loaded
}
pub struct ProfileLoader{
    dir: String,
    name: String,
    library: Vec<Library>,
    instances: HashMap<String, Profile>,
    interface: Option<Box<dyn ProfileInterface>>,
    state: ProfileLoaderState
}

impl ProfileLoader{
    pub fn new(dir: String, name: String) -> ProfileLoader{
        return ProfileLoader{dir: dir, name: name, library: Vec::new(), instances: HashMap::new(), interface: None, state: ProfileLoaderState::Unloaded};
    }

    pub fn try_load(&mut self){
        _ = self.new_profile();
        _ = self.compile_profile();
        unsafe{
            _ = self.load_library();
        }
    }

    fn _create_new_profile(&self) -> io::Result<()>{
        let mut p = Path::new(&self.dir).join(&self.name);

        debug!("Creating new profile {}", &self.name);

        if p.exists() {
            debug!("Profile {} already exists", &self.name);
            return Ok(());
        
        }
        fs::create_dir(&p)?;
        match fs_extra::dir::copy(Path::new("default_profile"), &p, 
            &fs_extra::dir::CopyOptions::new()
            .content_only(true)
        ){
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Error moving files")),
            _ => ()
        };
        
        p = p.join("Cargo.toml");
        let toml = fs::read_to_string(&p)?;
        let mut doc = toml.parse::<Document>().expect("Invalid Doc");
        doc["package"]["name"] = value(&self.name);
        fs::write(p, doc.to_string())?;

        return Ok(());
    }

    pub fn new_profile(&mut self) -> io::Result<()>{
        return match &self.state{
            ProfileLoaderState::Unloaded => match self._create_new_profile(){
                Ok(x) => {
                    self.state = ProfileLoaderState::Exists;
                    Ok(x)},
                Err(e) => {
                    error!("Failed to create profile {}", &self.name);
                    return Err(e)
                }
            },
            _ => Ok(())
        }
    }

    fn _attempt_compile(&self) -> Result<String, String>{
        let mut p = Path::new(&self.dir).join(&self.name);
        
        if !p.exists() {return Err("Directory does not exists".to_string());}
        p = p.join("Cargo.toml");
        let path = p.to_str().unwrap();

        let mut command = Command::new("cargo");
        command.args(["build", "--manifest-path", &format!("{}", path), "--message-format", "json-diagnostic-rendered-ansi"]);

        let output = match command.output(){
            Ok(x) => match String::from_utf8(x.stdout){
                Ok(y) => y,
                Err(_) => "".to_string()
            },
            Err(_) => return Err("Failed to start compile process".to_string())
        };
        
        let mut command_output: String = String::new();
        let mut success = false;

        for line in output.split("\n"){
            if line.len() < 2{
                continue;
            }

            let v: Value = match from_str(&line){
                Ok(x) => x,
                Err(_) => continue
            };
            if v["reason"] == "build-finished"{
                success = match v["success"].as_bool(){
                    Some(x) => x,
                    None => false
                };
            }
            let rendered = match v["message"]["rendered"].as_str(){
                Some(x) => x,
                None => ""
            };

            if rendered.len() > 0{
                if command_output.len() > 0{
                    command_output += "\n";
                }
                command_output += rendered;
            }
        }

        if success{
            debug!("Sucessfully compiled {}", &self.name);
            return Ok(command_output);
        }
        debug!("Failed to compiled {}", &self.name);
        return Err(command_output);
    }

    pub fn compile_profile(&mut self) -> io::Result<()>{
        return match &self.state{
            ProfileLoaderState::Exists => match self._attempt_compile(){
                Ok(x) => {
                    self.state = ProfileLoaderState::Compiled(x);
                    Ok(())
                },
                Err(e) => {
                    self.state = ProfileLoaderState::FailedCompile(e);
                    Err(io::Error::new(io::ErrorKind::Other, "Failed to compile"))
                }
            },
            _  => Ok(())
        };
    }

    pub unsafe fn load_library(&mut self) -> io::Result<()>{
        return match &self.state{
            ProfileLoaderState::Compiled(_) => {

                // Generate Library path
                let mut p = Path::new(&self.dir).join(&self.name).join("target");
                p = p.join("debug");

                #[cfg(windows)]
                {p = p.join("main.dll")};

                #[cfg(not(windows))]
                {p = p.join("libmain.so")};
                
                debug!("Attempting load at {:?}", p);

                // Attempt to load the library
                match Library::new(&p){
                    Ok(lib) => {
                        debug!("Loaded library for {}", self.name);

                        self.library.push(lib);
                        self.state = ProfileLoaderState::Loaded;
                    },
                    Err(x) => {
                        error!("Failed to load library for {}", &self.name);
                        error!("Error: {}", x);
                        return Err(io::Error::new(io::ErrorKind::Other, "Failed to load profile"))
                    }
                }

                type ProfileCreate = extern fn() -> *mut dyn ProfileInterface;
                let lib = match self.library.last(){
                    Some(x) => x,
                    None => return Err(io::Error::new(io::ErrorKind::Other, "Failed to load profile"))
                };
                let constructor: Symbol<ProfileCreate> = match lib.get(b"_plugin_create"){
                    Err(_) => {
                        debug!("Failed to find function");
                        return Err(io::Error::new(io::ErrorKind::Other, "Failed to find load function"))},
                    Ok(x) => x
                };
                debug!("Loaded Function for {}", &self.name);

                let boxed_raw = constructor();

                let plugin = Box::from_raw(boxed_raw);
                debug!("Loaded plugin: {}", plugin.profile_name());

                let boxed_raw = constructor();

                let plugin = Box::from_raw(boxed_raw);
                self.interface = Some(plugin);

                self.state = ProfileLoaderState::Loaded;
                
                Ok(())
            },
            _ => Ok(())
        };
    }

    pub fn generate_instance(&mut self, name: String, light_manager: &LightManager) -> Result<(), ()>{
        if self.instances.contains_key(&name){
            return Err(());
        }

        return match &self.state{
            ProfileLoaderState::Loaded =>{
                let interface = match &self.interface {
                    Some(x) => x,
                    None => return Err(())
                };
                let mut p = Profile::new( name.clone(), false, false, light_manager.new_template());
                interface.created(&mut p);
                self.instances.insert(name, p);
                debug!("Created instance of {}, with name {}", interface.profile_name(), &self.name);
                Ok(())
            }
            _ => Err(())
        };
    }

    pub fn remove_instance(&mut self, name: String) -> Result<(), ()>{
        let mut instance = match self.instances.remove(&name){
            None => return Err(()),
            Some(x) => x
        };
        match &self.interface{
            Some(x) => x.destroy(&mut instance),
            None => ()
        }
        return Ok(());
    }

    pub fn get_instance_names(&self) -> Vec<String>{
        return self.instances.keys().cloned().collect();
    }

    pub fn get_instance(&self, name: String) -> Option<&Profile>{
        return self.instances.get(&name);
    }

    pub fn get_instance_mut(&mut self, name: String) -> Option<&mut Profile>{
        return self.instances.get_mut(&name);
    }

    pub fn unload(&mut self) {
        debug!("Unloading instances for {}", &self.name);

        for name in self.get_instance_names() {
            let _ = self.remove_instance(name.clone());
        }
        self.instances.clear();
        self.interface = None;
        
        for lib in self.library.drain(..) {
            drop(lib);
        }
    }

    pub fn t(&self){
        match &self.state{
            ProfileLoaderState::FailedCompile(x) => debug!("{}", x),
            ProfileLoaderState::Compiled(_) => debug!("{:?}", ProfileLoaderState::Compiled("".to_string())),
            _ => debug!("{:?}", &self.state)
        }
        
    }

    pub fn update(&mut self){
        let interface = match &self.interface{
            None => return,
            Some(x) => x
        };
        for (_, i) in &mut self.instances{
            if i.is_on(){
                interface.update(i);
            }
        }
    }

    pub fn update_light_structure(&mut self, state: &LightManager){
        for (_, instance) in &mut self.instances{
            instance.m().sync_structure(state);
        }
    }

}
impl Drop for ProfileLoader {
    fn drop(&mut self) {
        if !self.instances.is_empty() || !self.library.is_empty() {
            self.unload();
        }
    }
}