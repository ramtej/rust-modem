pub struct Integrator<T: Iterator<Item = f32>> {
    sig: T,
    amplitude: f32,
    prev: f32,
    accum: f32,
}

impl<T: Iterator<Item = f32>> Integrator<T> {
    pub fn new(mut sig: T, amplitude: f32) -> Integrator<T> {
        let x = (sig.next().unwrap() / amplitude).acos();

        Integrator {
            sig: sig,
            amplitude: amplitude,
            prev: x,
            accum: 0.0,
        }
    }
}

impl<T: Iterator<Item = f32>> Iterator for Integrator<T> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let next = match self.sig.next() {
            None => return None,
            Some(s) => s,
        } / self.amplitude;

        // Round to 5 decimal places.
        let next = (next * 10000.0).trunc() / 10000.0;
        assert!(next >= -1.0 && next <= 1.0);

        let x = next.acos();
        let diff = (x - self.prev).abs();

        self.accum += next * diff;
        self.prev = x;

        Some(self.accum)
    }
}
