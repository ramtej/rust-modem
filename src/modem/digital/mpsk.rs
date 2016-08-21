use std::f32::consts::PI;

use super::DigitalPhasor;
use super::util::bytes_to_bits;

pub struct MPSK {
    bits_per_symbol: usize,
    num_symbols: f32,
    amplitude: f32,
    phase_offset: f32,
}

impl MPSK {
    pub fn new(bits_per_symbol: usize, phase_offset: f32, amplitude: f32) -> MPSK {
        MPSK {
            bits_per_symbol: bits_per_symbol,
            num_symbols: (1 << bits_per_symbol) as f32,
            amplitude: amplitude,
            phase_offset: phase_offset,
        }
    }

    fn inner(&self, b: &[u8]) -> f32 {
        self.phase(b) + self.phase_offset
    }

    fn phase(&self, b: &[u8]) -> f32 {
        2.0 * PI * bytes_to_bits(b) as f32 / self.num_symbols
    }
}

impl DigitalPhasor for MPSK {
    fn bits_per_symbol(&self) -> usize { self.bits_per_symbol }

    fn i(&self, _: usize, b: &[u8]) -> f32 {
        self.amplitude * self.inner(b).cos()
    }

    fn q(&self, _: usize, b: &[u8]) -> f32 {
        self.amplitude * self.inner(b).sin()
    }
}

#[cfg(test)]
mod test {
    use super::MPSK;
    use digital::DigitalPhasor;

    #[test]
    fn test_mpsk() {
        let mpsk = MPSK::new(2, 0.0, 1.0);
        assert_eq!(mpsk.i(0, &[0, 0]), 1.0);
        assert_eq!(mpsk.q(0, &[0, 0]), 0.0);

        assert!(mpsk.i(0, &[0, 1]).abs() < 0.001);
        assert_eq!(mpsk.q(0, &[0, 1]), 1.0);

        assert_eq!(mpsk.i(0, &[1, 0]), -1.0);
        assert!(mpsk.q(0, &[1, 0]).abs() < 0.001);

        assert!(mpsk.i(0, &[1, 1]).abs() < 0.001);
        assert_eq!(mpsk.q(0, &[1, 1]), -1.0);
    }
}
