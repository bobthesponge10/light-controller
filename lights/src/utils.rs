use crate::structs::color::Color;

pub fn mut_vec_to_vec_mut<T>(v: &mut Vec<T>) -> Vec<&mut T>{
    let s = v.len();

    let mut mut_slices_iter = v.iter_mut();
    let mut mut_slices = Vec::with_capacity(s);
    for _ in 0..s{
        mut_slices.push(
            match mut_slices_iter.next(){Some(x) => x, _ => break}
        );
    }
    return mut_slices;
}
pub fn ref_vec_to_vec_ref<T>(v: &Vec<T>) -> Vec<&T>{
    return v
        .into_iter()
        .map(|x| x)
        .collect();
}

pub fn temp_to_color(temp: u32) -> Color{
    // https://tannerhelland.com/2012/09/18/convert-temperature-rgb-algorithm-code.html

    let (mut r, mut g, mut b);
    if temp <= 66 {
        r = 255.0;
    }else{
        r = temp.try_into().unwrap();
        r = r - 60.0;
        r = 329.698727488 * (f64::powf(r,-0.1332047592));
        r = r.clamp(0.0, 255.0);
    };

    if temp <= 66{
        g = temp.try_into().unwrap();
        g = 99.4708025861 * f64::ln(g) - 161.1195681661;
        
    }else{
        g = temp.try_into().unwrap();
        g = g -60.0;
        g = 161.1195681661 * f64::powf(g, -0.0755148492);
    }
    g = g.clamp(0.0, 255.0);

    if temp >= 66{
        b = 255.0;
    }else{
        if temp <= 19{
            b = 0.0;
        }else{
            b = temp.try_into().unwrap();
            b = b - 10.0;
            b = 138.5177312231 * f64::ln(b) - 305.0447927307;
            b = b.clamp(0.0, 255.0);
        }
    }

    return Color::new(f64::round(r) as u8, f64::round(g) as u8, f64::round(b) as u8);
}