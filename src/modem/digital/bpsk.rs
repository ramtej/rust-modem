use super::DigitalPhasor;
use super::util::bit_to_sign;

pub struct BPSK {
    phase: f32,
    amplitude: f32,
}

impl BPSK {
    pub fn new(phase: f32, amplitude: f32) -> BPSK {
        BPSK {
            phase: phase,
            amplitude: amplitude,
        }
    }

    fn common(&self, b: u8) -> f32 {
        bit_to_sign(b) * self.amplitude
    }
}

impl DigitalPhasor for BPSK {
    fn bits_per_symbol(&self) -> usize { 1 }

    fn i(&self, _: usize, b: &[u8]) -> f32 {
        self.common(b[0]) * self.phase.cos()
    }

    fn q(&self, _: usize, b: &[u8]) -> f32 {
        self.common(b[0]) * self.phase.sin()
    }
}
