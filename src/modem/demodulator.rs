extern crate num;
use super::{carrier, pll, fir};

const LOCK_SAMPLES: usize = 64;

pub struct Demodulator<'a, S>
    where S: Iterator<Item = num::Complex<f64>>
{
    carrier: carrier::Carrier,
    sig: S,
    pll: pll::PLL,
    lpi: fir::FIRFilter<'a>,
    lpq: fir::FIRFilter<'a>,
}

impl<'a, S> Demodulator<'a, S>
    where S: Iterator<Item = num::Complex<f64>>
{
    pub fn new<F>(carrier: carrier::Carrier, sig: S, lp: F) -> Demodulator<'a, S>
        where F: Fn() -> fir::FIRFilter<'a>
    {
        Demodulator {
            carrier: carrier,
            sig: sig,
            pll: pll::PLL::new(),
            lpi: lp(),
            lpq: lp(),
        }
    }

    pub fn lock_phase(&mut self) {
        for _ in 0..LOCK_SAMPLES {
            self.pll.handle(self.carrier.next(), self.sig.next().unwrap());
        }
    }
}

impl<'a, S> Iterator for Demodulator<'a, S>
    where S: Iterator<Item = num::Complex<f64>>
{
    type Item = (f64, f64);

    fn next(&mut self) -> Option<(f64, f64)> {
        let x = match self.sig.next() {
            Some(x) => x.re,
            None => return None,
        };

        let phase = self.carrier.next() + self.pll.phase_offset;

        Some((
            2.0 * self.lpi.add(x * phase.cos()),
            2.0 * self.lpq.add(x * -phase.sin())
        ))
    }
}
