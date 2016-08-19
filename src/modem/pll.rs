extern crate num;

const CHANGE: f32 = 0.447214;

pub struct PLL {
    pub phase_offset: f32,
}

impl PLL {
    pub fn new() -> PLL {
        PLL {
            phase_offset: 0.0,
        }
    }

    pub fn handle(&mut self, carrier_phase: f32, x: num::Complex<f32>) {
        let inner = carrier_phase + self.phase_offset;
        let carrier = num::Complex::new(inner.cos(), inner.sin());
        let err = (x * carrier.conj()).arg();

        self.phase_offset += CHANGE * err;
    }
}
