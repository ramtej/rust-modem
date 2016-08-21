pub trait Phasor {
    fn next(&mut self, s: usize) -> Option<(f32, f32)>;
}

pub struct Raw {
    amplitude: f32,
}

impl Raw {
    pub fn new(amplitude: f32) -> Raw {
        Raw {
            amplitude: amplitude,
        }
    }

    fn i(&self) -> f32 { self.amplitude }
    fn q(&self) -> f32 { 0.0 }
}

impl Phasor for Raw {
    fn next(&mut self, _: usize) -> Option<(f32, f32)> {
        Some((self.i(), self.q()))
    }
}
