use num::complex::Complex32;

use data::{Source, SourceUpdate};
use phasor::Phasor;
use carrier::Carrier;
use digital::DigitalPhasor;

fn real(i: f32, q: f32, cos: f32, sin: f32) -> f32 {
    i * cos - q * sin
}

fn imag(i: f32, q: f32, cos: f32, sin: f32) -> f32 {
    i * sin + q * cos
}

pub struct Modulator {
    carrier: Carrier,
    phasor: Box<Phasor>,
}

impl Modulator {
    pub fn new(c: Carrier, psr: Box<Phasor>) -> Modulator {
        Modulator {
            carrier: c,
            phasor: psr,
        }
    }

    pub fn into_carrier(self) -> Carrier { self.carrier }
}

impl Iterator for Modulator {
    type Item = Complex32;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        let (i, q) = match self.phasor.next(self.carrier.sample) {
            Some((i, q)) => (i, q),
            None => return None,
        };

        let cos = phase.cos();
        let sin = phase.sin();

        Some(Complex32::new(real(i, q, cos, sin), imag(i, q, cos, sin)))
    }
}

pub struct DigitalModulator {
    start_sample: usize,
    data: Box<Source>,
    carrier: Carrier,
    phasor: Box<DigitalPhasor>,
}

impl DigitalModulator {
    pub fn new(c: Carrier, psr: Box<DigitalPhasor>, src: Box<Source>) -> DigitalModulator {
        DigitalModulator {
            start_sample: c.sample + 1,
            data: src,
            carrier: c,
            phasor: psr,
        }
    }

    pub fn into_carrier(self) -> Carrier { self.carrier }
}

impl Iterator for DigitalModulator {
    type Item = Complex32;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        let bits = match self.data.update(self.carrier.sample - self.start_sample) {
            SourceUpdate::Finished => return None,
            SourceUpdate::Changed(b) => {
                self.phasor.update(self.carrier.sample, b);
                b
            }
            SourceUpdate::Unchanged(b) => b,
        };

        let (i, q) = self.phasor.next(self.carrier.sample, bits);

        let cos = phase.cos();
        let sin = phase.sin();

        Some(Complex32::new(real(i, q, cos, sin), imag(i, q, cos, sin)))
    }
}
