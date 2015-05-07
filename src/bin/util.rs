use std;

pub trait Write16 {
    fn write_i16(&mut self, n: i16) -> std::io::Result<usize>;
}

impl<W> Write16 for W where W: std::io::Write {
    fn write_i16(&mut self, n: i16) -> std::io::Result<usize> {
        let buf: [u8; 2] = unsafe { std::mem::transmute(n) };
        self.write(&buf)
    }
}
