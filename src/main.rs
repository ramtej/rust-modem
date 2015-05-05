mod bits;
mod consts;
mod modem;
mod util;

use modem::{carrier, phasor, freq, signal};
use util::Write16;

fn main() {
    let mut out = std::io::stdout();

    let c = carrier::Basic::new(
        freq::Freq::new(400).sample_freq(consts::SAMPLES_PER_SEC));

    let p: Box<phasor::Phasor> = {
        let s = match std::env::args().skip(1).next() {
            Some(s) => s,
            None => panic!("no modulator given"),
        };

        match s.as_ref() {
            "bask" => Box::new(phasor::BASK::new(consts::AMPLITUDE)),
            "bpsk" => Box::new(phasor::BPSK::new(0.0, consts::AMPLITUDE)),
            "bfsk" => Box::new(phasor::BFSK::new(
                freq::Freq::new(400).sample_freq(consts::SAMPLES_PER_SEC),
                consts::AMPLITUDE)),
            _ => panic!("invalid modulator"),
        }
    };

    let mut sig = signal::Signal::new(c, p);

    let samples = consts::SAMPLES_PER_BIT * bits::BITS.len() as u32;

    for s in 0..samples {
        let bit = bits::BITS[s as usize / consts::SAMPLES_PER_BIT as usize];
        out.write_i16(sig.eval(s, bit) as i16).unwrap();
    }
}

