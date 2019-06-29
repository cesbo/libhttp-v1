use std::io::{
    self,
    Read,
};

use super::{
    HttpBuffer,
    HttpTransferExt,
};


#[derive(Debug)]
pub struct HttpLength {
    len: usize,
}


impl HttpLength {
    pub fn new(len: usize) -> Self {
        HttpLength {
            len,
        }
    }
}


impl HttpTransferExt for HttpLength {
    fn fill_buf<'a>(&mut self, buf: &'a mut HttpBuffer, src: &mut dyn Read) -> io::Result<&'a [u8]> {
        if self.len == 0 {
            return Ok(&[])
        }

        if buf.pos >= buf.cap {
            buf.cap = src.read(&mut buf.buf)?;
            buf.pos = 0;
        }
        Ok(&buf.buf[buf.pos .. buf.cap])
    }

    #[inline]
    fn consume(&mut self, amt: usize) { self.len -= amt }
}
