use std::io::{
    self,
    Read,
    BufRead,
    Write,
};
use std::cmp;
use std::net::TcpStream;


pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;


enum HttpTransferEncoding {
    /// Stream without defined length. Reading until EOF
    Eof,
    /// Content-Length
    Length(usize),
    /// Transfer-Encoding: chunked
    Chunked(usize),
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


pub struct HttpStream {
    inner: TcpStream,
    wbuf: HttpBuffer,
    rbuf: HttpBuffer,

    transfer: HttpTransferEncoding,
}


impl HttpStream {
    pub fn new(inner: TcpStream) -> HttpStream {
        HttpStream {
            inner,
            wbuf: HttpBuffer::default(),
            rbuf: HttpBuffer::default(),
            transfer: HttpTransferEncoding::Eof,
        }
    }

    #[inline]
    pub fn set_stream_eof(&mut self) {
        self.transfer = HttpTransferEncoding::Eof;
    }

    #[inline]
    pub fn set_stream_length(&mut self, len: usize) {
        self.transfer = HttpTransferEncoding::Length(len);
    }

    #[inline]
    pub fn set_stream_chunked(&mut self) {
        self.transfer = HttpTransferEncoding::Chunked(0);
    }

    fn fill_stream(&mut self) -> io::Result<&[u8]> {
        dbg!((self.rbuf.pos, self.rbuf.cap));
        if self.rbuf.pos >= self.rbuf.cap {
            self.rbuf.cap = self.inner.read(&mut self.rbuf.buf)?;
            self.rbuf.pos = 0;
        }
        Ok(&self.rbuf.buf[self.rbuf.pos .. self.rbuf.cap])
    }

    fn fill_length(&mut self, len: usize) -> io::Result<&[u8]> {
        if self.rbuf.pos >= self.rbuf.cap {
            let remain = dbg!(cmp::min(len, self.rbuf.buf.len()));
            self.rbuf.cap = self.inner.read(&mut self.rbuf.buf[0 .. remain])?;
            self.rbuf.pos = 0;
        }
        Ok(&self.rbuf.buf[self.rbuf.pos .. self.rbuf.cap])
    }

    fn fill_chunked(&mut self, _len: usize) -> io::Result<&[u8]> {
        unimplemented!()
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
            self.inner.write(buf)
        } else {
            let r = (&mut self.wbuf.buf[self.wbuf.cap ..]).write(buf)?;
            self.wbuf.cap += r;
            Ok(r)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        while self.wbuf.pos < self.wbuf.cap {
            match self.inner.write(&self.wbuf.buf[self.wbuf.pos .. self.wbuf.cap]) {
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
        self.inner.flush()
    }
}
