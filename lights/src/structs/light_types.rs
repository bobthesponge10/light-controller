use super::light_primitive::*;
use crate::utils::*;

#[derive(Debug)]
pub struct LightStrip{
    lights: Vec<Light>,
    pin:u8,
    length: usize
}
#[derive(Debug)]
pub struct Bulb{
    light: Light,
    ip: String,
    name: String
}
#[derive(Debug)]
pub struct BulbGroup{
    bulbs: Vec<Bulb>,
    length: usize,
    name: String
}

impl LightStrip{
    pub fn new(pin: u8, length: usize) -> LightStrip{
        let v: Vec<Light> = Vec::with_capacity(length);
        return LightStrip {lights: v, pin: pin, length: length}
    }
}
impl Bulb{
    pub fn new(ip: String, name: String) -> Bulb{
        return Bulb {light: Light::RGBT(RgbtLight::default()), ip: ip, name: name}
    }
}
impl BulbGroup{
    pub fn new(name: String) -> BulbGroup{
        let v: Vec<Bulb> = Vec::new();
        return BulbGroup { bulbs: v, length: 0, name: name }
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

#[derive(Debug)]
enum LightingTypes{
    LightStrip(LightStrip),
    Bulb(Bulb),
    BulbGroup(BulbGroup)
}
