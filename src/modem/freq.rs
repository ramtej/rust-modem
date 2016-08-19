use std;

#[derive(Copy, Clone)]
pub struct Freq {
    // Cycles per second.
    hz: usize,
    sr: usize,
}

impl Freq {
    pub fn new(hz: usize, sr: usize) -> Freq {
        Freq {
            hz: hz,
            sr: sr,
        }
    }

    // Get radians per second.
    pub fn ang_freq(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.hz as f32
    }

    // Get radians per sample given the samples per second.
    pub fn sample_freq(&self) -> f32 {
        self.ang_freq() / self.sr as f32
    }
}
