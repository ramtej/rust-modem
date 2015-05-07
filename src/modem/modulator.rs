use super::{phasor, carrier, integrator};

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
}

impl<'a> Iterator for Encoder<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
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

        Some(self.phasor.i(s, bits) * self.carrier.inner(s).cos() -
             self.phasor.q(s, bits) * self.carrier.inner(s).sin())
    }
}

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

pub struct FrequencyModulator<'a, 'b> {
    carrier: &'b carrier::Carrier,
    int: &'a mut integrator::Integrator<'a>,
    amplitude: f64,
    deviation: f64,
    sample: usize,
}

impl<'a, 'b> FrequencyModulator<'a, 'b> {
    pub fn new(carrier: &'b carrier::Carrier,
               int: &'a mut integrator::Integrator<'a>,
               amplitude: f64, deviation: f64)
        -> FrequencyModulator<'a, 'b>
    {
        FrequencyModulator {
            carrier: carrier,
            int: int,
            amplitude: amplitude,
            deviation: deviation,
            sample: 0,
        }
    }
}

impl<'a, 'b> Iterator for FrequencyModulator<'a, 'b> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        let next = match self.int.next() {
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
