use std::cmp;

pub struct FIRFilter<'a> {
    coefs: &'a [f32],
    history: Vec<f32>,
    idx: usize,
}

impl<'a> FIRFilter<'a> {
    pub fn new(coefs: &'a [f32]) -> FIRFilter<'a> {
        FIRFilter {
            coefs: coefs,
            history: vec![0.0; coefs.len()],
            idx: 0,
        }
    }

    fn calc(&self) -> f32 {
        let mut cur = self.idx;

        self.coefs.iter().fold(0.0, |s, &coef| {
            cur = cmp::min(cur - 1, self.history.len() - 1);
            s + self.history[cur] * coef
        })
    }

    pub fn add(&mut self, sample: f32) -> f32 {
        self.history[self.idx] = sample;

        self.idx += 1;
        self.idx %= self.history.len();

        self.calc()
    }
}
