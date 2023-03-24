use super::color::*;

pub enum Res{
    Color(Color),
    Temp(u32),
    Transparency(u8),
    Mixed,
}

pub trait ColorT{
    fn set_color(&mut self, color: Color) -> &mut Self;
    fn get_color(&self) -> Color;
}
pub trait TempT{
    fn set_temp(&mut self, temp: u32) -> &mut Self;
    fn get_temp(&self) -> u32;
}
pub trait TranspT{
    fn set_transp(&mut self, transp: u8) -> &mut Self;
    fn get_transp(&self) -> u8;
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct RgbLight{
    color: Color,
    transparency: u8
}
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct RgbtLight{
    color: Color,
    temp: u32,
    transparency: u8
}
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct TLight{
    temp: u32,
    transparency: u8
}

#[derive(Debug)]
pub enum Light{
    RGB(RgbLight),
    RGBT(RgbtLight),
    T(TLight)
}

impl ColorT for Light {
    fn set_color(&mut self, color: Color) -> &mut Self{
        match self{
            Light::RGB(s) => s.color = color,
            Light::RGBT(s) => s.color = color,
            _ => ()
        }
        return self
    }
    fn get_color(&self) -> Color{
        return match self{
            Light::RGB(s) => s.color,
            Light::RGBT(s) => s.color,
            _ => Color::default()
        }
    }
}
impl TempT for Light {
    fn set_temp(&mut self, temp: u32) -> &mut Self{
        match self{
            Light::RGBT(s) => s.temp = temp,
            Light::T(s) => s.temp = temp,
            _ => ()
        }
        return self
    }
    fn get_temp(&self) -> u32{
        return match self{
            Light::RGBT(s) => s.temp,
            Light::T(s) => s.temp,
            _ => 0
        }
    }
}
impl TranspT for Light{
    fn set_transp(&mut self, transp: u8) -> &mut Self {
        match self{
            Light::RGB(s) => s.transparency = transp,
            Light::RGBT(s) => s.transparency = transp,
            Light::T(s) => s.transparency = transp,
        }
        return self
    }
    fn get_transp(&self) -> u8 {
        return match self{
            Light::RGB(s) => s.transparency,
            Light::RGBT(s) => s.transparency,
            Light::T(s) => s.transparency
        }
    }
}
pub trait LightVec{
    fn _get_lights_mut(&mut self) -> Vec<&mut Light>;
    fn _get_lights(&self) -> Vec<&Light>;

    fn set_color(&mut self, color: Color) -> &mut Self{
        for i in self._get_lights_mut(){
            i.set_color(color);
        }
        return self;
    }
    fn set_temp(&mut self, temp: u32) -> &mut Self{
        for i in self._get_lights_mut(){
            i.set_temp(temp);
        }
        return self;
    }
    fn set_transp(&mut self, transp: u8) -> &mut Self{
        for i in self._get_lights_mut(){
            i.set_transp(transp);
        }
        return self;
    }

    fn get_color(&self) -> Res{
        let v = self._get_lights();
        let first = v.first();
        let f = match first {
            None => return Res::Color(Color::default()),
            Some(f) => f
        };
        for i in v.iter().skip(1){
            if i.get_color() != f.get_color() {return Res::Mixed}
        }

        return Res::Mixed;
    }
    fn get_temp(&self) -> Res{
        let v = self._get_lights();
        let first = v.first();
        let f = match first {
            None => return Res::Temp(0),
            Some(f) => f
        };
        for i in v.iter().skip(1){
            if i.get_temp() != f.get_temp() {return Res::Mixed}
        }

        return Res::Mixed;
    }
    fn get_transp(&self) -> Res{
        let v = self._get_lights();
        let first = v.first();
        let f = match first {
            None => return Res::Transparency(0),
            Some(f) => f
        };
        for i in v.iter().skip(1){
            if i.get_transp() != f.get_transp() {return Res::Mixed}
        }

        return Res::Mixed;
    }
}
