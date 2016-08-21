use freq::Freq;
use util::mod_trig;

use super::DigitalPhasor;

pub struct BFSK {
    deviation: f32,
    amplitude: f32,
    phase: f32,
    prev: u8,
}

impl BFSK {
    pub fn new(d: Freq, a: f32) -> BFSK {
        BFSK {
            deviation: d.sample_freq(),
            amplitude: a,
            phase: 0.0,
            prev: 0,
        }
    }

    fn inner(&self, s: usize, b: u8) -> f32 {
        self.rads(s, b) + self.phase
    }

    fn rads(&self, s: usize, b: u8) -> f32 {
        b as f32 * self.deviation * s as f32
    }
}

impl DigitalPhasor for BFSK {
    fn bits_per_symbol(&self) -> usize { 1 }

    fn i(&self, s: usize, b: &[u8]) -> f32 {
        self.amplitude * self.inner(s, b[0]).cos()
    }

    fn q(&self, s: usize, b: &[u8]) -> f32 {
        self.amplitude * self.inner(s, b[0]).sin()
    }

    fn update(&mut self, s: usize, b: &[u8]) {
        if b[0] == self.prev {
            return;
        }

        self.phase = mod_trig(self.phase + if b[0] == 1 {
            -self.rads(s, 1)
        } else {
            self.rads(s - 1, 1)
        });

        self.prev = b[0];
    }
}
