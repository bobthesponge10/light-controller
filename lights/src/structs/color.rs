use rand::Rng;
use colored::Colorize;

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Color{
    red: u8,
    green: u8,
    blue:u8
}
impl Color{
    pub fn new(r: u8, g:u8, b:u8)->Color{
        Color {red: r, green: g, blue: b}
    }
    pub fn random()->Color{
        let mut rng = rand::thread_rng();
        Color {red: rng.gen_range(0..=255), green: rng.gen_range(0..=255), blue: rng.gen_range(0..=255)}
    }
    pub fn as_string(&self) -> String{
        return " ".on_truecolor(self.red, self.green, self.blue).to_string();
    }
}