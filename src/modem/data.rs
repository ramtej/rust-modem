use std;

#[derive(PartialEq, Eq)]
pub enum SourceUpdate<'a> {
    Changed(&'a [u8]),
    Unchanged(&'a [u8]),
    Finished,
}

pub trait Source {
    fn next(&mut self) -> SourceUpdate;
}

struct SymbolClock {
    samples_per_symbol: usize,
    counter: usize,
}

impl SymbolClock {
    pub fn new(samples_per_symbol: usize) -> SymbolClock {
        SymbolClock {
            samples_per_symbol: samples_per_symbol,
            counter: samples_per_symbol - 1,
        }
    }

    pub fn next(&mut self) -> bool {
        self.counter += 1;
        self.counter %= self.samples_per_symbol;

        self.counter == 0
    }
}

pub struct Bits<'a> {
    bits: &'a [u8],
    clock: SymbolClock,
    bits_per_symbol: usize,
    idx: usize,
}

impl<'a> Bits<'a> {
    pub fn new(bits: &'a [u8], samples_per_symbol: usize, bits_per_symbol: usize)
        -> Bits<'a>
    {
        Bits {
            bits: bits,
            clock: SymbolClock::new(samples_per_symbol),
            bits_per_symbol: bits_per_symbol,
            idx: 0,
        }
    }

    fn bits(&self) -> Option<&[u8]> {
        let start = (self.idx - 1) * self.bits_per_symbol;
        let end = start + self.bits_per_symbol;

        if end <= self.bits.len() {
            Some(&self.bits[start..end])
        } else {
            None
        }
    }
}

impl<'a> Source for Bits<'a> {
    fn next(&mut self) -> SourceUpdate {
        if self.clock.next() {
            self.idx += 1;

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
        assert!(samples_per_symbol % bits_per_symbol == 0);

        EvenOddOffset {
            data: data,
            clock: SymbolClock::new(samples_per_symbol / bits_per_symbol),
            cur: [0, 0],
        }
    }
}

impl<D: Source> Source for EvenOddOffset<D> {
    fn next(&mut self) -> SourceUpdate {
        let bits = match self.data.next() {
            SourceUpdate::Finished => return SourceUpdate::Finished,
            SourceUpdate::Changed(b) => {
                self.clock.next();
                self.cur[0] = b[0];

                return SourceUpdate::Changed(&self.cur[..]);
            }
            SourceUpdate::Unchanged(b) => b,
        };

        // Half-symbol update?
        if self.clock.next() {
            self.cur[1] = bits[1];
            SourceUpdate::Changed(&self.cur[..])
        } else {
            SourceUpdate::Unchanged(&self.cur[..])
        }
    }
}

pub struct AsciiBits<R: std::io::Read> {
    stream: R,
    clock: SymbolClock,
    bits: Vec<u8>,
}

impl<R: std::io::Read> AsciiBits<R> {
    pub fn new(stream: R, samples_per_symbol: usize, bits_per_symbol: usize)
        -> AsciiBits<R>
    {
        AsciiBits {
            stream: stream,
            clock: SymbolClock::new(samples_per_symbol),
            bits: vec![0; bits_per_symbol],
        }
    }

    fn next_bit(&mut self) -> Option<u8> {
        loop {
            let mut buf = [0; 1];

            let bit = match self.stream.read(&mut buf) {
                Ok(1) => buf[0],
                _ => return None,
            };

            if (bit as char).is_whitespace() {
                continue;
            }

            assert!((bit as char).is_digit(2));

            return Some(bit - b'0');
        }
    }

    fn read_bits(&mut self) -> bool {
        // Needs Vec::as_mut_slice to be cleaner, but it's unstable.
        for i in 0..self.bits.len() {
            self.bits[i] = match self.next_bit() {
                Some(b) => b,
                None => return false,
            }
        }

        true
    }
}

impl<R: std::io::Read> Source for AsciiBits<R> {
    fn next(&mut self) -> SourceUpdate {
        if self.clock.next() {
            if self.read_bits() {
                SourceUpdate::Changed(&self.bits[..])
            } else {
                SourceUpdate::Finished
            }
        } else {
            SourceUpdate::Unchanged(&self.bits[..])
        }
    }
}

#[cfg(test)]
mod test {
    use std;
    use std::io::Write;
    use super::{Bits, Source, SourceUpdate, SymbolClock, EvenOddOffset, AsciiBits};

