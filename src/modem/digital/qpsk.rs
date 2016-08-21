use super::DigitalPhasor;
use super::util::bit_to_sign;

pub struct QPSK {
    phase_cos: f32,
    phase_sin: f32,
    amplitude: f32,
}

impl QPSK {
    pub fn new(phase: f32, amplitude: f32) -> QPSK {
        QPSK {
            phase_cos: phase.cos(),
            phase_sin: phase.sin(),
            amplitude: amplitude * 0.5f32.sqrt(),
        }
    }
}

impl DigitalPhasor for QPSK {
    fn bits_per_symbol(&self) -> usize { 2 }

    fn i(&self, _: usize, b: &[u8]) -> f32 {
        self.amplitude * (
            bit_to_sign(b[0]) as f32 * self.phase_cos -
            bit_to_sign(b[1]) as f32 * self.phase_sin
        )
    }

    fn q(&self, _: usize, b: &[u8]) -> f32 {
        self.amplitude * (
            bit_to_sign(b[1]) as f32 * self.phase_cos +
            bit_to_sign(b[0]) as f32 * self.phase_sin
        )
    }
}
