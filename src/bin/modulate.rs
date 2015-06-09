extern crate modem;
extern crate getopts;

mod bits;
mod util;

use modem::{carrier, phasor, freq, modulator, integrator, digital};
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

    let params = modulator::Params::new(br, sr);
    let c = carrier::Carrier::new(freq::Freq::new(800, sr));

    let p: Box<digital::DigitalPhasor> = {
        match dmod.as_ref() {
            "bask" => Box::new(digital::BASK::new(amplitude)),
            "bpsk" => Box::new(digital::BPSK::new(0.0, amplitude)),
            "bfsk" => Box::new(digital::BFSK::new(freq::Freq::new(200, sr),
                               amplitude)),
            "qpsk" => Box::new(digital::QPSK::new(0.0, amplitude)),
            _ => panic!("invalid digital modulation"),
        }
    };

    let dmodul = Box::new(
        modulator::DigitalModulator::new(params, c, p, bits::BITS).map(|x| x.re)
    );

    let mut modul: Box<Iterator<Item = f64>> = if let Some(s) = amod {
        let aphasor: Box<phasor::Phasor> = match s.as_ref() {
            "fm" => {
                let int = integrator::Integrator::new(dmodul);

                Box::new(phasor::FM::new(int, std::i16::MAX as f64,
                                         freq::Freq::new(1000, sr)))
            },
            "am" => {
                Box::new(phasor::AM::new(dmodul, std::i16::MAX as f64))
            },
            _ => panic!("invalid analog modulation"),
        };

        let fc = carrier::Carrier::new(freq::Freq::new(1000, sr));
        Box::new(modulator::Modulator::new(fc, aphasor).map(|x| x.re))
    } else {
        dmodul
    };

    output(&mut modul);
}

fn output(iter: &mut Iterator<Item = f64>) {
    let mut out = std::io::stdout();

    for sample in iter {
        out.write_i16(sample as i16).unwrap();
    }
}
