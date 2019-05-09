use std::io::{
    self,
    Read,
    BufRead,
    Write,
};
use std::cmp;
use std::net::{
    ToSocketAddrs,
    TcpStream,
};
use std::time::Duration;

use openssl::ssl::{
    SslMethod,
    SslConnector,
    SslStream,
};

use crate::response::Response;
use crate::error::Result;


const DEFAULT_BUF_SIZE: usize = 8 * 1024;


trait Stream: Read + Write {}
impl Stream for TcpStream {}
impl Stream for SslStream<TcpStream> {}


enum HttpTransferEncoding {
    /// Reading until EOF
    Eof,
    /// Content-Length
    Length(usize),
    /// Transfer-Encoding: chunked
    /// (chunk-size, first)
    Chunked(usize, bool),
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
    timeout: Duration,
    inner: Option<Box<dyn Stream>>,
    rbuf: HttpBuffer,
    wbuf: HttpBuffer,

    transfer: HttpTransferEncoding,
}


impl Default for HttpStream {
    fn default() -> HttpStream {
        HttpStream {
            timeout: Duration::from_secs(3),
            inner: None,
            rbuf: HttpBuffer::default(),
            wbuf: HttpBuffer::default(),
            transfer: HttpTransferEncoding::Eof,
        }
    }
}


impl HttpStream {
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout
    }

    pub fn connect(&mut self, tls: bool, host: &str, port: u16) -> Result<()> {
        self.rbuf.pos = 0;
        self.rbuf.cap = 0;
        self.wbuf.pos = 0;
        self.wbuf.cap = 0;
        self.transfer = HttpTransferEncoding::Eof;

        if self.inner.is_some() {
            // keep-alive
        } else {
            let addrs = (host, port).to_socket_addrs()?;
            let get_stream = || -> io::Result<TcpStream> {
                let mut last_err = None;
                for addr in addrs {
                    match TcpStream::connect_timeout(&addr, self.timeout) {
                        Ok(v) => return Ok(v),
                        Err(e) => last_err = Some(e),
                    };
                }
                Err(last_err.unwrap_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput,
                        "could not resolve to any addresses")
                }))
            };

            let stream = get_stream()?;
            stream.set_read_timeout(Some(self.timeout))?;
            stream.set_write_timeout(Some(self.timeout))?;

            if tls {
                let connector = SslConnector::builder(SslMethod::tls())?;
                let mut ssl = connector.build().configure()?;
                ssl.set_use_server_name_indication(true);
                ssl.set_verify_hostname(true);
                let stream = ssl.connect(host, stream)?;
                self.inner = Some(Box::new(stream));
            } else {
                self.inner = Some(Box::new(stream));
            }
        }

        Ok(())
    }

    pub fn configure(&mut self, response: &Response) -> Result<()> {
        self.transfer = HttpTransferEncoding::Eof;

        if let Some(len) = response.get_header("content-length") {
            let len = len.parse().unwrap_or(0);
            self.transfer = HttpTransferEncoding::Length(len);
            return Ok(());
        }

        if let Some(te) = response.get_header("transfer-encoding") {
            // TODO: parse te
            if te == "chunked" {
                self.transfer = HttpTransferEncoding::Chunked(0, true);
                return Ok(())
            }
        }

        Ok(())
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

    fn fill_chunked(&mut self, len: usize, first: bool) -> io::Result<&[u8]> {
        let mut len = len;
        if len == 0 {
            // step:
            // 0 - check CRLF before chunk-size
            // 1 - parse chunk-size
            // 2 - skip chunk-ext
            // 3 - skip trailer
            // 4 - almost done
            // 100 - ok
            let mut step = if first { 1 } else { 0 };

            'M: loop {
                while self.rbuf.pos < self.rbuf.cap {
                    let b = self.rbuf.buf[self.rbuf.pos];
                    self.rbuf.pos += 1;

                    if step == 1 {
                        // chunk-size
                        let d = match b {
                            b'0' ..= b'9' => b - b'0',
                            b'a' ..= b'f' => b - b'a' + 10,
                            b'A' ..= b'F' => b - b'A' + 10,
                            b'\n' => {
                                if len == 0 {
                                    // skip trailer
                                    step = 3;
                                    continue
                                } else {
                                    step = 100;
                                    break 'M
                                }
                            }
                            b'\r' => { step = 4; continue }
                            b';' | b' ' | b'\t' => { step = 2; continue }
                            _ => break 'M,
                        };

                        step = 1;
                        len = len * 16 + usize::from(d);
                    }

                    else if step == 0 {
                        // skip CRLF after chunk
                        match b {
                            b'\r' => continue,
                            b'\n' => { step = 1; continue }
                            _ => break 'M,
                        }
                    }

                    else if step == 2 {
                        // skip chunk-ext
                        match b {
                            b'\r' => { step = 4; continue }
                            b'\n' if len == 0 => { step = 3; continue }
                            b'\n' => { step = 100; break 'M }
                            _ => continue,
                        }
                    }

                    else if step == 3 {
                        // skip trailer
                        match b {
                            b'\r' => { step = 4; continue }
                            b'\n' => { step = 100; break 'M }
                            _ => continue,
                        }
                    }

                    else if step == 4 {
                        // almost done
                        if b == b'\n' { step = 100 }
                        break 'M
                    }
                }

                let inner = self.inner.as_mut().unwrap(); // TODO: fix
                self.rbuf.cap = inner.read(&mut self.rbuf.buf)?;
                self.rbuf.pos = 0;
            }

            if step != 100 && self.rbuf.cap > self.rbuf.pos {
                return Err(io::Error::new(io::ErrorKind::InvalidData,
                    "invalid chunk-size format"));
            }

            if len == 0 {
                return Ok(&self.rbuf.buf[0 .. 0]);
            }

            self.transfer = HttpTransferEncoding::Chunked(len, false);
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
            HttpTransferEncoding::Chunked(len, first) => self.fill_chunked(len, first),
        }
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        match &mut self.transfer {
            HttpTransferEncoding::Eof => {},
            // TODO: chunk_size >= amt
            HttpTransferEncoding::Length(len) => *len -= amt,
            HttpTransferEncoding::Chunked(len, _) => *len -= amt,
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
                    return Err(io::Error::new(io::ErrorKind::WriteZero,
                        "failed to write the buffered data"));
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
