use super::{freq, integrator};

pub trait Phasor {
    fn next(&mut self, s: usize) -> Option<(f64, f64)>;
}

pub struct Raw {
    amplitude: f64,
}

impl Raw {
    pub fn new(amplitude: f64) -> Raw {
        Raw {
            amplitude: amplitude,
        }
    }

    fn i(&self) -> f64 { self.amplitude }
    fn q(&self) -> f64 { 0.0 }
}

impl Phasor for Raw {
    fn next(&mut self, _: usize) -> Option<(f64, f64)> {
        Some((self.i(), self.q()))
    }
}

pub struct FM<T: Iterator<Item = f64>> {
    integ: integrator::Integrator<T>,
    amplitude: f64,
    deviation: f64,
}

impl<T: Iterator<Item = f64>> FM<T> {
    pub fn new(integ: integrator::Integrator<T>, amplitude: f64,
               deviation: freq::Freq)
        -> FM<T>
    {
        FM {
            integ: integ,
            amplitude: amplitude / 2.0,
            deviation: deviation.sample_freq(),
        }
    }

    fn i(&self, inner: f64) -> f64 {
        self.amplitude * inner.cos()
    }

    fn q(&self, inner: f64) -> f64 {
        self.amplitude * inner.sin()
    }
}

impl<T: Iterator<Item = f64>> Phasor for FM<T> {
    fn next(&mut self, _: usize) -> Option<(f64, f64)> {
        let next = match self.integ.next() {
            Some(s) => s,
            None => return None,
        };

        let inner = self.deviation * next;

        Some((self.i(inner), self.q(inner)))
    }
}

pub struct AM {
    sig: Box<Iterator<Item = f64>>,
    amplitude: f64,
}

impl AM {
    pub fn new(sig: Box<Iterator<Item = f64>>, amplitude: f64) -> AM {
        AM {
            sig: sig,
            amplitude: amplitude,
        }
    }
}

impl Phasor for AM {
    fn next(&mut self, _: usize) -> Option<(f64, f64)> {
        let next = match self.sig.next() {
            Some(s) => s,
            None => return None,
        };

        Some((self.amplitude * next, 0.0))
    }
}
