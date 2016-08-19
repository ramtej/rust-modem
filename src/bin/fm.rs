extern crate modem;

mod util;

use modem::{carrier, freq, modulator, integrator, phasor};
use util::Write16;

// Samples per second.
const SAMPLES_PER_SEC: usize = 10000;
// Amplitude for the signal.
const AMPLITUDE: f32 = std::i16::MAX as f32;

fn main() {
    let mut out = std::io::stdout();

    let modul = modulator::Modulator::new(
        carrier::Carrier::new(freq::Freq::new(200, SAMPLES_PER_SEC)),
        Box::new(phasor::Raw::new(1.0))
    ).map(|x| x.re);

    let int = integrator::Integrator::new(modul, 1.0);
    let fm = Box::new(phasor::FM::new(int, AMPLITUDE,
        freq::Freq::new(200, SAMPLES_PER_SEC)));

    let fc = carrier::Carrier::new(freq::Freq::new(1400, SAMPLES_PER_SEC));
    let fmodul = modulator::Modulator::new(fc, fm);

    for sample in fmodul {
        out.write_i16(sample.re as i16).unwrap();
    }
}
