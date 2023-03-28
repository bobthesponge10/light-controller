extern crate std;
use std::collections::HashMap;
use std::path::Path;
use std::fs;


use crate::{managers::{profile_manager::*, light_manager::*}, structs::{light_types::LightingTypes, profile::*}};

pub struct System{
    profiles_dir: String,
    profiles: HashMap<String, ProfileLoader>,
    light_state: LightManager
}

impl System{
    pub fn new(profiles_dir: String) -> System{
        return System{profiles_dir, profiles: HashMap::new(), light_state: LightManager::new()}
    }

    pub fn init(&mut self){
        let p = Path::new(&self.profiles_dir);
        let paths = fs::read_dir(p).unwrap();

        for path_res in paths{
            let path = match path_res{
                Ok(x) => x.path(),
                Err(_) => continue
            };
            
            if path.is_dir(){
                let dir_name = match path.file_name(){
                    None => continue,
                    Some(x) => match x.to_str(){
                        None => continue,
                        Some(y) => y.to_string()
                    }
                };
                _ = self.add_profile(dir_name);
            }
        }
    }

    pub fn add_profile(&mut self, name: String) -> Result<(), ()>{
        if self.profiles.contains_key(&name){
            return Err(());
        }
        let mut p = ProfileLoader::new(self.profiles_dir.clone(), name.clone());
        p.try_load();
        self.profiles.insert(name, p);
        return Ok(());
    }
    pub fn remove_profile(&mut self, name: String) -> Result<(), ()>{
        return match self.profiles.remove(&name){
            Some(_) => Ok(()),
            None => Err(())
        };
    }
    pub fn get_profile(&self, name: String) -> Option<&ProfileLoader>{
        return self.profiles.get(&name);
    }
    pub fn get_profile_mut(&mut self, name: String) -> Option<&mut ProfileLoader>{
        return self.profiles.get_mut(&name);
    }
    pub fn get_profile_names(&self) -> Vec<String>{
        return self.profiles.keys().cloned().collect();
    }


    pub fn create_instance(&mut self, profile_name: String, instance_name: String) -> Result<(), ()>{
        match self.profiles.get_mut(&profile_name){
            None => Err(()),
            Some(x) => {
                x.generate_instance(instance_name, &self.light_state)
            }
        }
    }
    pub fn remove_instance(&mut self, profile_name: String, instance_name: String) -> Result<(), ()>{
        match self.profiles.get_mut(&profile_name){
            None => Err(()),
            Some(x) => {
                x.remove_instance(instance_name)
            }
        }
    }
    pub fn get_instance(&self, profile_name: String, instance_name: String) -> Option<&Profile>{
        return match self.profiles.get(&profile_name){
            None => None,
            Some(x) => {
                match x.get_instance(instance_name){
                    None => None,
                    Some(y) => Some(y)
                }
            }
        };
    }
    pub fn get_instance_mut(&mut self, profile_name: String, instance_name: String) -> Option<&mut Profile>{
        return match self.profiles.get_mut(&profile_name){
            None => None,
            Some(x) => {
                match x.get_instance_mut(instance_name){
                    None => None,
                    Some(y) => Some(y)
                }
            }
        };
    }
    pub fn get_instances_key(&self) -> Vec<(String, String)>{
        let mut out = Vec::new();
        for profile in self.get_profile_names(){
            let p = match self.get_profile(profile.clone()){
                None => continue,
                Some(x) => x
            };
            for interface in p.get_instance_names(){
                out.push((profile.clone(), interface));
            }
        }

        return out;
    }


    pub fn add_light(&mut self, light: LightingTypes)->u32{
        let out = self.light_state.add_light(light);
        self.update_light_structure();
        return out;
    }
    pub fn remove_light(&mut self, id:u32){
        self.light_state.remove_light(id);
        self.update_light_structure();
    }
    pub fn get_light(&self, id:u32) -> Option<&LightingTypes>{
        return self.light_state.get_light(id);
    }
    pub fn get_light_mut(&mut self, id:u32) -> Option<&mut LightingTypes>{
        return self.light_state.get_light_mut(id);
    }
    pub fn get_lights_id(&self) -> Vec<u32>{
        return self.light_state.get_all_ids();
    }


    pub fn update(&mut self){
        for (_, i) in &mut self.profiles{
            i.update();
        }
    }

    //pub fn update_light_state()

    fn update_light_structure(&mut self){
        for (_, i) in &mut self.profiles{
            i.update_light_structure(&self.light_state);
        }
    }
}