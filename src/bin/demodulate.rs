extern crate getopts;
extern crate modem;
extern crate num;

mod util;

use modem::{carrier, freq, demodulator, fir};
use util::Read16;

const SAMPLE_RATE: usize = 10000;

fn main() {
    let mut parser = getopts::Options::new();

    parser.optflag("h", "help", "show usage")
          .optopt("b", "", "baud rate (symbols/sec)", "RATE");

    let args: Vec<String> = std::env::args().skip(1).collect();
    let opts = parser.parse(&args).unwrap();

    if opts.opt_present("h") {
        print!("{}", parser.short_usage("demodulate"));
        print!("{}", parser.usage(""));
        return;
    }

    let input = std::io::stdin().iter_16().map(|x| x as f64);

    let mut hfir = hilbert();
    let analytic = Box::new(input.map(move |x| {
        num::Complex::new(x, hfir.add(x))
    }));

    let carrier_freq = freq::Freq::new(900, SAMPLE_RATE);
    let mut demod = demodulator::Demodulator::new(
        carrier::Carrier::new(carrier_freq), analytic, lowpass);
    demod.lock_phase();

    for (i, q) in demod {
        println!("i:{}\tq:{}", i, q);
    }
}

// Create a Hilbert transform FIR filter. Generated with matlab.
pub fn hilbert() -> fir::FIRFilter<'static> {
    const COEFS: &'static [f64] = &[
        -0.007576,
        -2.803e-16,
        -0.019824,
        3.7096e-16,
        -0.044089,
        1.3201e-16,
        -0.089244,
        -3.2694e-16,
        -0.18728,
        -1.6739e-16,
        -0.62794,
        0.0,
        0.62794,
        1.6739e-16,
        0.18728,
        3.2694e-16,
        0.089244,
        -1.3201e-16,
        0.044089,
        -3.7096e-16,
        0.019824,
        2.803e-16,
        0.007576,
    ];

    fir::FIRFilter::new(COEFS)
}

// Create a lowpass FIR filter with
//   passband: 0 to 1000Hz
//   stopband: 1500 to 5000Hz
// Assuming a 10,000Hz sample rate. Generated with matlab.
pub fn lowpass() -> fir::FIRFilter<'static> {
    const COEFS: &'static [f64] = &[
        8.6464950643449706e-05,
        -0.0011227727551926443,
        -0.0010137373532784653,
        -0.00051892546397063074,
        0.00065737693207229997,
        0.0019426724039296576,
        0.0023575316971358984,
        0.0011698129325984573,
        -0.0014109570575621668,
        -0.0040119731215088154,
        -0.0047065995954001117,
        -0.0022692944513388992,
        0.0026579628895631122,
        0.0073998732470493874,
        0.0085194671337849165,
        0.0040456650224074651,
        -0.0046645972566385554,
        -0.012862659808170144,
        -0.014703261637603555,
        -0.0069572953029268525,
        0.00800563700908981,
        0.022172065878291854,
        0.025574286331781385,
        0.012291851983914071,
        -0.014450589851381347,
        -0.041421606566596714,
        -0.05018918856526014,
        -0.025933101216317672,
        0.03394517722329659,
        0.11612232604813434,
        0.19513123601730936,
        0.24347923270043995,
        0.24347923270043995,
        0.19513123601730936,
        0.11612232604813434,
        0.03394517722329659,
        -0.025933101216317672,
        -0.05018918856526014,
        -0.041421606566596714,
        -0.014450589851381347,
        0.012291851983914071,
        0.025574286331781385,
        0.022172065878291854,
        0.00800563700908981,
        -0.0069572953029268525,
        -0.014703261637603555,
        -0.012862659808170144,
        -0.0046645972566385554,
        0.0040456650224074651,
        0.0085194671337849165,
        0.0073998732470493874,
        0.0026579628895631122,
        -0.0022692944513388992,
        -0.0047065995954001117,
        -0.0040119731215088154,
        -0.0014109570575621668,
        0.0011698129325984573,
        0.0023575316971358984,
        0.0019426724039296576,
        0.00065737693207229997,
        -0.00051892546397063074,
        -0.0010137373532784653,
        -0.0011227727551926443,
        8.6464950643449706e-05,
    ];

    fir::FIRFilter::new(COEFS)
}
