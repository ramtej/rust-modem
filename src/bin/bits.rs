pub const BITS: &'static [u8] = &[
    // sync header
    0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
    // data
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 0, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 1, 0, 1, 0, 0, 1, 1,
    0, 1, 0, 1, 0, 0, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 1, 0, 1, 0, 0, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 0, 1, 1,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 0, 1, 1,
    0, 1, 0, 1, 0, 0, 1, 1,
    0, 1, 0, 1, 0, 0, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 1, 0, 1, 0, 0, 1, 1,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 1, 0, 1, 0, 0, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 1, 0, 1, 0, 0, 1, 1,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 1, 0, 1, 0, 0, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    0, 0, 0, 1, 1, 0, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 1, 0, 1, 0, 0, 1, 1,
    0, 0, 0, 1, 1, 0, 1, 0,
];
