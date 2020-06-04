//! HTTP Core: Transfer module
//!
//! Implements IO buffer and transfer encoding algorithms:
//!
//! - persist - waits while server closes connection
//! - length - reads content limited by `Content-Length`
//! - chunked - reads content divided into a series of non-overlapping chunks

mod void;
mod buffer;
mod chunked;
mod length;
mod persist;


use {
    std::{
        cmp,
        fmt,
        io::{
            self,
            BufRead,
            Read,
            Write,
        },
    },

    self::{
        void::VoidStream,
        buffer::HttpBuffer,
        chunked::HttpChunked,
        length::HttpLength,
        persist::HttpPersist,
    },
};


pub trait HttpTransferStream: Read + Write + fmt::Debug {}


trait HttpTransferExt: fmt::Debug {
    fn fill_buf<'a>(&mut self, buf: &'a mut HttpBuffer, src: &mut dyn Read) -> io::Result<&'a [u8]>;
    fn consume(&mut self, amt: usize);
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
    rbuf: HttpBuffer,
    wbuf: HttpBuffer,

    stream: Box<dyn HttpTransferStream>,
    transfer: Box<dyn HttpTransferExt>,
}


impl Default for HttpTransfer {
    fn default() -> Self {
        HttpTransfer {
            rbuf: HttpBuffer::default(),
            wbuf: HttpBuffer::default(),

            stream: Box::new(VoidStream),
            transfer: Box::new(HttpPersist),
        }
    }
}


impl HttpTransfer {
    /// Attach Stream
    #[inline]
    pub fn init(&mut self, stream: Box<dyn HttpTransferStream>) { self.stream = stream }

    /// Clear IO buffers
    /// Should be called before each request
    pub fn clear(&mut self) {
        self.rbuf.clear();
        self.wbuf.clear();
        self.transfer = Box::new(HttpPersist);
    }

    /// Close connection
    #[inline]
    pub fn close(&mut self) {
        self.clear();
        self.init(Box::new(VoidStream));
    }

    /// Content-Length defined in the headers or response without content
    #[inline]
    pub fn set_transfer_length(&mut self, len: usize) {
        if self.rbuf.cap - self.rbuf.pos > len {
            self.rbuf.cap = self.rbuf.pos + len;
        }
        self.transfer = Box::new(HttpLength::new(len))
    }

    /// Transfer-Encoded: chunked
    #[inline]
    pub fn set_transfer_chunked(&mut self) { self.transfer = Box::new(HttpChunked::new()) }

    /// Receive content until connection closed
    #[inline]
    pub fn set_transfer_persist(&mut self) { self.transfer = Box::new(HttpPersist) }
}


impl Read for HttpTransfer {
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


impl BufRead for HttpTransfer {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.transfer.fill_buf(&mut self.rbuf, &mut self.stream)
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
            self.stream.write(buf)
        } else {
            let r = (&mut self.wbuf.buf[self.wbuf.cap ..]).write(buf)?;
            self.wbuf.cap += r;
            Ok(r)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        while self.wbuf.pos < self.wbuf.cap {
            match self.stream.write(&self.wbuf.buf[self.wbuf.pos .. self.wbuf.cap]) {
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
        self.stream.flush()
    }
}
