use std;
use super::{util, freq};

#[derive(Copy, Clone)]
pub struct Carrier {
    sample_freq: f32,
    pub sample: usize,
}

impl Carrier {
    pub fn new(freq: freq::Freq) -> Carrier {
        Carrier {
            sample_freq: freq.sample_freq(),
            sample: std::usize::MAX,
        }
    }

    fn inner(&self, s: usize) -> f32 {
        util::mod_trig(self.sample_freq * s as f32)
    }

    pub fn next(&mut self) -> f32 {
        self.sample = self.sample.wrapping_add(1);
        self.inner(self.sample)
    }
}
