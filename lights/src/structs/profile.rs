use std::collections::HashMap;

use crate::managers::light_manager::LightManager;
use super::color::Color;

pub struct Profile{
    lights: LightManager,
    name: String,
    on: bool,
    enabled: bool,
    data: HashMap<String, ProfileData>
}

impl Profile{
    pub fn new( name: String, on: bool, enabled: bool, lights: LightManager) -> Profile{
        return Profile { name, on, enabled, lights, data: HashMap::new() };
    }
    pub fn instance_name(&self) -> String{
        return self.name.clone();
    }
    pub fn set_instance_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        return self;
    }
    pub fn is_on(&self) -> bool{
        return self.on;
    }
    pub fn set_on(&mut self, state: bool) -> &mut Self{
        self.on = state;
        return self;
    }
    pub fn is_enabled(&self) -> bool{
        return self.enabled;
    }

    pub fn get_data(&self, key: &str) -> Option<&ProfileData>{
        return self.data.get(key);
    }
    pub fn get_int(&self, key: &str) -> Option<i32>{
        return match self.get_data(key){
            Some(x) => {
                match x{
                    ProfileData::Int(y) => Some(y.clone()),
                    _ => None
                }
            },
            _ => None
        };
    }
    pub fn get_color(&self, key: &str) -> Option<Color>{
        return match self.get_data(key){
            Some(x) => {
                match x{
                    ProfileData::Color(y) => Some(y.clone()),
                    _ => None
                }
            },
            _ => None
        };
    }

    pub fn set_data(&mut self, key: &str, value: ProfileData){
        self.data.insert(key.to_string(), value);
    }

    pub fn m(&mut self) -> &mut LightManager{
        return &mut self.lights;
    }

}

pub enum ProfileData{
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Color(Color)
}


pub trait ProfileInterface{
    fn profile_name(&self) -> String;
    fn update(&self, parent: &mut Profile) -> ();

    fn created(&self, _parent: &mut Profile){}
    fn destroy(&self, _parent: &mut Profile){}
}


#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::structs::profile::ProfileInterface {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn $crate::structs::profile::ProfileInterface> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}