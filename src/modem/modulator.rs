extern crate num;

use super::{phasor, digital, carrier};
use std;

#[derive(Copy, Clone)]
pub struct Params {
    // Symbols per second.
    pub baud_rate: u32,
    // Samples per second.
    pub sample_rate: u32,
    // Samples per bit.
    pub samples_per_bit: u32,
}

impl Params {
    pub fn new(br: u32, sr: u32) -> Params {
        Params {
            baud_rate: br,
            sample_rate: sr,
            samples_per_bit: sr / br,
        }
    }
}

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

impl<'a> Modulator {
    pub fn new(c: carrier::Carrier, psr: Box<phasor::Phasor>) -> Modulator {
        Modulator {
            carrier: c,
            phasor: psr,
        }
    }
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

pub struct DigitalModulator<'a> {
    params: Params,

    carrier: carrier::Carrier,
    phasor: Box<digital::DigitalPhasor>,

    bits: &'a [u8],
    samples: usize,

    prev_idx: usize,
}

impl<'a> DigitalModulator<'a> {
    pub fn new(p: Params, c: carrier::Carrier, psr: Box<digital::DigitalPhasor>,
               b: &'a [u8])
        -> DigitalModulator<'a>
    {
        let samples = p.samples_per_bit as usize * b.len() /
            psr.group_size() as usize;

        DigitalModulator {
            params: p,
            carrier: c,
            phasor: psr,
            bits: b,
            samples: samples,
            prev_idx: std::usize::MAX,
        }
    }
}

impl<'a> Iterator for DigitalModulator<'a> {
    type Item = num::Complex<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        if self.carrier.sample >= self.samples {
            return None;
        }

        let idx = self.carrier.sample / self.params.samples_per_bit as usize *
            self.phasor.group_size() as usize;
        let bits = &self.bits[idx..idx + self.phasor.group_size() as usize];

        if idx != self.prev_idx {
            self.phasor.update(self.carrier.sample, bits);
            self.prev_idx = idx;
        }

        let (i, q) = match self.phasor.next(self.carrier.sample, bits) {
            Some((i, q)) => (i, q),
            None => return None,
        };

        let cos = phase.cos();
        let sin = phase.sin();

        Some(num::Complex::new(real(i, q, cos, sin), imag(i, q, cos, sin)))
    }
}
