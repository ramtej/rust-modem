use std;

pub trait Phasor {
    fn group_size(&self) -> u32;

    fn i(&self, s: usize, bits: &[u8]) -> f64;
    fn q(&self, s: usize, bits: &[u8]) -> f64;

    fn update(&mut self, _s: usize, _b: &[u8]) {}
}

fn bit_to_sign(b: u8) -> f64 {
    if b == 0 {
        -1.0
    } else {
        1.0
    }
}

fn mod_trig(x: f64) -> f64 {
    const TWO_PI: f64 = std::f64::consts::PI * 2.0;

    x - TWO_PI * (x / TWO_PI).floor()
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
    fn group_size(&self) -> u32 { 1 }

    fn i(&self, _: usize, b: &[u8]) -> f64 {
        self.common(b[0]) * self.phase.cos()
    }

    fn q(&self, _: usize, b: &[u8]) -> f64 {
        self.common(b[0]) * self.phase.sin()
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
    fn group_size(&self) -> u32 { 1 }

    fn i(&self, _: usize, b: &[u8]) -> f64 {
        b[0] as f64 * self.amplitude
    }

    fn q(&self, _: usize, _: &[u8]) -> f64 {
        0.0
    }
}

pub struct BFSK {
    deviation: f64,
    amplitude: f64,
    phase: f64,
    prev: u8,
}

impl BFSK {
    pub fn new(d: f64, a: f64) -> BFSK {
        BFSK {
            deviation: d,
            amplitude: a,
            phase: 0.0,
            prev: 0,
        }
    }

    fn inner(&self, s: usize, b: u8) -> f64 {
        self.rads(s, b) + self.phase
    }

    fn rads(&self, s: usize, b: u8) -> f64 {
        b as f64 * self.deviation * s as f64
    }
}

impl Phasor for BFSK {
    fn group_size(&self) -> u32 { 1 }

    fn i(&self, s: usize, b: &[u8]) -> f64 {
        self.amplitude * self.inner(s, b[0]).cos()
    }

    fn q(&self, s: usize, b: &[u8]) -> f64 {
        self.amplitude * self.inner(s, b[0]).sin()
    }

    fn update(&mut self, s: usize, b: &[u8]) {
        if b[0] == self.prev {
            return;
        }

        self.phase = mod_trig(self.phase + if b[0] == 1 {
            -self.rads(s, 1)
        } else {
            self.rads(s - 1, 1)
        });

        self.prev = b[0];
    }
}
