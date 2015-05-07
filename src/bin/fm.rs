extern crate modem;

mod consts;
mod util;

use modem::{carrier, freq, modulator, integrator};
use util::Write16;

fn main() {
    let mut out = std::io::stdout();

    let c = carrier::Basic::new(
        freq::Freq::new(10).sample_freq(consts::SAMPLES_PER_SEC));
    let mut s = carrier::CarrierSignal::new(&c);
    let mut int = integrator::Integrator::new(&mut s);

    let fc = carrier::Basic::new(
        freq::Freq::new(300).sample_freq(consts::SAMPLES_PER_SEC));

    let fm = modulator::FrequencyModulator::new(&fc, &mut int,
        consts::AMPLITUDE, 10.0);

    for sample in fm {
        out.write_i16(sample as i16).unwrap();
    }
}
