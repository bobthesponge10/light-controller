use std::path::Path;
use std::{fs, io};
use std::process::Command;
use fs_extra;
use toml_edit::{Document, value};
use log::*;
use serde_json::{from_str, Value};
use libloading::*;

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
    instances: Vec<Profile>,
    state: ProfileLoaderState
}

impl ProfileLoader{
    pub fn new(dir: String, name: String) -> ProfileLoader{
        return ProfileLoader{dir: dir, name: name, library: Vec::new(), instances: Vec::new(), state: ProfileLoaderState::Unloaded};
    }

    fn create_new_profile(&self) -> io::Result<()>{
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
            ProfileLoaderState::Unloaded => match self.create_new_profile(){
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

    fn attempt_compile(&self) -> Result<String, String>{
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
            return Ok(command_output);
        }
        return Err(command_output);
    }

    pub fn compile_profile(&mut self) -> io::Result<()>{
        return match &self.state{
            ProfileLoaderState::Exists => match self.attempt_compile(){
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
                let mut p = Path::new(&self.dir).join(&self.name).join("target");
                p = p.join("debug");
                p = p.join("main");

                debug!("{}", &p.to_string_lossy());
                
                match Library::new(&p){
                    Ok(lib) => {
                        debug!("Loaded Library for {}", self.name);

                        self.library.push(lib);
                        self.state = ProfileLoaderState::Loaded;
                        Ok(())
                    },
                    Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Failed to load profile"))
                }
            },
            _ => Ok(())
        };
        

    }

    pub unsafe fn generate_instance(&mut self, name: String) -> Result<(), ()>{
        type ProfileCreate = unsafe fn() -> *mut dyn ProfileInterface;
        let lib = match self.library.last(){
            Some(x) => x,
            None => return Err(())
        };

        let constructor: Symbol<ProfileCreate> = match lib.get(b"_plugin_create"){
            Err(_) => {
                debug!("Failed to find function");
                return Err(())},
            Ok(x) => x
        };
        debug!("Loaded Function for {}", &self.name);
        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);
        debug!("Loaded plugin: {}", plugin.profile_name());
        
        let p = Profile::new(plugin, name, false, false);

        self.instances.push(p);
        
        return Ok(());
    }

    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for profile in self.instances.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", profile.profile_name());
            //plugin.on_plugin_unload();
        }
        
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

}
impl Drop for ProfileLoader {
    fn drop(&mut self) {
        debug!("Dropping");
        if !self.instances.is_empty() || !self.library.is_empty() {
            self.unload();
        }
    }
}