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
    pub fn ang_freq(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.hz as f64
    }

    // Get radians per sample given the samples per second.
    pub fn sample_freq(&self) -> f64 {
        self.ang_freq() / self.sr as f64
    }

    // Calculate the number of samples needed to go through the given number of
    // cycles of the wave.
    pub fn samples_for_cycles(&self, cycles: usize) -> usize {
        (self.sr as f64 / self.hz as f64 * cycles as f64) as usize
    }
}
