use lights::lighting_system::*;
use lights::structs::{light_types::*, light_primitive::*};
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

    let mut system = System::new(profiles_dir);
    system.init();

    _ = system.create_instance("basic-pattern".to_string(), "Basic Pattern Test".to_string());
    system.get_instance_mut("basic-pattern".to_string(), "Basic Pattern Test".to_string()).unwrap().set_on(true);
    _ = system.add_light(LightStrip::new_enum("main strip".to_string(), 0, 300, RgbLight::default_enum()));

    for _ in 0..500{
        system.update();
    }


    info!("Closing")
}
