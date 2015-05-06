use std;

pub trait Phasor {
    fn i(&self, s: u32, b: u8) -> f64;
    fn q(&self, s: u32, b: u8) -> f64;

    fn update(&mut self, _s: u32, _b: u8) {}
}

fn bit_to_sign(b: u8) -> f64 {
    if b == 0 {
        -1.0
    } else {
        1.0
    }
}

pub struct BPSK {
    phase: f64,
    amplitude: f64,
}

impl BPSK {
    pub fn new(phase: f64, amplitude: f64) -> BPSK {
        BPSK {
            phase: phase,
            amplitude: amplitude,
        }
    }

    fn common(&self, b: u8) -> f64 {
        bit_to_sign(b) * self.amplitude
    }
}

impl Phasor for BPSK {
    fn i(&self, _: u32, b: u8) -> f64 {
        self.common(b) * self.phase.cos()
    }

    fn q(&self, _: u32, b: u8) -> f64 {
        self.common(b) * self.phase.sin()
    }
}

pub struct BASK {
    amplitude: f64,
}

impl BASK {
    pub fn new(a: f64) -> BASK {
        BASK {
            amplitude: a,
        }
    }
}

impl Phasor for BASK {
    fn i(&self, _: u32, b: u8) -> f64 {
        b as f64 * self.amplitude
    }

    fn q(&self, _: u32, _: u8) -> f64 {
        0.0
    }
}

pub struct BFSK {
    deviation: f64,
    amplitude: f64,
    phase: f64,
}

impl BFSK {
    pub fn new(d: f64, a: f64) -> BFSK {
        BFSK {
            deviation: d,
            amplitude: a,
            phase: 0.0,
        }
    }

    fn inner(&self, s: u32, b: u8) -> f64 {
        self.rads(s, b) + self.phase
    }

    fn rads(&self, s: u32, b: u8) -> f64 {
        b as f64 * self.deviation * s as f64
    }
}

fn mod_trig(x: f64) -> f64 {
    const TWO_PI: f64 = std::f64::consts::PI * 2.0;

    x - TWO_PI * (x / TWO_PI).floor()
}

impl Phasor for BFSK {
    fn i(&self, s: u32, b: u8) -> f64 {
        self.amplitude * self.inner(s, b).cos()
    }

    fn q(&self, s: u32, b: u8) -> f64 {
        self.amplitude * self.inner(s, b).sin()
    }

    fn update(&mut self, s: u32, b: u8) {
        self.phase = mod_trig(self.phase + if b == 1 {
            -self.rads(s, 1)
        } else {
            self.rads(s - 1, 1)
        });
    }
}
