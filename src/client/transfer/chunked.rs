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
pub struct HttpChunked {
    first: bool,
    len: usize,
}


impl HttpChunked {
    pub fn new() -> Self {
        HttpChunked {
            first: true,
            len: 0,
        }
    }
}


impl HttpTransferExt for HttpChunked {
    fn fill_buf<'a>(&mut self, buf: &'a mut HttpBuffer, src: &mut dyn Read) -> io::Result<&'a [u8]> {
        if self.len == 0 {
            // step:
            // 0 - check CRLF before chunk-size
            // 1 - parse chunk-size
            // 2 - skip chunk-ext
            // 3 - skip trailer
            // 4 - almost done
            // 100 - ok
            let mut step = 0;
            if self.first {
                step = 1;
                self.first = false;
            }

            loop {
                if buf.pos >= buf.cap {
                    buf.cap = src.read(&mut buf.buf)?;
                    buf.pos = 0;
                }

                let b = buf.buf[buf.pos];
                buf.pos += 1;

                if step == 1 {
                    // chunk-size
                    let d = match b {
                        b'0' ..= b'9' => b - b'0',
                        b'a' ..= b'f' => b - b'a' + 10,
                        b'A' ..= b'F' => b - b'A' + 10,
                        b'\n' if self.len == 0 => { step = 3; continue }
                        b'\n' => { step = 100; break }
                        b'\r' => { step = 4; continue }
                        b';' | b' ' | b'\t' => { step = 2; continue }
                        _ => break,
                    };

                    self.len = self.len * 16 + usize::from(d);
                }

                else if step == 0 {
                    // skip CRLF after chunk
                    match b {
                        b'\r' => continue,
                        b'\n' => { step = 1; continue }
                        _ => break,
                    }
                }

                else if step == 2 {
                    // skip chunk-ext
                    match b {
                        b'\r' => { step = 4; continue }
                        b'\n' if self.len == 0 => { step = 3; continue }
                        b'\n' => { step = 100; break }
                        _ => continue,
                    }
                }

                else if step == 3 {
                    // skip trailer
                    match b {
                        b'\r' => { step = 4; continue }
                        b'\n' => { step = 100; break }
                        _ => continue,
                    }
                }

                else if step == 4 {
                    // almost done
                    if b == b'\n' { step = 100 }
                    break
                }
            }

            if step != 100 {
                return Err(io::Error::new(io::ErrorKind::InvalidData,
                    "invalid chunk-size format"));
            }

            if self.len == 0 {
                return Ok(&[]);
            }
        }

        if buf.pos >= buf.cap {
            buf.cap = src.read(&mut buf.buf)?;
            buf.pos = 0;
        }

        let remain = cmp::min(buf.cap, buf.pos + self.len);
        Ok(&buf.buf[buf.pos .. remain])
    }

    #[inline]
    fn consume(&mut self, amt: usize) { self.len -= amt }
}
