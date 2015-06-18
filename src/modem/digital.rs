use super::{util, freq};
use std;

pub trait DigitalPhasor {
    fn group_size(&self) -> u32;

    fn update(&mut self, _s: usize, _b: &[u8]) {}

    fn i(&self, s: usize, b: &[u8]) -> f64;
    fn q(&self, s: usize, b: &[u8]) -> f64;

    fn next(&self, s: usize, b: &[u8]) -> Option<(f64, f64)> {
        Some((self.i(s, b), self.q(s, b)))
    }
}

fn bit_to_sign(b: u8) -> f64 {
    (2 * b as i8 - 1) as f64
}

fn bytes_to_bits(bytes: &[u8]) -> u8 {
    let len = bytes.len() - 1;

    bytes.iter().enumerate().fold(0, |s, (i, &b)| {
        s | (b & 1) << (len - i)
    })
}

pub struct BPSK {
    phase: f64,
    amplitude: f64,
}

impl BPSK {
    pub fn new(phase: f64, amplitude: f64) -> BPSK {
        BPSK {
            phase: phase,
            amplitude: amplitude,
        }
    }

    fn common(&self, b: u8) -> f64 {
        bit_to_sign(b) * self.amplitude
    }
}

impl DigitalPhasor for BPSK {
    fn group_size(&self) -> u32 { 1 }

    fn i(&self, _: usize, b: &[u8]) -> f64 {
        self.common(b[0]) * self.phase.cos()
    }

    fn q(&self, _: usize, b: &[u8]) -> f64 {
        self.common(b[0]) * self.phase.sin()
    }
}

pub struct BASK {
    amplitude: f64,
}

impl BASK {
    pub fn new(a: f64) -> BASK {
        BASK {
            amplitude: a,
        }
    }
}

impl DigitalPhasor for BASK {
    fn group_size(&self) -> u32 { 1 }

    fn i(&self, _: usize, b: &[u8]) -> f64 {
        b[0] as f64 * self.amplitude
    }

    fn q(&self, _: usize, _: &[u8]) -> f64 {
        0.0
    }
}

pub struct BFSK {
    deviation: f64,
    amplitude: f64,
    phase: f64,
    prev: u8,
}

impl BFSK {
    pub fn new(d: freq::Freq, a: f64) -> BFSK {
        BFSK {
            deviation: d.sample_freq(),
            amplitude: a,
            phase: 0.0,
            prev: 0,
        }
    }

    fn inner(&self, s: usize, b: u8) -> f64 {
        self.rads(s, b) + self.phase
    }

    fn rads(&self, s: usize, b: u8) -> f64 {
        b as f64 * self.deviation * s as f64
    }
}

impl DigitalPhasor for BFSK {
    fn group_size(&self) -> u32 { 1 }

    fn i(&self, s: usize, b: &[u8]) -> f64 {
        self.amplitude * self.inner(s, b[0]).cos()
    }

    fn q(&self, s: usize, b: &[u8]) -> f64 {
        self.amplitude * self.inner(s, b[0]).sin()
    }

    fn update(&mut self, s: usize, b: &[u8]) {
        if b[0] == self.prev {
            return;
        }

        self.phase = util::mod_trig(self.phase + if b[0] == 1 {
            -self.rads(s, 1)
        } else {
            self.rads(s - 1, 1)
        });

        self.prev = b[0];
    }
}

pub struct QPSK {
    phase_cos: f64,
    phase_sin: f64,
    amplitude: f64,
}

impl QPSK {
    pub fn new(phase: f64, amplitude: f64) -> QPSK {
        QPSK {
            phase_cos: phase.cos(),
            phase_sin: phase.sin(),
            amplitude: amplitude / 2.0,
        }
    }
}

impl DigitalPhasor for QPSK {
    fn group_size(&self) -> u32 { 2 }

    fn i(&self, _: usize, b: &[u8]) -> f64 {
        self.amplitude * (
            bit_to_sign(b[0]) as f64 * self.phase_cos -
            bit_to_sign(b[1]) as f64 * self.phase_sin
        )
    }

    fn q(&self, _: usize, b: &[u8]) -> f64 {
        self.amplitude * (
            bit_to_sign(b[1]) as f64 * self.phase_cos +
            bit_to_sign(b[0]) as f64 * self.phase_sin
        )
    }
}

pub struct QAM16 {
    phase_cos: f64,
    phase_sin: f64,
    amplitude: f64,
}

impl QAM16 {
    pub fn new(phase: f64, amplitude: f64) -> QAM16 {
        QAM16 {
            phase_cos: phase.cos(),
            phase_sin: phase.sin(),
            amplitude: amplitude / 15.0 / 2.0,
        }
    }

    fn symbol(b: &[u8]) -> i32 {
        2 * bytes_to_bits(b) as i32 - 15
    }
}

impl DigitalPhasor for QAM16 {
    fn group_size(&self) -> u32 { 8 }

    fn i(&self, _: usize, b: &[u8]) -> f64 {
        self.amplitude * (
            QAM16::symbol(&b[..4]) as f64 * self.phase_cos -
            QAM16::symbol(&b[4..]) as f64 * self.phase_sin
        )
    }

    fn q(&self, _: usize, b: &[u8]) -> f64 {
        self.amplitude * (
            QAM16::symbol(&b[4..]) as f64 * self.phase_cos +
            QAM16::symbol(&b[..4]) as f64 * self.phase_sin
        )
    }
}

pub struct MSK {
    amplitude: f64,
    samples_per_bit: f64,
    bit: usize,
    bits: [u8; 2],
}

impl MSK {
    pub fn new(amplitude: f64, samples_per_bit: u32) -> MSK {
        MSK {
            amplitude: amplitude,
            samples_per_bit: samples_per_bit as f64,
            bit: 1,
            bits: [0, 0],
        }
    }

    fn b(&self) -> f64 {
        if self.bits[0] == self.bits[1] { 1.0 } else { -1.0 }
    }

    fn phi(&self) -> f64 {
        if self.bits[0] == 1 { 0.0 } else { std::f64::consts::PI }
    }

    fn inner(&self, s: usize) -> f64 {
        self.b() * std::f64::consts::FRAC_PI_2 * s as f64 /
            self.samples_per_bit + self.phi()
    }
}

impl DigitalPhasor for MSK {
    fn group_size(&self) -> u32 { 1 }

    fn i(&self, s: usize, _: &[u8]) -> f64 {
        self.amplitude * self.inner(s).cos()
    }

    fn q(&self, s: usize, _: &[u8]) -> f64 {
        self.amplitude * self.inner(s).sin()
    }

    fn update(&mut self, _: usize, b: &[u8]) {
        self.bits[self.bit] = b[0];

        self.bit += 1;
        self.bit %= 2;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_b2b() {
        assert_eq!(super::bytes_to_bits(&[0, 0, 0, 1]), 0b0001);
        assert_eq!(super::bytes_to_bits(&[0, 1, 0, 1]), 0b0101);
    }
}
