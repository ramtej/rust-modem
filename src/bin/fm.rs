extern crate modem;

mod util;

use modem::{carrier, freq, modulator, integrator, phasor};
use util::Write16;

// Samples per second.
const SAMPLES_PER_SEC: u32 = 10000;
// Amplitude for the signal.
const AMPLITUDE: f64 = std::i16::MAX as f64;

fn main() {
    let mut out = std::io::stdout();

    let c = carrier::Carrier::new(
        freq::Freq::new(10, SAMPLES_PER_SEC));

    let p = Box::new(phasor::Raw::new(1.0));
    let modul = modulator::Modulator::new(c, p).map(|x| x.re);
    let int = integrator::Integrator::new(modul, 1.0);
    let fm = Box::new(phasor::FM::new(int, AMPLITUDE,
        freq::Freq::new(800, SAMPLES_PER_SEC)));

    let fc = carrier::Carrier::new(freq::Freq::new(800, SAMPLES_PER_SEC));
    let fmodul = modulator::Modulator::new(fc, fm);

    for sample in fmodul {
        out.write_i16(sample.re as i16).unwrap();
    }
}
