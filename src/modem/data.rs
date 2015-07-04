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
            // Ensure the index moved forward and only by one symbol.
            assert!(idx == self.cur_idx + 1);

            Some(idx)
        }
    }

    fn sample_to_idx(&self, s: usize) -> usize {
        s / self.samples_per_symbol
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

pub struct EvenOddOffset<D: Source> {
    data: D,
    clock: SymbolClock,
    cur: [u8; 2],
}

impl<D: Source> EvenOddOffset<D> {
    pub fn new(data: D, samples_per_symbol: usize, bits_per_symbol: usize)
        -> EvenOddOffset<D>
    {
        assert!(bits_per_symbol == 2);

        EvenOddOffset {
            data: data,
            clock: SymbolClock::new(samples_per_symbol / bits_per_symbol),
            cur: [0, 0],
        }
    }
}

impl<D: Source> Source for EvenOddOffset<D> {
    fn update(&mut self, s: usize) -> SourceUpdate {
        let bits = match self.data.update(s) {
            SourceUpdate::Finished => return SourceUpdate::Finished,
            SourceUpdate::Changed(b) => {
                self.clock.update(s);
                self.cur[0] = b[0];

                return SourceUpdate::Changed(&self.cur[..]);
            }
            SourceUpdate::Unchanged(b) => b,
        };

        // Half-symbol update?
        if self.clock.update(s) {
            self.cur[1] = bits[1];
            SourceUpdate::Changed(&self.cur[..])
        } else {
            SourceUpdate::Unchanged(&self.cur[..])
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

    #[test]
    fn test_evenodd() {
        use super::{EvenOddOffset, Bits, Source, SourceUpdate};

        const BITS: &'static [u8] = &[1, 0, 0, 1];

        let ds = Bits::new(BITS, 10, 2);
        let mut eo = EvenOddOffset::new(ds, 10, 2);

        assert!(match eo.update(0) {
            SourceUpdate::Changed(b) => b[0] == 1 && b[1] == 0,
            _ => false,
        });

        assert!(match eo.update(3) {
            SourceUpdate::Unchanged(b) => b[0] == 1 && b[1] == 0,
            _ => false,
        });

        assert!(match eo.update(5) {
            SourceUpdate::Changed(b) => b[0] == 1 && b[1] == 0,
            _ => false,
        });

        assert!(match eo.update(10) {
            SourceUpdate::Changed(b) => b[0] == 0 && b[1] == 0,
            _ => false,
        });

        assert!(match eo.update(14) {
            SourceUpdate::Unchanged(b) => b[0] == 0 && b[1] == 0,
            _ => false,
        });

        assert!(match eo.update(15) {
            SourceUpdate::Changed(b) => b[0] == 0 && b[1] == 1,
            _ => false,
        });

        assert!(match eo.update(16) {
            SourceUpdate::Unchanged(b) => b[0] == 0 && b[1] == 1,
            _ => false,
        });

        assert!(match eo.update(20) {
            SourceUpdate::Finished => true,
            _ => false,
        });
    }
}
