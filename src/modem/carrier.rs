pub trait Carrier {
    fn inner(&self, s: usize) -> f64;
}

pub struct Basic {
    sample_freq: f64,
}

impl Basic {
    pub fn new(freq: f64) -> Basic {
        Basic {
            sample_freq: freq,
        }
    }
}

impl Carrier for Basic {
    fn inner(&self, s: usize) -> f64 {
        self.sample_freq * s as f64
    }
}
