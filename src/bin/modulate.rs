extern crate byteorder;
extern crate getopts;
extern crate modem;

use byteorder::{LittleEndian, WriteBytesExt};
use std::f32::consts::PI;

use modem::{phasor, modulator, integrator, digital, data};
use modem::freq::Freq;
use modem::rates::Rates;
use modem::carrier::Carrier;

// The maximum amplitude of the output waveform.
const AMPLITUDE: f32 = 1.0;

// This is a constant here so the multiline string indentation looks a little less awkward.
const USAGE: &'static str = "
    Modulate the bits on stdin to a waveform on stdout";

fn main() {
    let mut parser = getopts::Options::new();

    parser.optflag("h", "help", "show usage")
          .optopt("m", "", "digital modulation to use", "MOD")
          .optopt("n", "", "analog modulation to use", "MOD")
          .optopt("r", "", "sample rate (samples/sec)", "RATE")
          .optopt("b", "", "baud rate (symbols/sec)", "RATE")
          .optopt("c", "", "carrier frequency (Hz)", "FREQ")
          .optopt("p", "", "preamble cycles", "CYCLES");

    let args: Vec<_> = std::env::args().skip(1).collect();
    let opts = parser.parse(&args).unwrap();

    if opts.opt_present("h") {
        print!("{}\n{}", parser.short_usage("modulate"), parser.usage(USAGE));
        return;
    }

    // The digital modulation to use.
    let dmod = opts.opt_str("m").expect("digital modulation is required");
    // The analog modulation to use.
    let amod = opts.opt_str("n");

    // The sample rate to use.
    let sr: usize = match opts.opt_str("r") {
        Some(s) => s.parse().expect("invalid sample rate"),
        None => 10000,
    };

    // The baud rate to use.
    let br: usize = match opts.opt_str("b") {
        Some(s) => s.parse().expect("invalid baud rate"),
        None => 220,
    };

    let cf: usize = match opts.opt_str("c") {
        Some(f) => f.parse().expect("invalid carrier frequency"),
        None => 1000,
    };

    let pc: usize = match opts.opt_str("p") {
        Some(c) => {
            assert!(sr % cf == 0);
            c.parse().expect("invalid preamble cycles")
        },
        None => 0,
    };

    assert!(cf < sr / 2);

    let rates = Rates::new(br, sr);
    let carrier = Carrier::new(Freq::new(cf, sr));

    // Parse the digital modulation into a phasor.
    let phasor: Box<digital::DigitalPhasor> = match dmod.as_ref() {
        "bask" => Box::new(digital::bask::BASK::new(AMPLITUDE)),
        "bpsk" => Box::new(digital::bpsk::BPSK::new(PI / 4.0, AMPLITUDE)),
        "bfsk" => Box::new(digital::bfsk::BFSK::new(Freq::new(200, sr), AMPLITUDE)),
        "qpsk" => Box::new(digital::qpsk::QPSK::new(0.0, AMPLITUDE)),
        "qam16" => Box::new(digital::qam::QAM::new(4, 0.0, AMPLITUDE)),
        "qam256" => Box::new(digital::qam::QAM::new(8, 0.0, AMPLITUDE)),
        "msk" => Box::new(digital::msk::MSK::new(AMPLITUDE, rates.samples_per_symbol)),
        "mfsk" => Box::new(digital::mfsk::MFSK::new(4, Freq::new(50, sr),
            AMPLITUDE, digital::mfsk::IncreaseMap)),
        "16psk" => Box::new(digital::mpsk::MPSK::new(4, 0.0, AMPLITUDE)),
        "oqpsk" => Box::new(digital::oqpsk::OQPSK::new(AMPLITUDE)),
        "dqpsk" => Box::new(digital::dqpsk::DQPSK::new(AMPLITUDE)),
        "16cpfsk" => Box::new(digital::cpfsk::CPFSK::new(4, rates, AMPLITUDE, 1)),
        _ => panic!("invalid digital modulation"),
    };

    // Generate the initial carrier sync tone.
    let mut preamble = modulator::Modulator::new(carrier,
        Box::new(phasor::Raw::new(AMPLITUDE)));

    if pc > 0 {
        output((&mut preamble)
            .map(|x| x.re)
            .take(sr / cf * pc - 1));
    }

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
    let digi = modulator::DigitalModulator::new(carrier, phasor, src)
        .map(|x| x.re);

    // Wrap the digital modulator in an analog modulator if necessary.
    if let Some(s) = amod {
        let aphasor: Box<phasor::Phasor> = match s.as_ref() {
            "fm" => {
                let int = integrator::Integrator::new(digi, AMPLITUDE);

                Box::new(phasor::FM::new(int, std::i16::MAX as f32,
                                         Freq::new(1000, sr)))
            },
            "am" => {
                Box::new(phasor::AM::new(digi, std::i16::MAX as f32, 0.5))
            },
            _ => panic!("invalid analog modulation"),
        };

        output(modulator::Modulator::new(
            Carrier::new(Freq::new(1000, sr)),
            aphasor
        ).map(|x| x.re));
    } else {
        output(digi);
    }
}

// Output an iterator of f32 samples to stdout as i16 samples.
fn output<T: Iterator<Item = f32>>(iter: T) {
    let mut out = std::io::stdout();

    for sample in iter {
        out.write_f32::<LittleEndian>(sample).unwrap();
    }
}
