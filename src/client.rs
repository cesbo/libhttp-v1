use std::net::TcpStream;
use std::io::Write;

use crate::auth;
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
        if ! self.stream.is_ready() {
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

            self.stream.set(TcpStream::connect((host, port))?);
        } else {
            self.stream.clear();
        }
                
        if  ! &self.request.url.get_prefix().is_empty() {
            match *self.response.get_code() as i32 {
                401 => {
                    let head = match &self.response.get_header("www-authenticate") {
                        Some(v) => v,
                        _ => "",
                    };
                    if head[.. 6].eq_ignore_ascii_case("digest") {
                        auth::digest(&mut self.response, &mut self.request);
                    }
                }
                _ => auth::basic(&mut self.request),
            }
        }

        self.request.send(&mut self.stream)?;
        self.stream.flush()?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<()> {
        self.stream.flush()?;
        self.response.parse(&mut self.stream)?;

        loop {
            match self.response.get_header("content-length") {
                Some(v) => {
                    self.stream.set_stream_length(v.parse().unwrap_or(0));
                    break;
                },
                _ => {},
            };

            match self.response.get_header("transfer-encoding") {
                Some(v) if v == "chunked" => {
                    self.stream.set_stream_chunked();
                    break;
                },
                _ => {},
            };

            self.stream.set_stream_eof();
            break;
        }

        Ok(())
    }
}
