use std;

pub enum SourceUpdate<'a> {
    Changed(&'a [u8]),
    Unchanged(&'a [u8]),
    Finished,
}

pub trait Source {
    fn update(&mut self, s: usize) -> SourceUpdate;
}

struct SymbolClock {
    cur_idx: usize,
    samples_per_symbol: usize,
}

impl SymbolClock {
    pub fn new(samples_per_symbol: usize) -> SymbolClock {
        SymbolClock {
            cur_idx: std::usize::MAX,
            samples_per_symbol: samples_per_symbol,
        }
    }

    pub fn symbol_idx(&self) -> usize { self.cur_idx }

    pub fn update(&mut self, s: usize) -> bool {
        match self.check(s) {
            None => false,
            Some(idx) => {
                self.cur_idx = idx;
                true
            }
        }
    }

    fn check(&self, s: usize) -> Option<usize> {
        let idx = self.sample_to_idx(s);

        if self.cur_idx == idx {
            None
        } else {
            Some(idx)
        }
    }

    fn sample_to_idx(&self, s: usize) -> usize {
        s / self.samples_per_symbol as usize
    }
}

pub struct Bits<'a> {
    bits: &'a [u8],
    bits_per_symbol: usize,
    clock: SymbolClock,
}

impl<'a> Bits<'a> {
    pub fn new(bits: &'a [u8], samples_per_symbol: usize, bits_per_symbol: usize)
        -> Bits<'a>
    {
        Bits {
            bits: bits,
            bits_per_symbol: bits_per_symbol,
            clock: SymbolClock::new(samples_per_symbol),
        }
    }

    fn bits(&self) -> Option<&[u8]> {
        let start = self.clock.symbol_idx() * self.bits_per_symbol;
        let end = start + self.bits_per_symbol;

        if end <= self.bits.len() {
            Some(&self.bits[start..end])
        } else {
            None
        }
    }
}

impl<'a> Source for Bits<'a> {
    fn update(&mut self, s: usize) -> SourceUpdate {
        if self.clock.update(s) {
            match self.bits() {
                Some(b) => SourceUpdate::Changed(b),
                None => SourceUpdate::Finished,
            }
        } else {
            SourceUpdate::Unchanged(self.bits().unwrap())
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_bitclock() {
        use super::{SymbolClock};

        let mut bc = SymbolClock::new(10);

        assert!(bc.update(0));
        assert!(bc.symbol_idx() == 0);

        assert!(bc.update(10));
        assert!(bc.symbol_idx() == 1);

        assert_eq!(bc.sample_to_idx(100), 10);
    }

    #[test]
    fn test_bits() {
        use super::{Bits, Source, SourceUpdate};

        const BITS: &'static [u8] = &[1, 0, 1, 1];

        let mut ds = Bits::new(BITS, 10, 2);

        assert!(match ds.update(0) {
            SourceUpdate::Changed(b) => b[0] == 1 && b[1] == 0,
            _ => false,
        });

        assert!(if let SourceUpdate::Unchanged(_) = ds.update(5) { true } else { false });

        assert!(match ds.update(10) {
            SourceUpdate::Changed(b) => b[0] == 1 && b[1] == 1,
            _ => false,
        });

        assert!(if let SourceUpdate::Unchanged(_) = ds.update(18) { true } else { false });

        assert!(match ds.update(20) {
            SourceUpdate::Finished => true,
            _ => false,
        });
    }
}