// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use std::io::{
    self,
    BufRead,
    Read,
    Write,
};


#[derive(Debug)]
pub struct NullStream;


impl Read for NullStream {
    #[inline]
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> { Ok(0) }
}


impl BufRead for NullStream {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> { Ok(&[]) }

    #[inline]
    fn consume(&mut self, _amt: usize) {}
}


impl Write for NullStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
