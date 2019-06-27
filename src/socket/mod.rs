use std::{
    fmt,
    io::{
        self,
        Read,
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

mod null;
use self::null::NullStream;

mod ssl;
use self::ssl::{
    SslError,
    HandshakeError,
};


#[derive(Debug, Error)]
pub enum HttpSocketError {
    #[error_from("HttpSocket IO: {}", 0)]
    Io(io::Error),
    #[error_from("SSL: {}", 0)]
    Ssl(SslError),
    #[error_from("Handshake: {}", 0)]
    Handshake(HandshakeError),
}


type Result<T> = std::result::Result<T, HttpSocketError>;


trait Stream: Read + Write + fmt::Debug {}


impl Stream for NullStream {}
impl Stream for TcpStream {}
impl Stream for SslStream<TcpStream> {}


#[derive(Debug)]
pub struct HttpSocket {
    timeout: Duration,
    inner: Box<dyn Stream>,
}


impl Default for HttpSocket {
    fn default() -> HttpSocket {
        HttpSocket {
            timeout: Duration::from_secs(3),
            inner: Box::new(NullStream),
        }
    }
}


impl HttpSocket {
    /// Close connection
    #[inline]
    pub fn close(&mut self) {
        self.inner = Box::new(NullStream);
    }

    fn io_connect(&self, host: &str, port: u16) -> io::Result<TcpStream> {
        let mut last_err = None;
        let addrs = (host, port).to_socket_addrs()?;
        for addr in addrs {
            match TcpStream::connect_timeout(&addr, self.timeout) {
                Ok(v) => {
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
    pub fn connect(&mut self, tls: bool, host: &str, port: u16) -> Result<()> {
        let stream = self.io_connect(host, port)?;

        if tls {
            let connector = SslConnector::builder(SslMethod::tls()).map_err(SslError::from)?;
            let mut ssl = connector.build().configure().map_err(SslError::from)?;
            ssl.set_use_server_name_indication(true);
            ssl.set_verify_hostname(true);
            let stream = ssl.connect(host, stream).map_err(HandshakeError::from)?;
            self.inner = Box::new(stream);
        } else {
            self.inner = Box::new(stream);
        }

        Ok(())
    }
}


impl Read for HttpSocket {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.inner.read(buf) }
}


impl Write for HttpSocket {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.inner.write(buf) }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { self.inner.flush() }
}
