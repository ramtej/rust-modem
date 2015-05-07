mod bits;
mod consts;
mod modem;
mod util;

use modem::{carrier, phasor, freq, modulator, integrator};
use util::Write16;

fn main() {
    let mut args = std::env::args().skip(1);

    let modulator = match args.next() {
        Some(s) => s,
        None => panic!("no modulator given"),
    };

    let do_fm = match args.next() {
        Some(s) => s == "fm",
        None => false,
    };

    let amplitude = if do_fm {
        1.0
    } else {
        consts::AMPLITUDE
    };

    let c = carrier::Basic::new(
        freq::Freq::new(800).sample_freq(consts::SAMPLES_PER_SEC));

    let p: Box<phasor::Phasor> = {
        match modulator.as_ref() {
            "bask" => Box::new(phasor::BASK::new(amplitude)),
            "bpsk" => Box::new(phasor::BPSK::new(0.0, amplitude)),
            "bfsk" => Box::new(phasor::BFSK::new(
                freq::Freq::new(200).sample_freq(consts::SAMPLES_PER_SEC),
                amplitude)),
            "qpsk" => Box::new(phasor::QPSK::new(0.0, amplitude)),
            _ => panic!("invalid modulator"),
        }
    };

    let params = modulator::Params::new(consts::BAUD, consts::SAMPLES_PER_SEC);
    let mut encoder = modulator::Encoder::new(params, &c, p, &bits::BITS);

    let mut out = std::io::stdout();

    if !do_fm {
        for sample in encoder {
            out.write_i16(sample as i16).unwrap();
        }
    } else {
        let mut int = integrator::Integrator::new(&mut encoder);
        let fc = carrier::Basic::new(
            freq::Freq::new(600).sample_freq(consts::SAMPLES_PER_SEC));
        let fm = modulator::FrequencyModulator::new(&fc, &mut int, consts::AMPLITUDE,
            freq::Freq::new(200).sample_freq(consts::SAMPLES_PER_SEC));

        for sample in fm {
            out.write_i16(sample as i16).unwrap();
        }
    }
}
