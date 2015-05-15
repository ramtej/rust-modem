extern crate modem;

mod util;

use modem::{carrier, freq, modulator, integrator};
use util::Write16;

// Samples per second.
const SAMPLES_PER_SEC: u32 = 44000;
// Amplitude for the signal.
const AMPLITUDE: f64 = std::i16::MAX as f64;

fn main() {
    let mut out = std::io::stdout();

    let c = carrier::Carrier::new(
        freq::Freq::new(10, SAMPLES_PER_SEC));
    let mut s = carrier::CarrierSignal::new(&c);
    let mut int = integrator::Integrator::new(&mut s);

    let fc = carrier::Carrier::new(
        freq::Freq::new(300, SAMPLES_PER_SEC));

    let fm = modulator::FrequencyModulator::new(&fc, &mut int,
        AMPLITUDE, freq::Freq::new(10, SAMPLES_PER_SEC));

    for sample in fm {
        out.write_i16(sample as i16).unwrap();
    }
}
