pub fn bit_to_sign(b: u8) -> f32 {
    (2 * b as i8 - 1) as f32
}

pub fn bytes_to_bits(bytes: &[u8]) -> u8 {
    let len = bytes.len() - 1;

    bytes.iter().enumerate().fold(0, |s, (i, &b)| {
        s | (b & 1) << (len - i)
    })
}

pub fn max_symbol(bits_per_symbol: usize) -> usize {
    (1 << bits_per_symbol) - 1
}

#[cfg(test)]
mod test {
    use super::{bytes_to_bits, max_symbol};

    #[test]
    fn test_b2b() {
        assert_eq!(bytes_to_bits(&[0, 0, 0, 1]), 0b0001);
        assert_eq!(bytes_to_bits(&[0, 1, 0, 1]), 0b0101);
    }

    #[test]
    fn test_max_symbol() {
        assert_eq!(max_symbol(1), 0b1);
        assert_eq!(max_symbol(2), 0b11);
        assert_eq!(max_symbol(4), 0b1111);
        assert_eq!(max_symbol(8), 0b11111111);
    }
}
