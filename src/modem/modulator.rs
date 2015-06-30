extern crate num;

use super::{phasor, digital, carrier};
use std;

#[derive(Copy, Clone)]
pub struct Params {
    // Symbols per second.
    pub baud_rate: u32,
    // Samples per second.
    pub sample_rate: u32,
    // Samples per symbol.
    pub samples_per_symbol: u32,
}

impl Params {
    pub fn new(br: u32, sr: u32) -> Params {
        Params {
            baud_rate: br,
            sample_rate: sr,
            samples_per_symbol: sr / br,
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

pub struct DigitalModulator<'a> {
    params: Params,

    carrier: carrier::Carrier,
    phasor: Box<digital::DigitalPhasor>,

    bits: &'a [u8],
    start_sample: usize,

    cur_symbol: usize,
}

impl<'a> DigitalModulator<'a> {
    pub fn new(p: Params, c: carrier::Carrier, psr: Box<digital::DigitalPhasor>,
               b: &'a [u8])
        -> DigitalModulator<'a>
    {
        let start_sample = c.sample + 1;

        DigitalModulator {
            params: p,
            carrier: c,
            phasor: psr,
            bits: b,
            start_sample: start_sample,
            cur_symbol: std::usize::MAX,
        }
    }

    pub fn into_carrier(self) -> carrier::Carrier { self.carrier }
}

impl<'a> Iterator for DigitalModulator<'a> {
    type Item = num::Complex<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.carrier.next();

        let symbol = (self.carrier.sample - self.start_sample) /
            self.params.samples_per_symbol as usize *
            self.phasor.bits_per_symbol() as usize;

        let end = symbol + self.phasor.bits_per_symbol() as usize;

        if end > self.bits.len() {
            return None;
        }

        let bits = &self.bits[symbol..end];

        if symbol != self.cur_symbol {
            self.phasor.update(self.carrier.sample, bits);
            self.cur_symbol = symbol;
        }

        let (i, q) = self.phasor.next(self.carrier.sample, bits);

        let cos = phase.cos();
        let sin = phase.sin();

        Some(num::Complex::new(real(i, q, cos, sin), imag(i, q, cos, sin)))
    }
}
