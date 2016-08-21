use std::f32::consts::PI;

use super::DigitalPhasor;
use super::util::bytes_to_bits;

pub struct DQPSK {
    amplitude: f32,
    even: bool,
}

impl DQPSK {
    pub fn new(amplitude: f32) -> DQPSK {
        DQPSK {
            amplitude: amplitude,
            even: false,
        }
    }

    fn term(&self, symbol: u8) -> f32 {
        const MAP: [f32; 4] = [
            0.0,
            PI / 2.0,
            3.0 * PI / 2.0,
            PI,
        ];

        if self.even {
            MAP[symbol as usize] + PI / 4.0
        } else {
            MAP[symbol as usize]
        }
    }
}

impl DigitalPhasor for DQPSK {
    fn bits_per_symbol(&self) -> usize { 2 }

    fn update(&mut self, _: usize, _: &[u8]) {
        self.even = !self.even;
    }

    fn i(&self, _: usize, b: &[u8]) -> f32 {
        self.amplitude * self.term(bytes_to_bits(b)).cos()
    }

    fn q(&self, _: usize, b: &[u8]) -> f32 {
        self.amplitude * self.term(bytes_to_bits(b)).sin()
    }
}
