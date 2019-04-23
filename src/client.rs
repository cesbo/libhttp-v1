use std::net::TcpStream;
//use std::io::BufWriter;

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
        self.request.send(&mut stream)?;
        self.response.parse(&stream)?;
        self.stream = Some(stream);
        Ok(())
    }
}
