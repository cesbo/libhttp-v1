use std::net::TcpStream;
use std::mem;
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
    stream: HttpStream,
}
 

enum HttpStream {
    None,
    Read(BufReader<TcpStream>),
    Write(BufWriter<TcpStream>),
}


impl Default for HttpStream {
    #[inline]
    fn default() -> HttpStream {
        HttpStream::None
    }
}


impl HttpStream {
    #[inline]
    fn take(&mut self) -> HttpStream {
        mem::replace(self, HttpStream::None)
    }
}


impl Write for HttpClient {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut self.stream {
            HttpStream::Write(v) => v.write(buf),
            _ => Err(io::Error::new(io::ErrorKind::Other, "socket not ready")),
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match &mut self.stream {
            HttpStream::Write(v) => v.flush(),
            _ => Err(io::Error::new(io::ErrorKind::Other, "socket not ready")),
        }
    }
}


impl Read for HttpClient {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match &mut self.stream {
            HttpStream::Read(v) => v.read(buf),
            _ => Err(io::Error::new(io::ErrorKind::Other, "socket not ready")),
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
        let mut writer = BufWriter::new(stream);
        self.request.send(&mut writer)?; 
        self.stream = HttpStream::Write(writer);
        Ok(())
    }

    pub fn receive(&mut self) -> Result<()> {
        let writer = match self.stream.take() {
            HttpStream::Write(v) => v,
            _ => return Err(Error::Custom("HttpClient::receive() failed. wrong socket state")),
        };
        let stream = match writer.into_inner() {
            Ok(v) => v,
            _ => return Err(Error::Custom("HttpClient::receive() failed. wrong socket state")),
        };
        let mut reader = BufReader::new(stream);
        self.response.parse(&mut reader)?;
        self.stream = HttpStream::Read(reader);
        Ok(())
    } 
}