use std::net::TcpStream;
use std::io::{
    BufWriter,
    BufReader,
    Write,
    Read,
    ErrorKind,
    Error,
    Result,
};

use crate::request::Request;
use crate::response::Response;
use crate::error::{
    Error as CustomError,
    Result as CustomResult,
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
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.stream {
            Some(v) => v.write(buf),
            _ => return Err(Error::new(ErrorKind::Other, "socket not ready")),
            //_ => return Err(io::Error::new(io::ErrorKind::Other, "socket not ready")),
        }
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.flush()
    }
}


impl Read for HttpClient {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.stream {
            Some(v) => v.read(buf),
            _ => return Err(Error::new(ErrorKind::Other, "socket not ready")),
        }
    }
}


impl HttpClient {
    pub fn new() -> Self {
        HttpClient::default()
    }

    pub fn connect(&mut self) -> CustomResult<()> {
        let host = self.request.url.get_host();
        let port = match self.request.url.get_port() {
            0 => {
                match self.request.url.get_scheme() {
                    "http" => 80,
                    "https" => 443,
                    _ => return Err(CustomError::Custom("HttpClient: port not defined for unknown scheme")),
                }
            } 
            v => v,
        };
        let mut stream = TcpStream::connect((host, port))?;
        {
            let mut writer = BufWriter::new(&mut stream);
            self.request.send(&mut writer)?;
        }
        self.response.parse(&stream)?;
        self.stream = Some(stream);
        Ok(())
    }
}
