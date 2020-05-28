// Copyright (C) 2019-2020 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use {
    std::{
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
    },

    openssl::ssl::{
        SslMethod,
        SslConnector,
        SslStream,
    },
};


mod null;
pub (crate) use self::null::NullStream;


#[derive(Debug, Error)]
pub enum HttpStreamError {
    #[error_from("HttpStream IO: {}", 0)]
    Io(io::Error),
    #[error_from("SSL: {}", 0)]
    Ssl(openssl::error::ErrorStack),
    #[error_from("Handshake: {}", 0)]
    Handshake(openssl::ssl::HandshakeError<TcpStream>),
}


type Result<T> = std::result::Result<T, HttpStreamError>;


trait Stream: Read + Write + fmt::Debug {}


impl Stream for NullStream {}
impl Stream for TcpStream {}
impl Stream for SslStream<TcpStream> {}


/// HTTP socket - abstraction over TcpStream or SslStream
#[derive(Debug)]
pub struct HttpStream {
    timeout: Duration,
    inner: Box<dyn Stream>,
}


impl Default for HttpStream {
    fn default() -> Self {
        HttpStream {
            timeout: Duration::from_secs(3),
            inner: Box::new(NullStream),
        }
    }
}


impl HttpStream {
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
    pub fn connect(&mut self, tls: bool, host: &str, port: u16) -> Result<()> {
        let stream = self.io_connect(host, port)?;

        if tls {
            let connector = SslConnector::builder(SslMethod::tls())?;
            let mut ssl = connector.build().configure()?;
            ssl.set_use_server_name_indication(true);
            ssl.set_verify_hostname(true);
            let stream = ssl.connect(host, stream)?;
            self.inner = Box::new(stream);
        } else {
            self.inner = Box::new(stream);
        }

        Ok(())
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


#[cfg(test)]
mod tests {
    #[test]
    fn test_socket() {
        use super::*;
        use std::io::{Read, Write};

        let mut socket = HttpStream::default();
        socket.connect(true, "example.com", 443).unwrap();
        socket.write_all(concat!("GET / HTTP/1.0\r\n",
            "Host: example.com\r\n",
            "User-Agent: libhttp\r\n",
            "\r\n").as_bytes()).unwrap();
        socket.flush().unwrap();
        let mut body = String::new();
        socket.read_to_string(&mut body).unwrap();
    }
}
