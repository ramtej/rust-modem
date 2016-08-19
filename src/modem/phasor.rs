use super::{freq, integrator};

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

pub struct FM<T: Iterator<Item = f32>> {
    integ: integrator::Integrator<T>,
    amplitude: f32,
    deviation: f32,
}

impl<T: Iterator<Item = f32>> FM<T> {
    pub fn new(integ: integrator::Integrator<T>, amplitude: f32,
               deviation: freq::Freq)
        -> FM<T>
    {
        FM {
            integ: integ,
            amplitude: amplitude / 2.0,
            deviation: deviation.sample_freq(),
        }
    }

    fn i(&self, inner: f32) -> f32 {
        self.amplitude * inner.cos()
    }

    fn q(&self, inner: f32) -> f32 {
        self.amplitude * inner.sin()
    }
}

impl<T: Iterator<Item = f32>> Phasor for FM<T> {
    fn next(&mut self, _: usize) -> Option<(f32, f32)> {
        let next = match self.integ.next() {
            Some(s) => s,
            None => return None,
        };

        let inner = self.deviation * next;

        Some((self.i(inner), self.q(inner)))
    }
}

pub struct AM<T: Iterator<Item = f32>> {
    sig: T,
    amplitude: f32,
    multiplier: f32,
}

impl<T: Iterator<Item = f32>> AM<T> {
    pub fn new(sig: T, amplitude: f32, multiplier: f32) -> AM<T> {
        AM {
            sig: sig,
            amplitude: amplitude / 2.0,
            multiplier: multiplier / 2.0
        }
    }
}

impl<T: Iterator<Item = f32>> Phasor for AM<T> {
    fn next(&mut self, _: usize) -> Option<(f32, f32)> {
        let next = match self.sig.next() {
            Some(s) => s,
            None => return None,
        };

        Some((self.amplitude + self.multiplier * next, 0.0))
    }
}
