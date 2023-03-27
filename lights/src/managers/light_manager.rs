use std::collections::HashMap;

use crate::structs::{light_types::*, light_primitive::*};

#[derive(Debug, Clone)]
pub struct LightManager{
    next_id: u32,
    lights: HashMap<u32, LightingTypes>
}

impl LightManager{
    pub fn new() -> LightManager{
        return LightManager { next_id: 0, lights: HashMap::new() }
    }
    pub fn new_template(&self) -> LightManager{
        let mut out = self.clone();
        out.clear();
        return out;
    }
    pub fn as_string(&self) -> String{
        let mut out = String::new();
        let lst = self.get_light_strip_ids();
        if lst.len() > 0{
            out += "Light Strips:\n";
            for i in lst{
                out = (out + self.get_light(i).unwrap().as_string().as_str()) + "\n";
            }
        }
        let lst = self.get_bulb_group_ids();
        if lst.len() > 0{
            out += "Bulb Groups:\n";
            for i in lst{
                out = (out + self.get_light(i).unwrap().as_string().as_str()) + "\n";
            }
        }
        let lst = self.get_bulb_ids();
        if lst.len() > 0{
            out += "Bulbs:\n";
            for i in lst{
                out = (out + self.get_light(i).unwrap().as_string().as_str()) + "\n";
            }
        }
        return out;
    }
    fn get_id(&mut self) -> u32{
        while self.lights.contains_key(&self.next_id){
            self.next_id += 1;
        }
        let out = self.next_id;
        self.next_id += 1;
        return out;
    }

    pub fn add_light(&mut self, light: LightingTypes){
        let next_id = self.get_id();
        self.lights.insert(next_id, light.clone());
    }
    pub fn remove_light(&mut self, id: u32){
        self.lights.remove(&id);
    }

    pub fn clear(&mut self){
        for (_, i) in &mut self.lights{
            match i {
                LightingTypes::LightStrip(x) => x.clear(),
                LightingTypes::Bulb(x) => x.clear(),
                LightingTypes::BulbGroup(x) => x.clear()
            }
        }
    }

    pub fn get_light_strip_ids(&self) -> Vec<u32> {
        let mut out:Vec<u32> = Vec::new();
        for (id, i) in &self.lights{
            match i{
                LightingTypes::LightStrip(_) => out.push(id.clone()),
                _ => ()
            }
        }
        return out;
    }
    pub fn get_bulb_ids(&self) -> Vec<u32> {
        let mut out:Vec<u32> = Vec::new();
        for (id, i) in &self.lights{
            match i{
                LightingTypes::Bulb(_) => out.push(id.clone()),
                _ => ()
            }
        }
        return out;
    }
    pub fn get_bulb_group_ids(&self) -> Vec<u32> {
        let mut out:Vec<u32> = Vec::new();
        for (id, i) in &self.lights{
            match i{
                LightingTypes::BulbGroup(_) => out.push(id.clone()),
                _ => ()
            }
        }
        return out;
    }
    pub fn get_all_bulb_ids(&self) -> Vec<u32> {
        let mut out:Vec<u32> = Vec::new();
        for (id, i) in &self.lights{
            match i{
                LightingTypes::Bulb(_) => out.push(id.clone()),
                LightingTypes::BulbGroup(_) => out.push(id.clone()),
                _ => ()
            }
        }
        return out;
    }


    pub fn get_light_mut(&mut self, id: u32)->Option<&mut LightingTypes>{
        return self.lights.get_mut(&id);
    }
    pub fn get_light(&self, id: u32)->Option<&LightingTypes>{
        return self.lights.get(&id);
    }

}