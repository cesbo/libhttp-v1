use std::{
    fmt,
    io::{
        self,
        Read,
    },
};

mod buffer;
pub (crate) use self::buffer::HttpBuffer;

mod chunked;
pub (crate) use self::chunked::HttpChunked;

mod length;
pub (crate) use self::length::HttpLength;

mod persist;
pub (crate) use self::persist::HttpPersist;


pub (crate) trait HttpTransferExt: fmt::Debug {
    fn fill_buf<'a>(&mut self, buf: &'a mut HttpBuffer, src: &mut dyn Read) -> io::Result<&'a [u8]>;
    fn consume(&mut self, amt: usize);
}
