use super::DigitalPhasor;
use super::util::bit_to_sign;

pub struct OQPSK {
    amplitude: f32,
}

impl OQPSK {
    pub fn new(amplitude: f32) -> OQPSK {
        OQPSK {
            amplitude: amplitude * 0.5f32.sqrt(),
        }
    }
}

impl DigitalPhasor for OQPSK {
    fn bits_per_symbol(&self) -> usize { 2 }

    fn i(&self, _: usize, b: &[u8]) -> f32 {
        bit_to_sign(b[0]) * self.amplitude
    }

    fn q(&self, _: usize, b: &[u8]) -> f32 {
        bit_to_sign(b[1]) * self.amplitude
    }
}
