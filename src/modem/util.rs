use std;

pub fn mod_trig(x: f32) -> f32 {
    const TWO_PI: f32 = std::f32::consts::PI * 2.0;
    x - TWO_PI * (x / TWO_PI).floor()
}
