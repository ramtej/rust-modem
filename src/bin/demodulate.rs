extern crate getopts;
extern crate modem;
extern crate num;

mod util;

use modem::{carrier, freq, demodulator};
use util::Read16;

const SAMPLE_RATE: usize = 10000;

fn main() {
    let input = std::io::stdin().iter_16().map(|x| x as f64);

    let mut hfir = util::hilbert();
    let analytic = Box::new(input.map(move |x| {
        num::Complex::new(x, hfir.add(x))
    }));

    let carrier_freq = freq::Freq::new(900, SAMPLE_RATE);
    let mut demod = demodulator::Demodulator::new(
        carrier::Carrier::new(carrier_freq), analytic, util::lowpass);
    demod.lock_phase();

    for (i, q) in demod {
        println!("  i:{} q:{}", i, q);
    }
}
