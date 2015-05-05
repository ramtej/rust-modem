use std;

pub struct Freq {
    // Cycles per second.
    hz: u32,
}

impl Freq {
    pub fn new(hz: u32) -> Freq {
        Freq {
            hz: hz,
        }
    }

    // Get radians per second.
    pub fn ang_freq(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.hz as f64
    }

    // Get radians per sample given the samples per second.
    pub fn sample_freq(&self, sr: u32) -> f64 {
        self.ang_freq() / sr as f64
    }
}
