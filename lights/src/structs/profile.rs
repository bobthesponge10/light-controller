pub struct Profile{
    loaded: Box<dyn ProfileInterface>,
    name: String,
    on: bool,
    enabled: bool
}

impl Profile{
    pub fn new(loaded: Box<dyn ProfileInterface>, name: String, on: bool, enabled: bool) -> Profile{
        return Profile { loaded: loaded, name: name, on: on, enabled: enabled }
    }
    pub fn profile_name(&self) -> String{
        return self.loaded.profile_name();
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
    fn is_enabled(&self) -> bool{
        return self.enabled;
    }
}

pub trait ProfileInterface{
    fn profile_name(&self) -> String;
    //fn set_lights(&mut self, manager: Light_Manager) -> ();
    //fn update(&self) -> ();
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