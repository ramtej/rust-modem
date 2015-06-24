use std::cmp;

pub struct FIRFilter<'a> {
    coefs: &'a [f64],
    history: Vec<f64>,
    idx: usize,
}

impl<'a> FIRFilter<'a> {
    pub fn new(coefs: &'a [f64]) -> FIRFilter<'a> {
        FIRFilter {
            coefs: coefs,
            history: (0..coefs.len()).map(|_| 0.0).collect(),
            idx: 0,
        }
    }

    fn calc(&self) -> f64 {
        let mut cur = self.idx;

        self.coefs.iter().fold(0.0, |s, &coef| {
            cur = cmp::min(cur - 1, self.history.len() - 1);
            s + self.history[cur] * coef
        })
    }

    pub fn add(&mut self, sample: f64) -> f64 {
        self.history[self.idx] = sample;

        self.idx += 1;
        self.idx %= self.history.len();

        self.calc()
    }
}
