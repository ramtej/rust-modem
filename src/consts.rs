use std;

// Samples per second.
pub const SAMPLES_PER_SEC: u32 = 44000;
// Bits per second.
pub const BAUD: u32 = 60;
// Samples per bit.
pub const SAMPLES_PER_BIT: u32 = SAMPLES_PER_SEC / BAUD;
// Amplitude for the signal.
pub const AMPLITUDE: f64 = std::i16::MAX as f64;
