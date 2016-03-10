extern crate modem;
extern crate getopts;

mod util;

use modem::{carrier, phasor, freq, modulator, integrator, digital, data, rates};
use util::Write16;

// The maximum amplitude of the output waveform.
const AMPLITUDE: f64 = std::i16::MAX as f64;

// This is a constant here so the multiline string indentation looks a little less awkward.
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

    // The digital modulation to use.
    let dmod = match opts.opt_str("m") {
        Some(s) => s,
        None => panic!("no modulator given"),
    };

    // The analog modulation to use.
    let amod = opts.opt_str("n");

    // The sample rate to use.
    let sr: usize = match opts.opt_str("r") {
        Some(s) => s.parse().unwrap(),
        None => 10000,
    };

    // The baud rate to use.
    let br: usize = match opts.opt_str("b") {
        Some(s) => s.parse().unwrap(),
        None => 220,
    };

    let rates = rates::Rates::new(br, sr);

    // The 900Hz carrier frequency is arbitrary but sounds good.
    let carrier = carrier::Carrier::new(freq::Freq::new(900, sr));

    // Parse the digital modulation into a phasor.
    let phasor: Box<digital::DigitalPhasor> = match dmod.as_ref() {
        "bask" => Box::new(digital::BASK::new(AMPLITUDE)),
        "bpsk" => Box::new(digital::BPSK::new(std::f64::consts::PI/4.0, AMPLITUDE)),
        "bfsk" => Box::new(digital::BFSK::new(freq::Freq::new(200, sr), AMPLITUDE)),
        "qpsk" => Box::new(digital::QPSK::new(0.0, AMPLITUDE)),
        "qam16" => Box::new(digital::QAM::new(4, 0.0, AMPLITUDE)),
        "qam256" => Box::new(digital::QAM::new(8, 0.0, AMPLITUDE)),
        "msk" => Box::new(digital::MSK::new(AMPLITUDE, rates.samples_per_symbol)),
        "mfsk" => Box::new(digital::MFSK::new(4, freq::Freq::new(50, sr),
            AMPLITUDE, digital::IncreaseMap)),
        "16psk" => Box::new(digital::MPSK::new(4, 0.0, AMPLITUDE)),
        "oqpsk" => Box::new(digital::OQPSK::new(AMPLITUDE)),
        "dqpsk" => Box::new(digital::DQPSK::new(AMPLITUDE)),
        "16cpfsk" => Box::new(digital::CPFSK::new(4, rates, AMPLITUDE, 1)),
        _ => panic!("invalid digital modulation"),
    };

    // Generate the initial carrier sync tone.
    let mut preamble = modulator::Modulator::new(carrier,
        Box::new(phasor::Raw::new(AMPLITUDE)));

    output((&mut preamble)
        .map(|x| x.re)
        .take(rates.samples_per_symbol * 8));

    // Retrieve the carrier back from the preamble.
    let carrier = preamble.into_carrier();

    // Get the user-supplied bits.
    let bits = data::AsciiBits::new(std::io::stdin(), rates.samples_per_symbol,
                                    phasor.bits_per_symbol());
    let src: Box<data::Source> = match dmod.as_ref() {
        // MSK and OQPSK require an offset bit source
        "msk" | "oqpsk" =>
            Box::new(data::EvenOddOffset::new(bits,
                rates.samples_per_symbol,
                phasor.bits_per_symbol())),
        _ => Box::new(bits),
    };

    // Create the digital modulator and use only the real part of the signal.
    let dmodul = modulator::DigitalModulator::new(carrier, phasor, src)
        .map(|x| x.re);

    // Wrap the digital modulator in an analog modulator if necessary.
    if let Some(s) = amod {
        let aphasor: Box<phasor::Phasor> = match s.as_ref() {
            "fm" => {
                let int = integrator::Integrator::new(dmodul, AMPLITUDE);

                Box::new(phasor::FM::new(int, std::i16::MAX as f64,
                                         freq::Freq::new(1000, sr)))
            },
            "am" => {
                Box::new(phasor::AM::new(dmodul, std::i16::MAX as f64, 0.5))
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

// Output an iterator of f64 samples to stdout as i16 samples.
fn output<T: Iterator<Item = f64>>(iter: T) {
    let mut out = std::io::stdout();

    for sample in iter {
        out.write_i16(sample as i16).unwrap();
    }
}
