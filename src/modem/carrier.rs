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
            sample: 0,
        }
    }

    fn inner(&self, s: usize) -> f32 {
        util::mod_trig(self.sample_freq * s as f32)
    }

    pub fn next(&mut self) -> f32 {
        let sample = self.sample;
        self.sample += 1;

        self.inner(sample)
    }
}
