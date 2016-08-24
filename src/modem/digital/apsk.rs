/// Implements Amplitude and Phase Shift Keying (APSK) digital modulation. Symbols are
/// grouped into concentric rings, where each ring is offset from the origin by a radius,
/// The symbols in each ring are offset from each other by a constant phase angle.

use std::f32::consts::PI;
use std::ops::Range;

use super::DigitalPhasor;
use super::util::{bytes_to_bits, max_symbol};

/// Provides the APSK phasor.
pub struct APSK {
    /// Maximum amplitude.
    amplitude: f32,
    /// Determines total number of symbols over all rings.
    bits_per_symbol: usize,
    /// Ring specifications.
    rings: Vec<Ring>,
}

impl APSK {
    /// Create a new APSK phasor with the given maximum amplitude, symbol size, and ring
    /// specifications. The rings must be given in the order of the symbols they cover and
    /// in total must cover all possible symbol values for the given symbol size.
    pub fn new(amplitude: f32, bits_per_symbol: usize, rings: Vec<Ring>) -> APSK {
        assert!(verify(&rings[..], bits_per_symbol));

        APSK {
            amplitude: amplitude,
            bits_per_symbol: bits_per_symbol,
            rings: rings,
        }
    }

    /// Compute the radius and phase for the given symbol.
    fn common(&self, symbol: u8) -> (f32, f32) {
        let ring = self.rings.iter().find(|r| r.range.contains(symbol)).unwrap();
        let phase = 2.0 * PI * (symbol - ring.range.start) as f32 /
            (ring.range.end - ring.range.start) as f32 + ring.phase;

        (ring.radius, phase)
    }
}

impl DigitalPhasor for APSK {
    fn bits_per_symbol(&self) -> usize { self.bits_per_symbol }

    fn i(&self, _: usize, b: &[u8]) -> f32 {
        let (r, inner) = self.common(bytes_to_bits(b));
        self.amplitude * r * inner.cos()
    }

    fn q(&self, _: usize, b: &[u8]) -> f32 {
        let (r, inner) = self.common(bytes_to_bits(b));
        self.amplitude * r * inner.sin()
    }
}

/// Single ring for symbols.
pub struct Ring {
    /// Symbols contained in this ring.
    range: Range<u8>,
    /// Radius of ring.
    radius: f32,
    /// Phase offset between symbols.
    phase: f32,
}

impl Ring {
    /// Create a new ring to cover the given range of symbols at the given radius (as a
    /// fraction between 0 and 1 of the maximum amplitude) and with the given phase offset
    /// (radians) between symbols.
    pub fn new(range: Range<u8>, radius: f32, phase: f32) -> Ring {
        assert!(radius >= 0.0 && radius <= 1.0);

        Ring {
            range: range,
            radius: radius,
            phase: phase,
        }
    }
}

/// Verify that the given rings are in order and cover all possible symbol values.
fn verify(rings: &[Ring], bits_per_symbol: usize) -> bool {
    let mut prev = 0;

    for ring in rings {
        if ring.range.start != prev {
            return false;
        }

        prev = ring.range.end;
    }

    prev as usize == max_symbol(bits_per_symbol) + 1
}
