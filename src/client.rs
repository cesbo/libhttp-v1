use std::io::{
    self,
    BufRead,
    Read,
    Write,
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
    stream: HttpStream,
}


impl HttpClient {
    #[inline]
    pub fn new() -> Self { HttpClient::default() }

    pub fn send(&mut self) -> Result<()> {
        let mut tls = false;
        let host = self.request.url.get_host();
        let mut port = self.request.url.get_port();

        match self.request.url.get_scheme() {
            "http" => {
                if port == 0 {
                    port = 80;
                }
            },
            "https" => {
                if port == 0 {
                    port = 443;
                }
                tls = true;
            }
            _ => return Err(Error::Custom("HttpClient: unknown scheme")),
        };

        self.stream.connect(tls, host, port)?;
        self.request.send(&mut self.stream)?;
        self.stream.flush()?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<()> {
        self.stream.flush()?;
        self.response.parse(&mut self.stream)?;
        self.stream.configure(&self.response)?;
        Ok(())
    }
}


impl Read for HttpClient {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}


impl BufRead for HttpClient {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.stream.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.stream.consume(amt)
    }
}


impl Write for HttpClient {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}
