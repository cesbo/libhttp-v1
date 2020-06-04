//! HTTP Core: Void stream
//!
//! This is a module for initial state of HttpTransfer
//! Does nothing but returns 0 on read and consume all data on write

use {
    std::io::{
        self,
        Read,
        Write,
    },

    super::HttpTransferStream,
};


#[derive(Debug)]
pub struct VoidStream;


impl Read for VoidStream {
    #[inline]
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
}


impl Write for VoidStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}


impl HttpTransferStream for VoidStream {}
