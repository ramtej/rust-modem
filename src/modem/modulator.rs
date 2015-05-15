extern crate num;

use super::{phasor, carrier, integrator, freq};

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

pub struct Encoder<'a> {
    params: Params,

    carrier: &'a carrier::Carrier,
    phasor: Box<phasor::Phasor>,

    bits: &'a [u8],
    idx: usize,

    sample: usize,
    samples: usize,
}

impl<'a> Encoder<'a> {
    pub fn new(p: Params, c: &'a carrier::Carrier, psr: Box<phasor::Phasor>,
               b: &'a [u8]) -> Encoder<'a>
    {
        let samples = p.samples_per_bit as usize * b.len() /
            psr.group_size() as usize;

        Encoder {
            params: p,

            carrier: c,
            phasor: psr,

            bits: b,
            idx: 0,

            sample: 0,
            samples: samples,
        }
    }

    fn real(&self, i: f64, q: f64, cos: f64, sin: f64) -> f64 {
        i * cos - q * sin
    }

    fn imag(&self, i: f64, q: f64, cos: f64, sin: f64) -> f64 {
        i * sin + q * cos
    }
}

impl<'a> Iterator for Encoder<'a> {
    type Item = num::Complex<f64>;

    fn next(&mut self) -> Option<num::Complex<f64>> {
        if self.sample >= self.samples { return None; }

        let idx = self.sample / self.params.samples_per_bit as usize *
            self.phasor.group_size() as usize;
        let bits = &self.bits[idx..];

        if idx != self.idx {
            self.phasor.update(self.sample, bits);
            self.idx = idx;
        }

        let s = self.sample;
        self.sample += 1 ;

        let i = self.phasor.i(s, bits);
        let q = self.phasor.q(s, bits);

        let phase = self.carrier.inner(s);
        let cos = phase.cos();
        let sin = phase.sin();

        Some(num::Complex::new(self.real(i, q, cos, sin),
                               self.imag(i, q, cos, sin)))
    }
}

pub struct FrequencyModulator<'a, 'b> {
    carrier: &'b carrier::Carrier,
    integ: &'a mut integrator::Integrator<'a>,
    amplitude: f64,
    deviation: f64,
    sample: usize,
}

impl<'a, 'b> FrequencyModulator<'a, 'b> {
    pub fn new(carrier: &'b carrier::Carrier,
               integ: &'a mut integrator::Integrator<'a>,
               amplitude: f64, deviation: freq::Freq)
        -> FrequencyModulator<'a, 'b>
    {
        FrequencyModulator {
            carrier: carrier,
            integ: integ,
            amplitude: amplitude,
            deviation: deviation.sample_freq(),
            sample: 0,
        }
    }
}

impl<'a, 'b> Iterator for FrequencyModulator<'a, 'b> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        let next = match self.integ.next() {
            None => return None,
            Some(s) => s,
        };

        let sample = self.sample;
        self.sample += 1;

        Some(self.amplitude * (
            self.carrier.inner(sample) + self.deviation * next
        ).cos())
    }
}

pub struct AmplitudeModulator<'a> {
    carrier: &'a carrier::Carrier,
    sig: &'a mut Iterator<Item = f64>,
    amplitude: f64,
    sample: usize,
}

impl<'a> AmplitudeModulator<'a> {
    pub fn new(carrier: &'a carrier::Carrier, sig: &'a mut Iterator<Item = f64>,
           amplitude: f64)
        -> AmplitudeModulator<'a>
    {
        AmplitudeModulator {
            carrier: carrier,
            sig: sig,
            amplitude: amplitude,
            sample: 0,
        }
    }
}

impl<'a> Iterator for AmplitudeModulator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        let s = match self.sig.next() {
            None => return None,
            Some(s) => s,
        };

        let sample = self.sample;
        self.sample += 1;

        Some(self.amplitude * s * self.carrier.inner(sample).cos())
    }
}
