use std::io::{
    self,
    Read,
    BufRead,
    Write,
};
use std::cmp;
use std::net::TcpStream;


const DEFAULT_BUF_SIZE: usize = 8 * 1024;


enum HttpTransferEncoding {
    /// Reading until EOF
    Eof,
    /// Content-Length
    Length(usize),
    /// Transfer-Encoding: chunked
    Chunked(usize),
}


impl Default for HttpTransferEncoding {
    fn default() -> HttpTransferEncoding {
        HttpTransferEncoding::Eof
    }
}


struct HttpBuffer {
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
}


impl Default for HttpBuffer {
    fn default() -> HttpBuffer {
        HttpBuffer {
            buf: {
                let mut v = Vec::with_capacity(DEFAULT_BUF_SIZE);
                unsafe { v.set_len(DEFAULT_BUF_SIZE) };
                v.into_boxed_slice()
            },
            pos: 0,
            cap: 0,
        }
    }
}


#[derive(Default)]
pub struct HttpStream {
    inner: Option<TcpStream>,
    rbuf: HttpBuffer,
    wbuf: HttpBuffer,

    transfer: HttpTransferEncoding,
}


impl HttpStream {
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.inner.is_some()
    }

    pub (crate) fn clear(&mut self) {
        self.rbuf.pos = 0;
        self.rbuf.cap = 0;
        self.wbuf.pos = 0;
        self.wbuf.cap = 0;
        self.transfer = HttpTransferEncoding::Eof;
    }

    #[inline]
    pub (crate) fn set(&mut self, inner: TcpStream) {
        self.inner = Some(inner);
    }

    #[inline]
    pub (crate) fn set_stream_eof(&mut self) {
        self.transfer = HttpTransferEncoding::Eof;
    }

    #[inline]
    pub (crate) fn set_stream_length(&mut self, len: usize) {
        self.transfer = HttpTransferEncoding::Length(len);
    }

    #[inline]
    pub (crate) fn set_stream_chunked(&mut self) {
        self.transfer = HttpTransferEncoding::Chunked(0);
    }

    fn fill_stream(&mut self) -> io::Result<&[u8]> {
        if self.rbuf.pos >= self.rbuf.cap {
            let inner = self.inner.as_mut().unwrap(); // TODO: fix
            self.rbuf.cap = inner.read(&mut self.rbuf.buf)?;
            self.rbuf.pos = 0;
        }
        Ok(&self.rbuf.buf[self.rbuf.pos .. self.rbuf.cap])
    }

    fn fill_length(&mut self, len: usize) -> io::Result<&[u8]> {
        if self.rbuf.pos >= self.rbuf.cap {
            let remain = cmp::min(len, self.rbuf.buf.len());
            let inner = self.inner.as_mut().unwrap(); // TODO: fix
            self.rbuf.cap = inner.read(&mut self.rbuf.buf[0 .. remain])?;
            self.rbuf.pos = 0;
        }
        Ok(&self.rbuf.buf[self.rbuf.pos .. self.rbuf.cap])
    }

    fn fill_chunked(&mut self, len: usize) -> io::Result<&[u8]> {
        let mut len = len;

        if len == 0 {
            // step:
            // 0 - wait first digit
            // 1 - parse digit
            // 2 - skip chunk-ext
            let mut step = 0;
            loop {
                while self.rbuf.pos < self.rbuf.cap {
                    let b = self.rbuf.buf[self.rbuf.pos];
                    self.rbuf.pos += 1;

                    if step == 2 {
                        if b == b'\n' {
                            break;
                        } else {
                            continue;
                        }
                    }

                    let d = match b {
                        b'0' ..= b'9' => b - b'0',
                        b'a' ..= b'f' => b - b'a' + 10,
                        b'A' ..= b'F' => b - b'A' + 10,

                        b'\r' => continue,
                        b'\n' if step == 0 => continue,
                        b'\n' => break,

                        b';' => {
                            step = 2;
                            continue;
                        },

                        _ => {
                            return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid chunk-size format"));
                        },
                    };

                    step = 1;
                    len = len * 16 + usize::from(d);
                }

                if self.rbuf.cap > self.rbuf.pos {
                    break;
                }

                let inner = self.inner.as_mut().unwrap(); // TODO: fix
                self.rbuf.cap = inner.read(&mut self.rbuf.buf)?;
                self.rbuf.pos = 0;
            }

            self.transfer = HttpTransferEncoding::Chunked(len);
            if len == 0 {
                return Ok(&self.rbuf.buf[0 .. 0]);
            }
        }

        if self.rbuf.pos >= self.rbuf.cap {
            let inner = self.inner.as_mut().unwrap(); // TODO: fix
            self.rbuf.cap = inner.read(&mut self.rbuf.buf)?;
            self.rbuf.pos = 0;
        }

        if len >= self.rbuf.cap - self.rbuf.pos {
            Ok(&self.rbuf.buf[self.rbuf.pos .. self.rbuf.cap])
        } else {
            Ok(&self.rbuf.buf[self.rbuf.pos .. self.rbuf.pos + len])
        }
    }
}


impl Read for HttpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut rem = self.fill_buf()?;
        if ! rem.is_empty() {
            let nread = rem.read(buf)?;
            self.consume(nread);
            Ok(nread)
        } else {
            Ok(0)
        }
    }
}


impl BufRead for HttpStream {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self.transfer {
            HttpTransferEncoding::Eof => self.fill_stream(),
            HttpTransferEncoding::Length(len) => self.fill_length(len),
            HttpTransferEncoding::Chunked(len) => self.fill_chunked(len),
        }
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        match &mut self.transfer {
            HttpTransferEncoding::Eof => {},
            HttpTransferEncoding::Length(len) => *len -= amt,
            HttpTransferEncoding::Chunked(len) => *len -= amt,
        }
        self.rbuf.pos = cmp::min(self.rbuf.pos + amt, self.rbuf.cap);
    }
}


impl Write for HttpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.wbuf.cap + buf.len() > self.wbuf.buf.len() {
            self.flush()?;
        }

        if buf.len() >= self.wbuf.buf.len() {
            let inner = self.inner.as_mut().unwrap(); // TODO: fix
            inner.write(buf)
        } else {
            let r = (&mut self.wbuf.buf[self.wbuf.cap ..]).write(buf)?;
            self.wbuf.cap += r;
            Ok(r)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let inner = self.inner.as_mut().unwrap(); // TODO: fix
        while self.wbuf.pos < self.wbuf.cap {
            match inner.write(&self.wbuf.buf[self.wbuf.pos .. self.wbuf.cap]) {
                Ok(0) => {
                    return Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write the buffered data"));
                },
                Ok(n) => { self.wbuf.pos += n },
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {},
                Err(e) => {
                    return Err(e);
                },
            }
        }
        self.wbuf.pos = 0;
        self.wbuf.cap = 0;
        inner.flush()
    }
}
