use std;
use super::{util, freq};

pub struct Carrier {
    sample_freq: f64,
    pub sample: usize,
}

impl Carrier {
    pub fn new(freq: freq::Freq) -> Carrier {
        Carrier {
            sample_freq: freq.sample_freq(),
            sample: std::usize::MAX,
        }
    }

    fn inner(&self, s: usize) -> f64 {
        util::mod_trig(self.sample_freq * s as f64)
    }

    pub fn next(&mut self) -> f64 {
        self.sample += 1;
        self.inner(self.sample)
    }
}
