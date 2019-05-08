use std::io::Write;

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
    pub stream: HttpStream,
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
