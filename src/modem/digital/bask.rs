use super::DigitalPhasor;

pub struct BASK {
    amplitude: f32,
}

impl BASK {
    pub fn new(a: f32) -> BASK {
        BASK {
            amplitude: a,
        }
    }
}

impl DigitalPhasor for BASK {
    fn bits_per_symbol(&self) -> usize { 1 }

    fn i(&self, _: usize, b: &[u8]) -> f32 {
        b[0] as f32 * self.amplitude
    }

    fn q(&self, _: usize, _: &[u8]) -> f32 {
        0.0
    }
}
