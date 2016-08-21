use freq::Freq;
use util::mod_trig;

use super::DigitalPhasor;
use super::util::{bytes_to_bits, max_symbol};

pub trait SymbolMap {
    fn coef(&self, symbol: u8) -> f32;
}

pub struct DefaultMap {
    max_symbol: i32,
}

impl DefaultMap {
    pub fn new(bits_per_symbol: usize) -> DefaultMap {
        DefaultMap {
            max_symbol: max_symbol(bits_per_symbol) as i32,
        }
    }
}

impl SymbolMap for DefaultMap {
    fn coef(&self, symbol: u8) -> f32 {
        (2 * symbol as i32 - self.max_symbol) as f32
    }
}

pub struct IncreaseMap;

impl SymbolMap for IncreaseMap {
    fn coef(&self, symbol: u8) -> f32 {
        (2 * symbol) as f32
    }
}

pub struct MFSK<M: SymbolMap> {
    bits_per_symbol: usize,
    deviation: f32,
    amplitude: f32,
    map: M,
    phase_offset: f32,
    cur_coef: f32,
}

impl<M: SymbolMap> MFSK<M> {
    pub fn new(bits_per_symbol: usize, deviation: Freq, amplitude: f32, map: M)
        -> MFSK<M>
    {
        MFSK {
            bits_per_symbol: bits_per_symbol,
            deviation: deviation.sample_freq(),
            amplitude: amplitude,
            map: map,
            phase_offset: 0.0,
            cur_coef: 0.0,
        }
    }

    fn inner(&self, s: usize) -> f32 {
        self.cur_coef * self.deviation * s as f32 + self.phase_offset
    }
}

impl<M: SymbolMap> DigitalPhasor for MFSK<M> {
    fn bits_per_symbol(&self) -> usize { self.bits_per_symbol }

    fn update(&mut self, s: usize, b: &[u8]) {
        let next_coef = self.map.coef(bytes_to_bits(b));

        self.phase_offset += (self.cur_coef - next_coef) * self.deviation * s as f32;
        self.phase_offset = mod_trig(self.phase_offset);

        self.cur_coef = next_coef;
    }

    fn i(&self, s: usize, _: &[u8]) -> f32 {
        self.amplitude * self.inner(s).cos()
    }

    fn q(&self, s: usize, _: &[u8]) -> f32 {
        self.amplitude * self.inner(s).sin()
    }
}
