#[macro_use]
extern crate lights;
use lights::structs::profile::{*, ProfileData as P};
use lights::structs::light_types::*;
use lights::structs::light_primitive::*;
use lights::structs::color::*;

#[derive(Debug, Default)]
pub struct profile;

impl ProfileInterface for profile{
    fn profile_name(&self) -> String{
        return "Basic Pattern".to_string();
    }

    fn created(&self, parent: &mut Profile) -> (){
        let mut max = 0;
        parent.set_data("max", P::Int(300));
        parent.set_data("current", P::Int(0));
        for i in parent.m().get_light_strip_ids(){
            if let LightingTypes::LightStrip(x) = parent.m().get_light(i).unwrap(){
                if x.size() > max{
                    max = x.size();
                }
            }
        }
        self.set_color(parent);
    }

    fn update(&self, parent: &mut Profile) -> (){
        let mut current = parent.get_int("current").unwrap();
        let max = parent.get_int("max").unwrap();
        let color = parent.get_color("color").unwrap();

        for i in parent.m().get_light_strip_ids(){
            if let LightingTypes::LightStrip(x) = parent.m().get_light_mut(i).unwrap(){
                x.set_color_index(color, current.try_into().unwrap());
            }
        }

        current += 1;
        if current >= max{
            current = 0;
            self.set_color(parent);
        }

        parent.set_data("current", P::Int(current));
    }
}

impl profile{
    fn set_color(&self, parent: &mut Profile){
        parent.set_data("color", P::Color(Color::random()));
    }
    fn set_bulbs(&self, parent: &mut Profile){
        let color = parent.get_color("color").unwrap();

        for i in parent.m().get_all_bulb_ids(){
            if let LightingTypes::BulbGroup(x) = parent.m().get_light_mut(i).unwrap(){
                x.set_color(color);
            }
            if let LightingTypes::Bulb(x) = parent.m().get_light_mut(i).unwrap(){
                x.set_color(color);
            }
        }
    }
}


declare_plugin!(profile, profile::default);
