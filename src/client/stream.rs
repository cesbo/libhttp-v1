//! HTTP Core: Client stream
//!
//! TCP Socket with TLS if needed

use {
    std::{
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
    },

    openssl::ssl::{
        SslMethod,
        SslConnector,
        SslStream,
    },

    crate::{
        Result,
        transfer::HttpTransferStream,
    },
};


impl HttpTransferStream for TcpStream {}
impl HttpTransferStream for SslStream<TcpStream> {}


/// HTTP socket - abstraction over TcpStream or SslStream
#[derive(Debug)]
pub struct HttpStream {
    inner: Box<dyn HttpTransferStream>,
}


impl HttpStream {
    fn io_connect(host: &str, port: u16) -> Result<TcpStream> {
        let mut last_error = None;

        let timeout = Duration::from_secs(3);
        let addrs = (host, port).to_socket_addrs()?;

        for addr in addrs {
            match TcpStream::connect_timeout(&addr, timeout) {
                Ok(v) => {
                    v.set_read_timeout(Some(timeout))?;
                    v.set_write_timeout(Some(timeout))?;
                    return Ok(v);
                },
                Err(e) => last_error = Some(e),
            }
        }

        if let Some(e) = last_error {
            bail!(e);
        } else {
            bail!("address not resolved");
        }
    }

    /// Opens a TCP connection to a remote host
    pub fn connect(tls: bool, host: &str, port: u16) -> Result<HttpStream> {
        let stream = Self::io_connect(host, port)?;

        if tls {
            let connector = SslConnector::builder(SslMethod::tls())?;
            let mut ssl = connector.build().configure()?;
            ssl.set_use_server_name_indication(true);
            ssl.set_verify_hostname(true);
            let stream = ssl.connect(host, stream)?;

            Ok(HttpStream {
                inner: Box::new(stream),
            })
        }

        else {
            Ok(HttpStream {
                inner: Box::new(stream),
            })
        }
    }
}


impl Read for HttpStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.inner.read(buf) }
}


impl Write for HttpStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.inner.write(buf) }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { self.inner.flush() }
}


impl HttpTransferStream for HttpStream {}


#[cfg(test)]
mod tests {
    #[test]
    fn test_socket() {
        use super::*;
        use std::io::{Read, Write};

        let mut socket = HttpStream::connect(true, "example.com", 443).unwrap();
        socket.write_all(concat!("GET / HTTP/1.0\r\n",
            "Host: example.com\r\n",
            "User-Agent: libhttp\r\n",
            "\r\n").as_bytes()).unwrap();
        socket.flush().unwrap();
        let mut body = String::new();
        socket.read_to_string(&mut body).unwrap();
    }
}
