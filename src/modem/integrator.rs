pub struct Integrator<T: Iterator<Item = f64>> {
    sig: T,
    prev: f64,
    accum: f64,
}

impl<T: Iterator<Item = f64>> Integrator<T> {
    pub fn new(sig: T) -> Integrator<T> {
        // Why?
        let mut sig = sig;
        let x = sig.next().unwrap().acos();

        Integrator {
            sig: sig,
            prev: x,
            accum: 0.0,
        }
    }
}

impl<T: Iterator<Item = f64>> Iterator for Integrator<T> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        let next = match self.sig.next() {
            None => return None,
            Some(s) => s,
        };

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
