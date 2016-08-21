pub trait DigitalPhasor {
    fn bits_per_symbol(&self) -> usize;

    fn update(&mut self, _s: usize, _b: &[u8]) {}

    fn i(&self, s: usize, b: &[u8]) -> f32;
    fn q(&self, s: usize, b: &[u8]) -> f32;

    fn next(&self, s: usize, b: &[u8]) -> (f32, f32) {
        (self.i(s, b), self.q(s, b))
    }
}
