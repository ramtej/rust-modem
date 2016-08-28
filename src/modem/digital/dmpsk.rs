/// Implements Differential M-ary PSK (DMPSK), which encodes each symbol as a change in
/// phase rather than an absolute phase value. This removes

use super::DigitalPhasor;
use super::util::bytes_to_bits;
use util::mod_trig;

pub struct DMPSK {
    bits_per_symbol: usize,
    amplitude: f32,
    phase: f32,
    shift: f32,
}

impl DMPSK {
    pub fn new(bits_per_symbol: usize, amplitude: f32, phase: f32, shift: f32) -> DMPSK {
        DMPSK {
            bits_per_symbol: bits_per_symbol,
            amplitude: amplitude,
            phase: phase,
            shift: shift,
        }
    }
}

impl DigitalPhasor for DMPSK {
    fn bits_per_symbol(&self) -> usize { self.bits_per_symbol }

    fn update(&mut self, _: usize, b: &[u8]) {
        // NOTE: this implementation causes a slow accumulation of error and could be done
        // in a much better way.
        self.phase = mod_trig(self.phase + bytes_to_bits(b) as f32 * self.shift);
    }

    fn i(&self, _: usize, _: &[u8]) -> f32 {
        self.amplitude * self.phase.cos()
    }

    fn q(&self, _: usize, _: &[u8]) -> f32 {
        self.amplitude * self.phase.sin()
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;
    use super::DMPSK;
    use super::super::DigitalPhasor;

    #[test]
    fn test_dmpsk() {
        let mut d = DMPSK::new(2, 1.0, 0.0, PI / 2.0);

        assert!((d.i(0, &[]) - 1.0).abs() < 0.000001);
        assert!((d.q(0, &[]) - 0.0).abs() < 0.000001);

        d.update(123, &[0, 0]);
        assert!((d.i(0, &[]) - 1.0).abs() < 0.000001);
        assert!((d.q(0, &[]) - 0.0).abs() < 0.000001);

        d.update(123, &[0, 1]);
        assert!((d.i(0, &[]) - 0.0).abs() < 0.000001);
        assert!((d.q(0, &[]) - 1.0).abs() < 0.000001);

        d.update(123, &[1, 0]);
        assert!((d.i(0, &[]) - 0.0).abs() < 0.000001);
        assert!((d.q(0, &[]) - -1.0).abs() < 0.000001);

        d.update(123, &[1, 1]);
        assert!((d.i(0, &[]) - -1.0).abs() < 0.000001);
        assert!((d.q(0, &[]) - 0.0).abs() < 0.000001);

        d.update(123, &[0, 0]);
        assert!((d.i(0, &[]) - -1.0).abs() < 0.000001);
        assert!((d.q(0, &[]) - 0.0).abs() < 0.000001);

        d.update(123, &[0, 0]);
        assert!((d.i(0, &[]) - -1.0).abs() < 0.000001);
        assert!((d.q(0, &[]) - 0.0).abs() < 0.000001);

        d.update(123, &[1, 1]);
        assert!((d.i(0, &[]) - 0.0).abs() < 0.000001);
        assert!((d.q(0, &[]) - 1.0).abs() < 0.000001);
    }
}
