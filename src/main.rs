use lights::managers::profile_manager::*;
use log::*;



fn main(){
    env_logger::init();

    info!("Starting");

    let mut p = ProfileLoader::new("profiles".to_string(), "test2".to_string());

    p.new_profile();
    p.compile_profile();
    unsafe{
        p.load_library();
        p.generate_instance("el goblino".to_string());
    }
    p.t();

    info!("Closing")
}
