use freq::Freq;
use rates::Rates;

use super::DigitalPhasor;
use super::util::bytes_to_bits;

pub struct CPFSK {
    bits_per_symbol: usize,
    freq: f32,
    amplitude: f32,
}

impl CPFSK {
    pub fn new(bits_per_symbol: usize, rates: Rates, amplitude: f32, deviation: usize)
        -> CPFSK
    {
        CPFSK {
            bits_per_symbol: bits_per_symbol,
            freq: Freq::new(deviation * rates.baud_rate / 2,
                            rates.sample_rate).sample_freq(),
            amplitude: amplitude,
        }
    }

    fn coef(&self, symbol: u8) -> f32 {
        2.0 * symbol as f32
    }

    fn inner(&self, b: &[u8], s: usize) -> f32 {
        self.coef(bytes_to_bits(b)) * self.freq * s as f32
    }
}

impl DigitalPhasor for CPFSK {
    fn bits_per_symbol(&self) -> usize { self.bits_per_symbol }

    fn i(&self, s: usize, b: &[u8]) -> f32 {
        self.amplitude * self.inner(b, s).cos()
    }

    fn q(&self, s: usize, b: &[u8]) -> f32 {
        self.amplitude * self.inner(b, s).sin()
    }
}
