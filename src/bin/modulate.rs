extern crate modem;
extern crate getopts;

mod util;

use modem::{carrier, phasor, freq, modulator, integrator, digital, data, rates};
use util::Write16;

const AMPLITUDE: f64 = std::i16::MAX as f64;

const USAGE: &'static str = "
    Modulate the bits on stdin to a waveform on stdout";

fn main() {
    let mut parser = getopts::Options::new();

    parser.optflag("h", "help", "show usage")
          .optopt("m", "", "digital modulation to use", "MOD")
          .optopt("n", "", "analog modulation to use", "MOD")
          .optopt("r", "", "sample rate (samples/sec)", "RATE")
          .optopt("b", "", "baud rate (symbols/sec)", "RATE");

    let args: Vec<String> = std::env::args().skip(1).collect();
    let opts = parser.parse(&args).unwrap();

    if opts.opt_present("h") {
        print!("{}\n{}", parser.short_usage("modulate"), parser.usage(USAGE));
        return;
    }

    let dmod = match opts.opt_str("m") {
        Some(s) => s,
        None => panic!("no modulator given"),
    };

    let amod = opts.opt_str("n");

    let sr: usize = match opts.opt_str("r") {
        Some(s) => s.parse().unwrap(),
        None => 10000,
    };

    let br: usize = match opts.opt_str("b") {
        Some(s) => s.parse().unwrap(),
        None => 220,
    };

    let rates = rates::Rates::new(br, sr);
    let carrier = carrier::Carrier::new(freq::Freq::new(900, sr));

    let phasor: Box<digital::DigitalPhasor> = match dmod.as_ref() {
        "bask" => Box::new(digital::BASK::new(AMPLITUDE)),
        "bpsk" => Box::new(digital::BPSK::new(std::f64::consts::PI/4.0, AMPLITUDE)),
        "bfsk" => Box::new(digital::BFSK::new(freq::Freq::new(200, sr),
                           AMPLITUDE)),
        "qpsk" => Box::new(digital::QPSK::new(0.0, AMPLITUDE)),
        "qam16" => Box::new(digital::QAM::new(4, 0.0, AMPLITUDE)),
        "qam256" => Box::new(digital::QAM::new(8, 0.0, AMPLITUDE)),
        "msk" => Box::new(digital::MSK::new(AMPLITUDE,
                                            rates.samples_per_symbol)),
        "mfsk" => Box::new(digital::MFSK::new(4, freq::Freq::new(50, sr),
            AMPLITUDE, digital::IncreaseMap)),
        "16psk" => Box::new(digital::MPSK::new(4, 0.0, AMPLITUDE)),
        "oqpsk" => Box::new(digital::OQPSK::new(AMPLITUDE)),
        "dqpsk" => Box::new(digital::DQPSK::new(AMPLITUDE)),
        "16cpfsk" => Box::new(digital::CPFSK::new(4, rates, AMPLITUDE, 1)),
        _ => panic!("invalid digital modulation"),
    };

    let mut preamble = modulator::Modulator::new(carrier,
        Box::new(phasor::Raw::new(AMPLITUDE)));

    output((&mut preamble)
        .map(|x| x.re)
        .take(rates.samples_per_symbol * 8));

    let carrier = preamble.into_carrier();

    let bits = data::AsciiBits::new(std::io::stdin(), rates.samples_per_symbol,
                                    phasor.bits_per_symbol());
    let src: Box<data::Source> = match dmod.as_ref() {
        "msk" | "oqpsk" =>
            Box::new(data::EvenOddOffset::new(bits,
                rates.samples_per_symbol,
                phasor.bits_per_symbol())),
        _ => Box::new(bits),
    };

    let dmodul = modulator::DigitalModulator::new(carrier, phasor, src)
        .map(|x| x.re);

    if let Some(s) = amod {
        let aphasor: Box<phasor::Phasor> = match s.as_ref() {
            "fm" => {
                let int = integrator::Integrator::new(dmodul, AMPLITUDE);

                Box::new(phasor::FM::new(int, std::i16::MAX as f64,
                                         freq::Freq::new(1000, sr)))
            },
            "am" => {
                Box::new(phasor::AM::new(dmodul, std::i16::MAX as f64))
            },
            _ => panic!("invalid analog modulation"),
        };

        let modul = modulator::Modulator::new(
            carrier::Carrier::new(freq::Freq::new(1000, sr)),
            aphasor
        ).map(|x| x.re);

        output(modul);
    } else {
        output(dmodul);
    };
}

fn output<T: Iterator<Item = f64>>(iter: T) {
    let mut out = std::io::stdout();

    for sample in iter {
        out.write_i16(sample as i16).unwrap();
    }
}
