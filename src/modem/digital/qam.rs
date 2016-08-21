use super::DigitalPhasor;
use super::util::{bytes_to_bits, max_symbol};

pub struct QAM {
    bits_per_symbol: usize,
    // Number of bits per carrier.
    carrier_size: usize,
    max_symbol: f32,
    phase_cos: f32,
    phase_sin: f32,
    amplitude: f32,
}

impl QAM {
    pub fn new(bits_per_symbol: usize, phase: f32, amplitude: f32) -> QAM {
        // Must have a bit for i and a bit for q.
        assert!(bits_per_symbol > 1);

        let cs = bits_per_symbol / 2;
        let ms = max_symbol(cs) as f32;

        QAM {
            bits_per_symbol: bits_per_symbol,
            carrier_size: cs,
            max_symbol: ms,
            phase_cos: phase.cos(),
            phase_sin: phase.sin(),
            amplitude: amplitude / ms / 2.0,
        }
    }

    fn pos_symbol(&self, s: u8) -> f32 {
        2.0 * s as f32 - self.max_symbol
    }

    fn pos_bytes(&self, b: &[u8]) -> f32 {
        self.pos_symbol(bytes_to_bits(b))
    }
}

impl DigitalPhasor for QAM {
    fn bits_per_symbol(&self) -> usize { self.bits_per_symbol }

    fn i(&self, _: usize, b: &[u8]) -> f32 {
        let (msb, lsb) = b.split_at(self.carrier_size);

        self.amplitude * (
            self.pos_bytes(msb) * self.phase_cos -
            self.pos_bytes(lsb) * self.phase_sin
        )
    }

    fn q(&self, _: usize, b: &[u8]) -> f32 {
        let (msb, lsb) = b.split_at(self.carrier_size);

        self.amplitude * (
            self.pos_bytes(lsb) * self.phase_cos +
            self.pos_bytes(msb) * self.phase_sin
        )
    }
}

#[cfg(test)]
mod test {
    use super::QAM;
    use digital::DigitalPhasor;

    #[test]
    fn test_qam() {

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
}
