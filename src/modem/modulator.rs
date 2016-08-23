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

pub struct Modulator<'a> {
    carrier: &'a mut Carrier,
    phasor: Box<Phasor>,
}

impl<'a> Modulator<'a> {
    pub fn new(c: &'a mut Carrier, psr: Box<Phasor>) -> Modulator<'a> {
        Modulator {
            carrier: c,
            phasor: psr,
        }
    }
}

impl<'a> Iterator for Modulator<'a> {
    type Item = Complex32;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        let (i, q) = match self.phasor.next(self.carrier.sample) {
            Some((i, q)) => (i, q),
            None => return None,
        };

        let (sin, cos) = phase.sin_cos();

        Some(Complex32::new(real(i, q, cos, sin), imag(i, q, cos, sin)))
    }
}

pub struct DigitalModulator<'a> {
    data: Box<Source>,
    carrier: &'a mut Carrier,
    phasor: Box<DigitalPhasor>,
}

impl<'a> DigitalModulator<'a> {
    pub fn new(c: &'a mut Carrier, psr: Box<DigitalPhasor>, src: Box<Source>)
        -> DigitalModulator<'a>
    {
        DigitalModulator {
            data: src,
            carrier: c,
            phasor: psr,
        }
    }
}

impl<'a> Iterator for DigitalModulator<'a> {
    type Item = Complex32;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        let bits = match self.data.next() {
            SourceUpdate::Finished => return None,
            SourceUpdate::Changed(b) => {
                self.phasor.update(self.carrier.sample, b);
                b
            }
            SourceUpdate::Unchanged(b) => b,
        };

        let (i, q) = self.phasor.next(self.carrier.sample, bits);
        let (sin, cos) = phase.sin_cos();

        Some(Complex32::new(real(i, q, cos, sin), imag(i, q, cos, sin)))
    }
}
