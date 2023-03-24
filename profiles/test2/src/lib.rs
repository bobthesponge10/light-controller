#[macro_use]
extern crate lights;
use lights::structs::profile::*;

#[derive(Debug, Default)]
pub struct profile;

impl ProfileInterface for profile{
    #[no_mangle]
    fn profile_name(&self) -> String{
        return "Default Profile Name".to_string();
    }
    // fn update(&mut self) -> (){}
}


declare_plugin!(profile, profile::default);
