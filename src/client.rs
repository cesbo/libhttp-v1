use std::net::TcpStream;
use std::io::{
    BufWriter,
    BufReader,
    Write,
    Read,
    self,
};

use crate::request::Request;
use crate::response::Response;
use crate::error::{
    Error,
    Result,
};


#[derive(Default)]
pub struct HttpClient {
    pub response: Response,
    pub request: Request,
    stream: Option<TcpStream>,
}


enum HttpStream {
    None,
    Read(BufReader<TcpStream>),
    Write(BufWriter<TcpStream>),
}


impl Write for HttpClient {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut self.stream {
            Some(v) => v.write(buf),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "socket not ready")),
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.flush()
    }
}


impl Read for HttpClient {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match &mut self.stream {
            Some(v) => v.read(buf),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "socket not ready")),
        }
    }
}


impl HttpClient {
    pub fn new() -> Self {
        HttpClient::default()
    }

    pub fn send(&mut self) -> Result<()> {
        let host = self.request.url.get_host();
        let port = match self.request.url.get_port() {
            0 => {
                match self.request.url.get_scheme() {
                    "http" => 80,
                    "https" => 443,
                    _ => return Err(Error::Custom("HttpClient: port not defined for unknown scheme")),
                }
            } 
            v => v,
        };
        let stream = TcpStream::connect((host, port))?;
        self.stream = Some(stream);
        match &mut self.stream {
            Some(v) => {
                let mut writer = BufWriter::new(v);
                self.request.send(&mut writer)?;
                Ok(())
            }
            _ => return Err(Error::Custom("socket not ready")),
        }
    }

    pub fn receive(&mut self) -> Result<()> {
        match &mut self.stream {
            Some(v) => {
                self.response.parse(v)?;
                Ok(())
            }
            _ => return Err(Error::Custom("socket not ready")),
        }
    } 
}