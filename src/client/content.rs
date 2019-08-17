// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use std::{
    mem,
    io::{
        self,
        BufRead,
        BufReader,
        Read,
        Write,
    },
};

use libflate::gzip::Decoder;

use crate::client::transfer::{
    HttpTransfer,
};


enum HttpContentInner {
    None,
    Plain(HttpTransfer),
    Gzip(BufReader<Decoder<HttpTransfer>>),
}


pub struct HttpContent {
    inner: HttpContentInner,
}


impl Default for HttpContent {
    fn default() -> Self {
        HttpContent {
            inner: HttpContentInner::Plain(HttpTransfer::default()),
        }
    }
}


impl HttpContent {
    pub fn as_transfer_mut(&mut self) -> &mut HttpTransfer {
        match &mut self.inner {
            HttpContentInner::Plain(v) => v,
            HttpContentInner::Gzip(v) => v.get_mut().as_inner_mut(),
            HttpContentInner::None => unreachable!(),
        }
    }

    pub fn set_content_encoding(&mut self, encoding: &str) {
        let inner = mem::replace(&mut self.inner, HttpContentInner::None);
        let inner = match inner {
            HttpContentInner::Plain(v) => v,
            HttpContentInner::Gzip(v) => v.into_inner().into_inner(),
            HttpContentInner::None => unreachable!(),
        };
    }
}


impl Read for HttpContent {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.inner {
            HttpContentInner::Plain(v) => v.read(buf),
            HttpContentInner::Gzip(v) => v.read(buf),
            HttpContentInner::None => unreachable!(),
        }
    }
}


impl BufRead for HttpContent {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self.inner {
            HttpContentInner::Plain(v) => v.fill_buf(),
            HttpContentInner::Gzip(v) => v.fill_buf(),
            HttpContentInner::None => unreachable!(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self.inner {
            HttpContentInner::Plain(v) => v.consume(amt),
            HttpContentInner::Gzip(v) => v.consume(amt),
            HttpContentInner::None => unreachable!(),
        }
    }
}


impl Write for HttpContent {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner {
            HttpContentInner::Plain(v) => v.write(buf),
            HttpContentInner::Gzip(v) => v.get_mut().as_inner_mut().write(buf),
            HttpContentInner::None => unreachable!(),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.inner {
            HttpContentInner::Plain(v) => v.flush(),
            HttpContentInner::Gzip(v) => v.get_mut().as_inner_mut().flush(),
            HttpContentInner::None => unreachable!(),
        }
}
