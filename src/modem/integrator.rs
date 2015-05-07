pub struct Integrator<'a> {
    sig: &'a mut Iterator<Item = f64>,
    prev: f64,
    accum: f64,
}

impl<'a> Integrator<'a> {
    pub fn new(sig: &'a mut Iterator<Item = f64>) -> Integrator<'a> {
        let x = sig.next().unwrap().acos();

        Integrator {
            sig: sig,
            prev: x,
            accum: 0.0,
        }
    }
}

impl<'a> Iterator for Integrator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        let next = match self.sig.next() {
            None => return None,
            Some(s) => s,
        };

        let next = (next * 10000.0).trunc() / 10000.0;
        assert!(next >= -1.0 && next <= 1.0);

        let x = next.acos();
        let diff = (x - self.prev).abs();

        self.accum += next * diff;
        self.prev = x;

        Some(self.accum)
    }
}
