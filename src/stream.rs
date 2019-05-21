use std::{
    cmp,
    fmt,
    io::{
        self,
        Read,
        BufRead,
        Write,
    },
    net::{
        ToSocketAddrs,
        TcpStream,
    },
    time::Duration,
};

use openssl::ssl::{
    SslMethod,
    SslConnector,
    SslStream,
};

use crate::response::Response;


const DEFAULT_IP_TTL: u32 = 64;
const DEFAULT_TCP_NODELAY: bool = false;
const DEFAULT_BUF_SIZE: usize = 8 * 1024;


trait Stream: Read + Write {}
impl Stream for TcpStream {}
impl Stream for SslStream<TcpStream> {}


#[derive(Debug)]
pub struct HttpStreamError(String);


impl fmt::Display for HttpStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "HttpStream: {}", self.0) }
}


impl From<io::Error> for HttpStreamError {
    fn from(e: io::Error) -> Self { HttpStreamError(e.to_string()) }
}


impl From<openssl::error::ErrorStack> for HttpStreamError {
    fn from(e: openssl::error::ErrorStack) -> Self {
        let s = e.errors().get(0)
            .and_then(openssl::error::Error::reason)
            .unwrap_or("");

        HttpStreamError(format!("SSL: {}", s))
    }
}


impl From<openssl::ssl::HandshakeError<TcpStream>> for HttpStreamError {
    fn from(e: openssl::ssl::HandshakeError<TcpStream>) -> Self {
        let mut result = String::from("Handshake: ");

        match &e {
            openssl::ssl::HandshakeError::SetupFailure(ee) => {
                let s = ee.errors().get(0)
                    .and_then(openssl::error::Error::reason)
                    .unwrap_or("");
                result.push_str(s);
            }
            openssl::ssl::HandshakeError::Failure(ee) => {
                let inner_error = ee.error();
                if let Some(io_ee) = inner_error.io_error() {
                    result.push_str(&io_ee.to_string());
                } else if let Some(ssl_ee) = inner_error.ssl_error() {
                    let s = ssl_ee.errors().get(0)
                        .and_then(openssl::error::Error::reason)
                        .unwrap_or("unknown");
                    result.push_str(s);
                    let v = ee.ssl().verify_result();
                    if v.as_raw() != 0 {
                        result.push_str(": ");
                        result.push_str(v.error_string());
                    }
                }
            }
            _ => unimplemented!(),
        };

        HttpStreamError(result)
    }
}


/// Internal transfer state
enum HttpTransferEncoding {
    /// Reading until EOF
    Eof,
    /// Content-Length
    Length(usize),
    /// Transfer-Encoding: chunked
    /// (chunk-size, first)
    Chunked(usize, bool),
}


/// Read/Write buffer
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


/// HTTP transport stream
///
/// Supports next features:
///
/// - synchronous tcp stream
/// - buffering reader and writer
/// - chunked transfer-encoding
/// - returns EOF if content completely readed or connection closed
/// - keep-alive
/// - TLS encryption
///
/// Usage:
///
/// ```
/// use std::io::{Read, Write};
/// use http::HttpStream;
///
/// fn main() {
///     let mut stream = HttpStream::default();
///     stream.connect(true, "example.com", 443).unwrap();
///     stream.write_all(concat!("GET / HTTP/1.0\r\n",
///         "Host: example.com\r\n",
///         "User-Agent: libhttp\r\n",
///         "\r\n").as_bytes()).unwrap();
///     stream.flush().unwrap();
///     let mut body = String::new();
///     stream.read_to_string(&mut body).unwrap();
/// }
/// ```
pub struct HttpStream {
    timeout: Duration,
    ttl: u32,
    nodelay: bool,

    inner: Option<Box<dyn Stream>>,
    rbuf: HttpBuffer,
    wbuf: HttpBuffer,

    transfer: HttpTransferEncoding,
}


impl Default for HttpStream {
    fn default() -> HttpStream {
        HttpStream {
            timeout: Duration::from_secs(3),
            ttl: DEFAULT_IP_TTL,
            nodelay: DEFAULT_TCP_NODELAY,

            inner: None,
            rbuf: HttpBuffer::default(),
            wbuf: HttpBuffer::default(),

            transfer: HttpTransferEncoding::Eof,
        }
    }
}


