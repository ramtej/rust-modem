use modem::{phasor, carrier};

pub struct Signal<C: carrier::Carrier> {
    carrier: C,
    phasor: Box<phasor::Phasor>,
    prev: u8,
}

impl<C: carrier::Carrier> Signal<C> {
    pub fn new(c: C, p: Box<phasor::Phasor>) -> Signal<C> {
        Signal {
            carrier: c,
            phasor: p,
            prev: 0,
        }
    }

    pub fn eval(&mut self, s: u32, b: u8) -> f64 {
        if b != self.prev {
            self.phasor.update(s, b);
            self.prev = b;
        }

        self.phasor.i(s, b) * self.carrier.inner(s).cos() -
        self.phasor.q(s, b) * self.carrier.inner(s).sin()
    }
}
