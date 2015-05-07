extern crate modem;
extern crate getopts;

mod bits;
mod util;

use modem::{carrier, phasor, freq, modulator, integrator};
use util::Write16;
use getopts::Options;

fn main() {
    let mut parser = Options::new();

    parser.optflag("h", "help", "show usage")
          .optopt("m", "", "digital modulation to use", "MOD")
          .optopt("n", "", "analog modulation to use", "MOD")
          .optopt("r", "", "sample rate (samples/sec)", "RATE")
          .optopt("b", "", "baud rate (symbols/sec)", "RATE");

    let args: Vec<String> = std::env::args().skip(1).collect();
    let opts = parser.parse(&args).unwrap();

    if opts.opt_present("h") {
        print!("{}", parser.short_usage("modulate"));
        print!("{}", parser.usage(""));
        return;
    }

    let dmod = match opts.opt_str("m") {
        Some(s) => s,
        None => panic!("no modulator given"),
    };

    let amod = opts.opt_str("n");

    let sr: u32 = match opts.opt_str("r") {
        Some(s) => s.parse().unwrap(),
        None => 44000,
    };

    let br: u32 = match opts.opt_str("b") {
        Some(s) => s.parse().unwrap(),
        None => 220,
    };

    let amplitude = match amod {
        Some(_) => 1.0,
        None => std::i16::MAX as f64,
    };

    let c = carrier::Basic::new(freq::Freq::new(800).sample_freq(sr));

    let p: Box<phasor::Phasor> = {
        match dmod.as_ref() {
            "bask" => Box::new(phasor::BASK::new(amplitude)),
            "bpsk" => Box::new(phasor::BPSK::new(0.0, amplitude)),
            "bfsk" => Box::new(phasor::BFSK::new(
                freq::Freq::new(200).sample_freq(sr), amplitude)),
            "qpsk" => Box::new(phasor::QPSK::new(0.0, amplitude)),
            _ => panic!("invalid digital modulation"),
        }
    };

    let params = modulator::Params::new(br, sr);
    let mut encoder = modulator::Encoder::new(params, &c, p, &bits::BITS);

    match amod {
        Some(s) => match s.as_ref() {
            "fm" => {
                let mut int = integrator::Integrator::new(&mut encoder);
                let fc = carrier::Basic::new(freq::Freq::new(600).sample_freq(sr));
                let mut fm = modulator::FrequencyModulator::new(&fc, &mut int,
                    std::i16::MAX as f64, freq::Freq::new(200).sample_freq(sr));

                output(&mut fm);
            },
            _ => panic!("invalid analog modulation"),
        },
        None => output(&mut encoder),
    }
}

fn output(iter: &mut Iterator<Item = f64>) {
    let mut out = std::io::stdout();

    for sample in iter {
        out.write_i16(sample as i16).unwrap();
    }
}
