use std::net::TcpStream;
use std::io::{
    BufWriter,
    BufReader,
    Write,
    Read,
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
    stream_read: Option<BufReader<TcpStream>>,
    stream_write: Option<BufWriter<TcpStream>>,
}


impl Write for HttpClient {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.stream {
            Some(v) => v.write(buf),
            _ =>  Err(Error::Custom("socket not ready")),
        }
    }
}


impl Read for HttpClient {
    #[inline]
    fn read_to_string(&mut self, buf: &[u8]) -> String {
        let buffer = match self.stream {
            Some(v) => self.response.read(buf),
            _ =>  Err(Error::Custom("socket not ready")),
        };
        buffer.to_string()
    }
}


impl HttpClient {
    pub fn new() -> Self {
        HttpClient::default()
    }

    pub fn connect(&mut self) -> Result<()> {
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
        let mut stream = TcpStream::connect((host, port))?;
        {
            self.stream_write = BufWriter::new(&mut stream);
            self.request.send(&mut self.stream_write)?;
        }
        self.response.parse(&stream)?;
        self.stream = Some(stream);
        Ok(())
    }
}
