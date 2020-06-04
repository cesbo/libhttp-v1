//! HTTP Core: persist connection

use std::io::{
    self,
    Read,
};

use super::{
    HttpBuffer,
    HttpTransferExt,
};


#[derive(Debug)]
pub struct HttpPersist;


impl HttpTransferExt for HttpPersist {
    fn fill_buf<'a>(&mut self, buf: &'a mut HttpBuffer, src: &mut dyn Read) -> io::Result<&'a [u8]> {
        if buf.pos >= buf.cap {
            buf.cap = src.read(&mut buf.buf)?;
            buf.pos = 0;
        }
        Ok(&buf.buf[buf.pos .. buf.cap])
    }

    #[inline]
    fn consume(&mut self, _amt: usize) {}
}
