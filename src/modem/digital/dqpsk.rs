/// Implements Differential QPSK (DQPSK), which encodes each symbol as a change in phase rather
/// than an absolute phase value. This removes

use std::f32::consts::PI;

use super::DigitalPhasor;
use super::util::bytes_to_bits;
use util::mod_trig;

pub struct DQPSK {
    amplitude: f32,
    phase: f32,
}

impl DQPSK {
    pub fn new(amplitude: f32, phase: f32) -> DQPSK {
        DQPSK {
            amplitude: amplitude,
            phase: phase,
        }
    }
}

impl DigitalPhasor for DQPSK {
    fn bits_per_symbol(&self) -> usize { 2 }

    fn update(&mut self, _: usize, b: &[u8]) {
        // NOTE: this implementation causes a slow accumulation of error and could be done in a
        // much better way.
        self.phase = mod_trig(self.phase + bytes_to_bits(b) as f32 * PI / 2.0);
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
    use super::DQPSK;
    use super::super::DigitalPhasor;

    #[test]
    fn test_dqpsk() {
        let mut d = DQPSK::new(1.0, 0.0);

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
