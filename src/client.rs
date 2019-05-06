use std::net::TcpStream;
use std::io::{
    Write,
    Read,
    self,
};

use crate::request::Request;
use crate::response::Response;
use crate::stream::HttpStream;
use crate::error::{
    Error,
    Result,
};


#[derive(Default)]
pub struct HttpClient {
    pub response: Response,
    pub request: Request,
    stream: Option<HttpStream>,
}


impl Write for HttpClient {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(stream) = &mut self.stream {
            stream.write(buf)
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        if let Some(stream) = &mut self.stream {
            stream.flush()
        } else {
            unreachable!()
        }
    }
}


impl Read for HttpClient {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(stream) = &mut self.stream {
            stream.read(buf)
        } else {
            unreachable!()
        }
    }
}


impl HttpClient {
    pub fn new() -> Self { HttpClient::default() }

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

        let mut stream = HttpStream::new(TcpStream::connect((host, port))?);
        self.request.send(&mut stream)?;
        stream.flush()?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn receive(&mut self) -> Result<()> {
        if let Some(stream) = &mut self.stream {
            stream.flush()?;
            self.response.parse(stream)?;

            stream.set_stream_eof();
            if let Some(v) = self.response.get_header("content-length") {
                stream.set_stream_length(v.parse().unwrap_or(0))
            } else if let Some(v) = self.response.get_header("transfer-encoding") {
                if v == "chunked" { stream.set_stream_chunked() }
            }
        } else {
            unreachable!()
        }

        Ok(())
    }
}
