use std::{
    cmp,
    io::{
        self,
        Read,
        BufRead,
        Write,
    },
};

use crate::{
    HttpVersion,
    Response,
    socket::{
        HttpSocket,
        HttpSocketError,
    },
    transfer::{
        HttpBuffer,
        HttpTransferExt,
        HttpPersist,
        HttpLength,
        HttpChunked,
    },
};


#[derive(Debug, Error)]
pub enum HttpStreamError {
    #[error_from]
    Io(io::Error),
    #[error_from]
    Socket(HttpSocketError),
}


type Result<T> = std::result::Result<T, HttpStreamError>;


/// HTTP Connection type
#[derive(Debug, PartialEq)]
enum HttpConnection {
    /// Not connected. NullStream
    None,
    /// Connected and ready for request
    Ready,
    /// Close connection
    Close,
    /// Keep connection alive
    KeepAlive,
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
#[derive(Debug)]
pub struct HttpStream {
    socket: HttpSocket,
    rbuf: HttpBuffer,
    wbuf: HttpBuffer,

    // Content-Length or Transfer-Encoding
    transfer: Box<dyn HttpTransferExt>,
    // Connection
    connection: HttpConnection,
}


impl Default for HttpStream {
    fn default() -> HttpStream {
        HttpStream {
            socket: HttpSocket::default(),
            rbuf: HttpBuffer::default(),
            wbuf: HttpBuffer::default(),

            transfer: Box::new(HttpPersist),
            connection: HttpConnection::None,
        }
    }
}


impl HttpStream {
    /// Close connection
    #[inline]
    pub fn close(&mut self) {
        self.connection = HttpConnection::None;
        self.socket.close();
    }

    /// Opens a TCP connection to a remote host
    /// If connection already opened just clears read/write buffers
    pub fn connect(&mut self, tls: bool, host: &str, port: u16) -> Result<()> {
        self.rbuf.clear();
        self.wbuf.clear();
        self.transfer = Box::new(HttpPersist);

        if self.connection == HttpConnection::None {
            self.socket.connect(tls, host, port)?;
            self.connection = HttpConnection::Ready;
        }

        Ok(())
    }

    /// Checks response headers and set content parser behavior
    /// no_content - protocol specified response without content
    pub fn configure(&mut self, no_content: bool, response: &Response) -> Result<()> {
        match response.get_version() {
            HttpVersion::HTTP10 => self.connection = HttpConnection::Close,
            _ => self.connection = HttpConnection::KeepAlive,
        }

        if let Some(connection) = response.header.get("connection") {
            if connection.eq_ignore_ascii_case("close") {
                self.connection = HttpConnection::Close
            } else if connection.eq_ignore_ascii_case("keep-alive") {
                self.connection = HttpConnection::KeepAlive
            }
        }

        if no_content {
            self.transfer = Box::new(HttpLength::new(0));
            return Ok(());
        }

        if let Some(len) = response.header.get("content-length") {
            let len = len.parse().unwrap_or(0);
            self.transfer = Box::new(HttpLength::new(len));
            return Ok(());
        }

        if let Some(encoding) = response.header.get("transfer-encoding") {
            for i in encoding.split(',').map(|v| v.trim()) {
                if i == "chunked" {
                    self.transfer = Box::new(HttpChunked::new());
                    return Ok(());
                }
            }
        }

        self.transfer = Box::new(HttpPersist);

        Ok(())
    }

    /// Reads response body from receiving buffer and stream
    #[inline]
    pub fn skip_body(&mut self) -> Result<()> {
        io::copy(self, &mut io::sink())?;
        Ok(())
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
            if self.connection == HttpConnection::Close {
                self.close();
            } else {
                self.connection = HttpConnection::Ready;
            }
            Ok(0)
        }
    }
}


impl BufRead for HttpStream {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.transfer.fill_buf(&mut self.rbuf, &mut self.socket)
    }

    fn consume(&mut self, amt: usize) {
        self.transfer.consume(amt);
        self.rbuf.pos = cmp::min(self.rbuf.cap, self.rbuf.pos + amt);
    }
}


impl Write for HttpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.wbuf.cap + buf.len() > self.wbuf.buf.len() {
            self.flush()?;
        }

        if buf.len() >= self.wbuf.buf.len() {
            self.socket.write(buf)
        } else {
            let r = (&mut self.wbuf.buf[self.wbuf.cap ..]).write(buf)?;
            self.wbuf.cap += r;
            Ok(r)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        while self.wbuf.pos < self.wbuf.cap {
            match self.socket.write(&self.wbuf.buf[self.wbuf.pos .. self.wbuf.cap]) {
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
        self.wbuf.clear();
        self.socket.flush()
    }
}