    #[test]
    fn test_symbol_clock() {
        let mut bc = SymbolClock::new(5);

        assert!(bc.next());
        assert!(!bc.next());
        assert!(!bc.next());
        assert!(!bc.next());
        assert!(!bc.next());
        assert!(bc.next());
        assert!(!bc.next());
        assert!(!bc.next());
        assert!(!bc.next());
        assert!(!bc.next());
        assert!(bc.next());
    }

    #[test]
    fn test_bits() {
        const BITS: &'static [u8] = &[1, 0, 1, 1];

        let mut ds = Bits::new(BITS, 3, 2);

        assert!(ds.next() == SourceUpdate::Changed(&[1, 0]));
        assert!(ds.next() == SourceUpdate::Unchanged(&[1, 0]));
        assert!(ds.next() == SourceUpdate::Unchanged(&[1, 0]));
        assert!(ds.next() == SourceUpdate::Changed(&[1, 1]));
        assert!(ds.next() == SourceUpdate::Unchanged(&[1, 1]));
        assert!(ds.next() == SourceUpdate::Unchanged(&[1, 1]));
        assert!(ds.next() == SourceUpdate::Finished);
    }

    #[test]
    fn test_evenodd() {
        const BITS: &'static [u8] = &[1, 1, 1, 0, 0, 1];

        let ds = Bits::new(BITS, 4, 2);
        let mut eo = EvenOddOffset::new(ds, 4, 2);

        assert!(eo.next() == SourceUpdate::Changed(&[1, 0]));
        assert!(eo.next() == SourceUpdate::Unchanged(&[1, 0]));
        assert!(eo.next() == SourceUpdate::Changed(&[1, 1]));
        assert!(eo.next() == SourceUpdate::Unchanged(&[1, 1]));
        assert!(eo.next() == SourceUpdate::Changed(&[1, 1]));
        assert!(eo.next() == SourceUpdate::Unchanged(&[1, 1]));
        assert!(eo.next() == SourceUpdate::Changed(&[1, 0]));
        assert!(eo.next() == SourceUpdate::Unchanged(&[1, 0]));
        assert!(eo.next() == SourceUpdate::Changed(&[0, 0]));
        assert!(eo.next() == SourceUpdate::Unchanged(&[0, 0]));
        assert!(eo.next() == SourceUpdate::Changed(&[0, 1]));
        assert!(eo.next() == SourceUpdate::Unchanged(&[0, 1]));
        assert!(eo.next() == SourceUpdate::Finished);
    }

    #[test]
    fn test_ascii() {
        {
            let mut f = std::fs::File::create("ascii.bits").unwrap();
            f.write_all(b"000\n111\n101").unwrap();
        }

        {
            let f = std::fs::File::open("ascii.bits").unwrap();
            let mut a = AsciiBits::new(f, 1, 3);

            assert!(a.read_bits());
            assert!(a.read_bits());
            assert!(a.read_bits());
            assert!(!a.read_bits());
        }

        {
            let f = std::fs::File::open("ascii.bits").unwrap();
            let mut a = AsciiBits::new(f, 2, 3);

            assert!(a.next() == SourceUpdate::Changed(&[0,0,0]));
            assert!(a.next() == SourceUpdate::Unchanged(&[0,0,0]));
            assert!(a.next() == SourceUpdate::Changed(&[1,1,1]));
            assert!(a.next() == SourceUpdate::Unchanged(&[1,1,1]));
            assert!(a.next() == SourceUpdate::Changed(&[1,0,1]));
            assert!(a.next() == SourceUpdate::Unchanged(&[1,0,1]));
            assert!(a.next() == SourceUpdate::Finished);
        }

        std::fs::remove_file("ascii.bits").unwrap();
    }
}
