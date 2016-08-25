/// Implements Dual-Constellation QPSK (aka π/4-QPSK), which rotates the QPSK
/// constellation by π/4 between each symbol. As a result, the maximum phase shift is
/// 3π/4, so the modulated signal never passes through the origin.

use std::f32::consts::PI;

use super::DigitalPhasor;
use super::util::bytes_to_bits;

pub struct DCQPSK {
    amplitude: f32,
    even: bool,
}

impl DCQPSK {
    pub fn new(amplitude: f32) -> DCQPSK {
        DCQPSK {
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

impl DigitalPhasor for DCQPSK {
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
