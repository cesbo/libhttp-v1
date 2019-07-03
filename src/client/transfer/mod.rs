use std::{
    cmp,
    fmt,
    io::{
        self,
        BufRead,
        Read,
        Write,
    },
};

pub (crate) mod stream;
use self::stream::{
    HttpStream,
    HttpStreamError,
};

pub (crate) mod buffer;
use self::buffer::HttpBuffer;

mod chunked;
use self::chunked::HttpChunked;

mod length;
use self::length::HttpLength;

mod persist;
use self::persist::HttpPersist;


trait HttpTransferExt: fmt::Debug {
    fn fill_buf<'a>(&mut self, buf: &'a mut HttpBuffer, src: &mut dyn Read) -> io::Result<&'a [u8]>;
    fn consume(&mut self, amt: usize);
}


#[derive(Debug, Error)]
pub enum HttpTransferError {
    #[error_from]
    Io(io::Error),
    #[error_from]
    Socket(HttpStreamError),
}


type Result<T> = std::result::Result<T, HttpTransferError>;


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


/// HTTP transport
///
/// Supports next features:
///
/// - buffering reader and writer
/// - chunked transfer-encoding
/// - returns EOF if content completely readed or connection closed
/// - keep-alive
#[derive(Debug)]
pub struct HttpTransfer {
    socket: HttpStream,
    rbuf: HttpBuffer,
    wbuf: HttpBuffer,

    transfer: Box<dyn HttpTransferExt>,
    connection: HttpConnection,
}


impl Default for HttpTransfer {
    fn default() -> Self {
        HttpTransfer {
            socket: HttpStream::default(),
            rbuf: HttpBuffer::default(),
            wbuf: HttpBuffer::default(),

            transfer: Box::new(HttpPersist),
            connection: HttpConnection::None,
        }
    }
}


impl HttpTransfer {
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

    /// Close connection after end of response
    #[inline]
    pub fn set_connection_close(&mut self) { self.connection = HttpConnection::Close }

    /// Keep connection alive after end of response
    #[inline]
    pub fn set_connection_keep_alive(&mut self) { self.connection = HttpConnection::KeepAlive }

    /// Content-Length defined in the headers or response without content
    #[inline]
    pub fn set_content_length(&mut self, len: usize) { self.transfer = Box::new(HttpLength::new(len)) }

    /// Transfer-Encoded: chunked
    #[inline]
    pub fn set_content_chunked(&mut self) { self.transfer = Box::new(HttpChunked::new()) }

    /// Receive content until connection closed
    #[inline]
    pub fn set_content_persist(&mut self) { self.transfer = Box::new(HttpPersist) }
}


impl Read for HttpTransfer {
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


impl BufRead for HttpTransfer {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.transfer.fill_buf(&mut self.rbuf, &mut self.socket)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.transfer.consume(amt);
        self.rbuf.pos = cmp::min(self.rbuf.cap, self.rbuf.pos + amt);
    }
}


impl Write for HttpTransfer {
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
