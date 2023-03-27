use lights::managers::{profile_manager::*, light_manager::*};
use lights::structs::light_types::{LightingTypes, LightStrip};
use lights::structs::light_primitive::{Light, RgbLight};
use log::*;
use std::env;



fn main(){
    #[cfg(not(debug_assertions))]
    env::set_var("RUST_LOG", "INFO");
    #[cfg(debug_assertions)]
    env::set_var("RUST_LOG", "DEBUG");

    env_logger::init();

    info!("Starting");

    let profiles_dir = "profiles".to_string();

    let mut lights: LightManager = LightManager::new();
    lights.add_light(LightingTypes::LightStrip(LightStrip::new(0, 300, Light::RGB(RgbLight::default()))));



    let mut p1 = ProfileLoader::new(profiles_dir.clone(), "test2".to_string());
    let mut p2 = ProfileLoader::new(profiles_dir.clone(), "basic-pattern".to_string());

    let mut profiles = vec![p1, p2];

    for i in &mut profiles{
        i.new_profile();
        i.compile_profile();
        unsafe{
            i.load_library();
        }
        i.t()
    }

    profiles[1].generate_instance("TEST TEST TEST".to_string(), &lights);
    for i in 0..500{
        profiles[1].update();
    }


    info!("Closing")
}
