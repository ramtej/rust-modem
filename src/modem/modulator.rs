extern crate num;

use super::{phasor, digital, carrier, data};

fn real(i: f64, q: f64, cos: f64, sin: f64) -> f64 {
    i * cos - q * sin
}

fn imag(i: f64, q: f64, cos: f64, sin: f64) -> f64 {
    i * sin + q * cos
}

pub struct Modulator {
    carrier: carrier::Carrier,
    phasor: Box<phasor::Phasor>,
}

impl Modulator {
    pub fn new(c: carrier::Carrier, psr: Box<phasor::Phasor>) -> Modulator {
        Modulator {
            carrier: c,
            phasor: psr,
        }
    }

    pub fn into_carrier(self) -> carrier::Carrier { self.carrier }
}

impl Iterator for Modulator {
    type Item = num::Complex<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        let (i, q) = match self.phasor.next(self.carrier.sample) {
            Some((i, q)) => (i, q),
            None => return None,
        };

        let cos = phase.cos();
        let sin = phase.sin();

        Some(num::Complex::new(real(i, q, cos, sin), imag(i, q, cos, sin)))
    }
}

pub struct DigitalModulator {
    start_sample: usize,
    data: Box<data::Source>,
    carrier: carrier::Carrier,
    phasor: Box<digital::DigitalPhasor>,
}

impl DigitalModulator {
    pub fn new(c: carrier::Carrier, psr: Box<digital::DigitalPhasor>,
               src: Box<data::Source>)
        -> DigitalModulator
    {
        DigitalModulator {
            start_sample: c.sample + 1,
            data: src,
            carrier: c,
            phasor: psr,
        }
    }

    pub fn into_carrier(self) -> carrier::Carrier { self.carrier }
}

impl Iterator for DigitalModulator {
    type Item = num::Complex<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        let bits = match self.data.update(self.carrier.sample - self.start_sample) {
            data::SourceUpdate::Finished => return None,
            data::SourceUpdate::Changed(b) => {
                self.phasor.update(self.carrier.sample, b);
                b
            }
            data::SourceUpdate::Unchanged(b) => b,
        };

        let (i, q) = self.phasor.next(self.carrier.sample, bits);

        let cos = phase.cos();
        let sin = phase.sin();

        Some(num::Complex::new(real(i, q, cos, sin), imag(i, q, cos, sin)))
    }
}