impl HttpStream {
    /// Sets specified timeout for connect, read, write
    /// Default: 3sec
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout
    }

    /// Sets IP_TTL. Max value 255
    /// Default: 64
    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = ttl
    }

    /// Sets TCP_NODELAY. If sets, segments are always sent as soon as possible,
    /// even if there is only a small amount of data. When not set, data is
    /// buffered until there is a sufficient amount to send out,
    /// thereby avoiding the frequent sending of small packets.
    /// Default: false
    pub fn set_nodelay(&mut self, nodelay: bool) {
        self.nodelay = nodelay
    }

    fn io_connect(&self, host: &str, port: u16) -> io::Result<TcpStream> {
        let mut last_err = None;
        let addrs = (host, port).to_socket_addrs()?;
        for addr in addrs {
            match TcpStream::connect_timeout(&addr, self.timeout) {
                Ok(v) => {
                    if self.ttl != DEFAULT_IP_TTL { v.set_ttl(self.ttl)? }
                    if self.nodelay != DEFAULT_TCP_NODELAY { v.set_nodelay(self.nodelay)? }
                    v.set_read_timeout(Some(self.timeout))?;
                    v.set_write_timeout(Some(self.timeout))?;

                    return Ok(v)
                },
                Err(e) => last_err = Some(e),
            }
        }

        Err(last_err.unwrap_or_else(||
            io::Error::new(io::ErrorKind::InvalidInput, "address resolve failed")))
    }

    /// Opens a TCP connection to a remote host
    /// If connection already opened just clears read/write buffers
    pub fn connect(&mut self, tls: bool, host: &str, port: u16) -> Result<(), HttpStreamError> {
        self.rbuf.pos = 0;
        self.rbuf.cap = 0;
        self.wbuf.pos = 0;
        self.wbuf.cap = 0;
        self.transfer = HttpTransferEncoding::Eof;

        if self.inner.is_some() {
            // keep-alive
        } else {
            let stream = self.io_connect(host, port)?;

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

    /// Checks response headers and set content parser behavior
    pub fn configure(&mut self, response: &Response) -> Result<(), HttpStreamError> {
        self.transfer = HttpTransferEncoding::Eof;

        if let Some(len) = response.header.get("content-length") {
            let len = len.parse().unwrap_or(0);
            self.transfer = HttpTransferEncoding::Length(len);
            return Ok(());
        }

        if let Some(te) = response.header.get("transfer-encoding") {
            // TODO: parse te
            if te == "chunked" {
                self.transfer = HttpTransferEncoding::Chunked(0, true);
                return Ok(())
            }
        }

        Ok(())
    }

    /// HttpTransferEncoding::Eof
    fn fill_stream(&mut self) -> io::Result<&[u8]> {
        if self.rbuf.pos >= self.rbuf.cap {
            let inner = self.inner.as_mut().unwrap(); // TODO: fix
            self.rbuf.cap = inner.read(&mut self.rbuf.buf)?;
            self.rbuf.pos = 0;
        }
        Ok(&self.rbuf.buf[self.rbuf.pos .. self.rbuf.cap])
    }

    /// HttpTransferEncoding::Length
    fn fill_length(&mut self, len: usize) -> io::Result<&[u8]> {
        if self.rbuf.pos >= self.rbuf.cap {
            let remain = cmp::min(len, self.rbuf.buf.len());
            let inner = self.inner.as_mut().unwrap(); // TODO: fix
            self.rbuf.cap = inner.read(&mut self.rbuf.buf[0 .. remain])?;
            self.rbuf.pos = 0;
        }
        Ok(&self.rbuf.buf[self.rbuf.pos .. self.rbuf.cap])
    }

    /// HttpTransferEncoding::Chunked
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
                            b'\n' if len == 0 => { step = 3; continue }
                            b'\n' => { step = 100; break 'M }
                            b'\r' => { step = 4; continue }
                            b';' | b' ' | b'\t' => { step = 2; continue }
                            _ => break 'M,
                        };

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
