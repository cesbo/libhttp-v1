// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use std::{
    cmp,
    io::{
        self,
        Read,
    },
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
            let remain = cmp::min(buf.buf.len(), self.len);
            buf.cap = src.read(&mut buf.buf[.. remain])?;
            buf.pos = 0;
        }

        Ok(&buf.buf[buf.pos .. buf.cap])
    }

    #[inline]
    fn consume(&mut self, amt: usize) { self.len -= amt }
}
