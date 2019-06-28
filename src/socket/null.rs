use std::io::{
    self,
    Read,
    Write,
};


#[derive(Debug)]
pub struct NullStream;


impl Read for NullStream {
    #[inline]
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
}


impl Write for NullStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
