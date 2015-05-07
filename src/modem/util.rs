use std;

pub fn mod_trig(x: f64) -> f64 {
    const TWO_PI: f64 = std::f64::consts::PI * 2.0;

    x - TWO_PI * (x / TWO_PI).floor()
}
