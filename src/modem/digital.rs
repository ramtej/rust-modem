use super::{util, freq};
use std;

pub trait DigitalPhasor {
    fn group_size(&self) -> u32;

    fn update(&mut self, _s: usize, _b: &[u8]) {}

    fn i(&self, s: usize, b: &[u8]) -> f64;
    fn q(&self, s: usize, b: &[u8]) -> f64;

    fn next(&self, s: usize, b: &[u8]) -> (f64, f64) {
        (self.i(s, b), self.q(s, b))
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

fn max_symbol(group_size: u32) -> u32 {
    (1 << group_size) - 1
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
            amplitude: amplitude * 0.5f64.sqrt(),
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

pub struct QAM {
    group_size: u32,
    // Number of bits per carrier.
    carrier_size: u32,
    max_symbol: f64,
    phase_cos: f64,
    phase_sin: f64,
    amplitude: f64,
}

impl QAM {
    pub fn new(group_size: u32, phase: f64, amplitude: f64) -> QAM {
        // Must have a bit for i and a bit for q.
        assert!(group_size > 1);

        let cs = group_size / 2;
        let ms = max_symbol(cs) as f64;

        QAM {
            group_size: group_size,
            carrier_size: cs,
            max_symbol: ms,
            phase_cos: phase.cos(),
            phase_sin: phase.sin(),
            amplitude: amplitude / ms / 2.0,
        }
    }

    fn pos_symbol(&self, s: u8) -> f64 {
        2.0 * s as f64 - self.max_symbol
    }

    fn pos_bytes(&self, b: &[u8]) -> f64 {
        self.pos_symbol(bytes_to_bits(b))
    }
}

impl DigitalPhasor for QAM {
    fn group_size(&self) -> u32 { self.group_size }

    fn i(&self, _: usize, b: &[u8]) -> f64 {
        let (msb, lsb) = b.split_at(self.carrier_size as usize);

        self.amplitude * (
            self.pos_bytes(msb) * self.phase_cos -
            self.pos_bytes(lsb) * self.phase_sin
        )
    }

    fn q(&self, _: usize, b: &[u8]) -> f64 {
        let (msb, lsb) = b.split_at(self.carrier_size as usize);

        self.amplitude * (
            self.pos_bytes(lsb) * self.phase_cos +
            self.pos_bytes(msb) * self.phase_sin
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

pub trait SymbolMap {
    fn coef(&self, symbol: u8) -> f64;
}

pub struct DefaultMap {
    max_symbol: u32,
}

impl DefaultMap {
    pub fn new(group_size: u32) -> DefaultMap {
        DefaultMap {
            max_symbol: (1 << group_size) - 1,
        }
    }
}

impl SymbolMap for DefaultMap {
    fn coef(&self, symbol: u8) -> f64 {
        (2 * symbol as i32 - self.max_symbol as i32) as f64
    }
}

pub struct IncreaseMap;

impl SymbolMap for IncreaseMap {
    fn coef(&self, symbol: u8) -> f64 {
        (2 * symbol) as f64
    }
}

pub struct MFSK<M: SymbolMap> {
    group_size: u32,
    deviation: f64,
    amplitude: f64,
    map: M,
    phase_offset: f64,
    cur_coef: f64,
}

impl<M: SymbolMap> MFSK<M> {
    pub fn new(group_size: u32, deviation: freq::Freq, amplitude: f64, map: M)
        -> MFSK<M>
    {
        MFSK {
            group_size: group_size,
            deviation: deviation.sample_freq(),
            amplitude: amplitude,
            map: map,
            phase_offset: 0.0,
            cur_coef: 0.0,
        }
    }

    fn inner(&self, s: usize) -> f64 {
        self.cur_coef * self.deviation * s as f64 + self.phase_offset
    }
}

impl<M: SymbolMap> DigitalPhasor for MFSK<M> {
    fn group_size(&self) -> u32 { self.group_size }

    fn update(&mut self, s: usize, b: &[u8]) {
        let next_coef = self.map.coef(bytes_to_bits(b));

        self.phase_offset += (self.cur_coef - next_coef) * self.deviation * s as f64;
        self.phase_offset = util::mod_trig(self.phase_offset);

        self.cur_coef = next_coef;
    }

    fn i(&self, s: usize, _: &[u8]) -> f64 {
        self.amplitude * self.inner(s).cos()
    }

    fn q(&self, s: usize, _: &[u8]) -> f64 {
        self.amplitude * self.inner(s).sin()
    }
}

pub struct MPSK {
    group_size: u32,
    num_symbols: f64,
    amplitude: f64,
    phase_offset: f64,
}

impl MPSK {
    pub fn new(group_size: u32, phase_offset: f64, amplitude: f64) -> MPSK {
        MPSK {
            group_size: group_size,
            num_symbols: (1 << group_size) as f64,
            amplitude: amplitude,
            phase_offset: phase_offset,
        }
    }

    fn inner(&self, b: &[u8]) -> f64 {
        self.phase(b) + self.phase_offset
    }

    fn phase(&self, b: &[u8]) -> f64 {
        2.0 * std::f64::consts::PI * bytes_to_bits(b) as f64 / self.num_symbols
    }
}

impl DigitalPhasor for MPSK {
    fn group_size(&self) -> u32 { self.group_size }

    fn i(&self, _: usize, b: &[u8]) -> f64 {
        self.amplitude * self.inner(b).cos()
    }

    fn q(&self, _: usize, b: &[u8]) -> f64 {
        self.amplitude * self.inner(b).sin()
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_b2b() {
        assert_eq!(super::bytes_to_bits(&[0, 0, 0, 1]), 0b0001);
        assert_eq!(super::bytes_to_bits(&[0, 1, 0, 1]), 0b0101);
    }

    #[test]
    fn test_max_symbol() {
        use super::max_symbol;
        assert_eq!(max_symbol(1), 0b1);
        assert_eq!(max_symbol(2), 0b11);
        assert_eq!(max_symbol(4), 0b1111);
        assert_eq!(max_symbol(8), 0b11111111);
    }

    #[test]
    fn test_qam() {
        use super::{QAM, DigitalPhasor};

        let qam = QAM::new(4, 0.0, 6.0);

        assert_eq!(qam.i(0, &[0,0,0,0]), -3.0);
        assert_eq!(qam.q(0, &[0,0,0,0]), -3.0);

        assert_eq!(qam.i(0, &[0,0,0,1]), -3.0);
        assert_eq!(qam.q(0, &[0,0,0,1]), -1.0);

        assert_eq!(qam.i(0, &[1,0,1,1]), 1.0);
        assert_eq!(qam.q(0, &[1,0,1,1]), 3.0);

        assert_eq!(qam.i(0, &[1,1,1,1]), 3.0);
        assert_eq!(qam.q(0, &[1,1,1,1]), 3.0);
    }

    #[test]
    fn test_mpsk() {
        use super::{MPSK, DigitalPhasor};
        use std::f64::consts::PI;

        let mpsk = MPSK::new(2, 0.0, 1.0);
        assert_eq!(mpsk.i(0, &[0, 0]), 1.0);
        assert_eq!(mpsk.q(0, &[0, 0]), 0.0);

        assert!(mpsk.i(0, &[0, 1]).abs() < 0.001);
        assert_eq!(mpsk.q(0, &[0, 1]), 1.0);

        assert_eq!(mpsk.i(0, &[1, 0]), -1.0);
        assert!(mpsk.q(0, &[1, 0]).abs() < 0.001);

        assert!(mpsk.i(0, &[1, 1]).abs() < 0.001);
        assert_eq!(mpsk.q(0, &[1, 1]), -1.0);
    }
}
