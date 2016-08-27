use std::f32::consts::PI;

use super::DigitalPhasor;
use super::util::bit_to_sign;

pub struct MSK {
    amplitude: f32,
    samples_per_bit: usize,
}

impl MSK {
    pub fn new(amplitude: f32, samples_per_symbol: usize) -> MSK {
        assert!(samples_per_symbol % 2 == 0);

        MSK {
            amplitude: amplitude,
            samples_per_bit: samples_per_symbol / 2,
        }
    }

    fn inner(&self, s: usize) -> f32 {
        PI / 2.0 * s as f32 / self.samples_per_bit as f32
    }
}

impl DigitalPhasor for MSK {
    fn bits_per_symbol(&self) -> usize { 2 }

    fn i(&self, s: usize, b: &[u8]) -> f32 {
        self.amplitude * bit_to_sign(b[0]) * self.inner(s).cos()
    }

    fn q(&self, s: usize, b: &[u8]) -> f32 {
        -self.amplitude * bit_to_sign(b[1]) * self.inner(s).sin()
    }
}
