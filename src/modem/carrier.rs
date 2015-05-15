use super::util;

pub struct Carrier {
    sample_freq: f64,
}

impl Carrier {
    pub fn new(freq: f64) -> Carrier {
        Carrier {
            sample_freq: freq,
        }
    }

    pub fn inner(&self, s: usize) -> f64 {
        util::mod_trig(self.sample_freq * s as f64)
    }
}

pub struct CarrierSignal<'a> {
    carrier: &'a Carrier,
    sample: usize,
}

impl<'a> CarrierSignal<'a> {
    pub fn new(carrier: &'a Carrier) -> CarrierSignal<'a> {
        CarrierSignal {
            carrier: carrier,
            sample: 0,
        }
    }
}

impl<'a> Iterator for CarrierSignal<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        let sample = self.sample;
        self.sample += 1;

        Some(self.carrier.inner(sample).cos())
    }
}
