use super::{util, freq};

pub struct Carrier {
    sample_freq: f64,
}

impl Carrier {
    pub fn new(freq: freq::Freq) -> Carrier {
        Carrier {
            sample_freq: freq.sample_freq(),
        }
    }

    pub fn inner(&self, s: usize) -> f64 {
        util::mod_trig(self.sample_freq * s as f64)
    }
}
