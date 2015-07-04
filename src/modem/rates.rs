#[derive(Copy, Clone)]
pub struct Rates {
    // Symbols per second.
    pub baud_rate: usize,
    // Samples per second.
    pub sample_rate: usize,
    // Samples per symbol.
    pub samples_per_symbol: usize,
}

impl Rates {
    pub fn new(br: usize, sr: usize) -> Rates {
        Rates {
            baud_rate: br,
            sample_rate: sr,
            samples_per_symbol: sr / br,
        }
    }
}
