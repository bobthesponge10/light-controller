use super::light_primitive::*;
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct LightStrip{
    lights: Vec<Light>,
    light_type: Light,
    pin:u8,
    length: usize,
    name: String
}
#[derive(Debug, Clone)]
pub struct Bulb{
    light: Light,
    ip: String,
    name: String
}
#[derive(Debug, Clone)]
pub struct BulbGroup{
    bulbs: Vec<Bulb>,
    length: usize,
    name: String
}

impl LightStrip{
    pub fn new(name: String, pin: u8, length: usize, type_: Light) -> LightStrip{
        let mut base = type_.clone();
        base.clear();
        let mut v: Vec<Light> = Vec::with_capacity(length);
        for _ in 0..length{
            v.push(base.clone());
        }
        return LightStrip {name: name, lights: v, light_type:base, pin: pin, length: length}
    }
    pub fn new_enum(name: String, pin: u8, length: usize, type_: Light) -> LightingTypes{
        return LightingTypes::LightStrip(LightStrip::new(name, pin, length, type_));
    }

    pub fn get_pin(&self) -> u8{
        return self.pin;
    }
    pub fn set_pin(&mut self, pin: u8){
        self.pin = pin;
    }
    pub fn get_length(&self) -> usize{
        return self.length;
    }
    pub fn set_length(&mut self, length: usize){
        self.length = length;
        let diff = (length as isize) - (self.lights.len() as isize);

        if diff > 0{
            for _ in 0..diff{
                self.lights.push(self.light_type.clone());
            }
        }else if diff < 0{
            for _ in 0..(-diff){
                self.lights.pop();
            }
        }
    }
    pub fn get_name(&self) -> String{
        return self.name.clone();
    }
    pub fn set_name(&mut self, name: String){
        self.name = name;
    }
}
impl Bulb{
    pub fn new(ip: String, name: String) -> Bulb{
        return Bulb {light: Light::RGBT(RgbtLight::default()), ip: ip, name: name}
    }
    pub fn new_enum(ip: String, name: String) -> LightingTypes{
        return LightingTypes::Bulb(Bulb::new(name, ip));
    }
    pub fn get_ip(&self) -> String{
        return self.ip.clone();
    }
    pub fn set_ip(&mut self, ip: String){
        self.ip = ip;
    }
    pub fn get_name(&self) -> String{
        return self.name.clone();
    }
    pub fn set_name(&mut self, name: String){
        self.name = name;
    }
}
impl BulbGroup{
    pub fn new(name: String) -> BulbGroup{
        let v: Vec<Bulb> = Vec::new();
        return BulbGroup { bulbs: v, length: 0, name: name }
    }
    pub fn new_enum(name: String) -> LightingTypes{
        return LightingTypes::BulbGroup(BulbGroup::new(name));
    }
    pub fn add_bulb(&mut self, b: Bulb){
        self.bulbs.push(b);
        self.length = self.bulbs.len();
    }
    pub fn remove_bulb(&mut self, index: usize){
        if index < self.length{
            self.bulbs.remove(index);
            self.length = self.bulbs.len();
        }
    }
    pub fn get_name(&self) -> String{
        return self.name.clone();
    }
    pub fn set_name(&mut self, name: String){
        self.name = name;
    }
}





impl LightVec for LightStrip{
    fn _get_lights_mut(&mut self) -> Vec<&mut Light> {
        return mut_vec_to_vec_mut(&mut self.lights);
    }
    fn _get_lights(&self) -> Vec<&Light> {
        return ref_vec_to_vec_ref(&self.lights);
    }
}
impl LightVec for Bulb{
    fn _get_lights_mut(&mut self) -> Vec<&mut Light> {
        return vec![&mut self.light];
    }
    fn _get_lights(&self) -> Vec<&Light> {
        return vec![&self.light];
    }
}
impl LightVec for BulbGroup{
    fn _get_lights_mut(&mut self) -> Vec<&mut Light> {
        let mut out: Vec<&mut Light> = Vec::new();
        for i in mut_vec_to_vec_mut(&mut self.bulbs){
            let mut a = i._get_lights_mut();
            out.append(&mut a);
        }
        return out;
    }
    fn _get_lights(&self) -> Vec<&Light> {
        let mut out: Vec<&Light> = Vec::new();
        for i in ref_vec_to_vec_ref(&self.bulbs){
            let mut a = i._get_lights();
            out.append(&mut a);
        }
        return out;
    }
}

#[derive(Debug, Clone)]
pub enum LightingTypes{
    LightStrip(LightStrip),
    Bulb(Bulb),
    BulbGroup(BulbGroup)
}

impl LightVec for LightingTypes{
    fn _get_lights_mut(&mut self) -> Vec<&mut Light>{
        return match self{
            LightingTypes::LightStrip(x) => x._get_lights_mut(),
            LightingTypes::BulbGroup(x) => x._get_lights_mut(),
            LightingTypes::Bulb(x) => x._get_lights_mut()
        }
    }
    fn _get_lights(&self) -> Vec<&Light> {
        return match self{
            LightingTypes::LightStrip(x) => x._get_lights(),
            LightingTypes::BulbGroup(x) => x._get_lights(),
            LightingTypes::Bulb(x) => x._get_lights()
        }
    }
}