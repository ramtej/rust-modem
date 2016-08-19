use std;

pub trait Read16 {
    fn read_i16(&mut self) -> std::io::Result<i16>;

    fn iter_16(self) -> Iter16<Self>
        where Self: Sized
    {
        Iter16(self)
    }
}

impl<R> Read16 for R where R: std::io::Read {
    fn read_i16(&mut self) -> std::io::Result<i16> {
        let mut buf = [0, 0];

        match self.read(&mut buf) {
            Ok(2) => Ok(unsafe { std::mem::transmute(buf) }),
            Ok(_) => Err(std::io::Error::new(std::io::ErrorKind::Other,
                                             "no more words available")),
            Err(e) => Err(e),
        }
    }
}

pub struct Iter16<R: Read16>(R);

impl<R: Read16> Iterator for Iter16<R> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.read_i16() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }
}
