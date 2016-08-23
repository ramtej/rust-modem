use num::complex::Complex32;

use data::{Source, SourceUpdate};
use phasor::Phasor;
use carrier::Carrier;
use digital::DigitalPhasor;

pub struct Modulator<'a> {
    carrier: &'a mut Carrier,
    phasor: Box<Phasor>,
}

impl<'a> Modulator<'a> {
    pub fn new(c: &'a mut Carrier, phasor: Box<Phasor>) -> Modulator<'a> {
        Modulator {
            carrier: c,
            phasor: phasor,
        }
    }
}

pub struct IQSample {
    carrier: f32,
    pub i: f32,
    pub q: f32,
}

impl IQSample {
    fn new(carrier: f32, i: f32, q: f32) -> IQSample {
        IQSample {
            carrier: carrier,
            i: i,
            q: q,
        }
    }

    fn real(&self, cos: f32, sin: f32) -> f32 {
        self.i * cos - self.q * sin
    }

    fn imag(&self, cos: f32, sin: f32) -> f32 {
        self.i * sin + self.q * cos
    }

    pub fn modulate(&self) -> Complex32 {
        let (sin, cos) = self.carrier.sin_cos();
        Complex32::new(self.real(cos, sin), self.imag(cos, sin))
    }
}

impl<'a> Iterator for Modulator<'a> {
    type Item = IQSample;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        match self.phasor.next(self.carrier.sample) {
            Some((i, q)) => Some(IQSample::new(phase, i, q)),
            None => None,
        }
    }
}

pub struct DigitalModulator<'a> {
    data: Box<Source>,
    carrier: &'a mut Carrier,
    phasor: Box<DigitalPhasor>,
}

impl<'a> DigitalModulator<'a> {
    pub fn new(c: &'a mut Carrier, phasor: Box<DigitalPhasor>, src: Box<Source>)
        -> DigitalModulator<'a>
    {
        DigitalModulator {
            data: src,
            carrier: c,
            phasor: phasor,
        }
    }
}

impl<'a> Iterator for DigitalModulator<'a> {
    type Item = IQSample;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        let bits = match self.data.next() {
            SourceUpdate::Finished => return None,
            SourceUpdate::Changed(b) => {
                self.phasor.update(self.carrier.sample, b);
                b
            },
            SourceUpdate::Unchanged(b) => b,
        };

        let (i, q) = self.phasor.next(self.carrier.sample, bits);

        Some(IQSample::new(phase, i, q))
    }
}
