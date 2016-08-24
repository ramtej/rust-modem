extern crate byteorder;
extern crate getopts;
extern crate modem;

use byteorder::{LittleEndian, WriteBytesExt};
use std::f32::consts::PI;

use modem::{phasor, modulator, digital, data};
use modem::freq::Freq;
use modem::rates::Rates;
use modem::carrier::Carrier;

// The maximum amplitude of the output waveform.
const AMPLITUDE: f32 = 1.0;

// This is a constant here so the multiline string indentation looks a little less awkward.
const USAGE: &'static str = "
    Modulate the bits on stdin to a waveform on stdout";

fn main() {
    let mut out = std::io::stdout();
    let mut parser = getopts::Options::new();

    parser.optflag("h", "help", "show usage")
          .optopt("m", "", "digital modulation to use", "MOD")
          .optopt("r", "", "sample rate (samples/sec)", "RATE")
          .optopt("b", "", "baud rate (symbols/sec)", "RATE")
          .optopt("c", "", "carrier frequency (Hz)", "FREQ")
          .optopt("p", "", "preamble cycles", "CYCLES")
          .optflag("", "iq", "output raw IQ samples");

    let args: Vec<_> = std::env::args().skip(1).collect();
    let opts = parser.parse(&args).unwrap();

    if opts.opt_present("h") {
        print!("{}\n{}", parser.short_usage("modulate"), parser.usage(USAGE));
        return;
    }

    // The digital modulation to use.
    let dmod = opts.opt_str("m").expect("digital modulation is required");

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
    let mut carrier = Carrier::new(Freq::new(cf, sr));

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
        "16apsk" => Box::new(digital::apsk::APSK::new(AMPLITUDE, 4, vec![
            digital::apsk::Ring::new(0..4, 0.5, PI / 4.0),
            digital::apsk::Ring::new(4..16, 1.0, PI / 12.0),
        ])),
        _ => panic!("invalid digital modulation"),
    };

    // Get the user-supplied bits.
    let bits = data::AsciiBits::new(std::io::stdin(), rates.samples_per_symbol,
                                    phasor.bits_per_symbol());

    let src: Box<data::Source> = match dmod.as_ref() {
        // MSK and OQPSK require an offset bit source
        "msk" | "oqpsk" =>
            Box::new(data::EvenOddOffset::new(bits, rates.samples_per_symbol,
                phasor.bits_per_symbol())),
        _ => Box::new(bits),
    };

    if opts.opt_present("iq") {
        for s in modulator::DigitalModulator::new(&mut carrier, phasor, src) {
            out.write_f32::<LittleEndian>(s.i).unwrap();
            out.write_f32::<LittleEndian>(s.q).unwrap();
        }

        return;
    }

    if pc > 0 {
        // Generate the initial carrier sync tone.
        let preamble = modulator::Modulator::new(&mut carrier,
            Box::new(phasor::Raw::new(AMPLITUDE)));

        for s in preamble.map(|x| x.modulate().re).take(sr / cf * pc - 1) {
            out.write_f32::<LittleEndian>(s).unwrap();
        }
    }

    let digi = modulator::DigitalModulator::new(&mut carrier, phasor, src)
                   .map(|x| x.modulate().re);

    for s in digi {
        out.write_f32::<LittleEndian>(s).unwrap();
    }
}
