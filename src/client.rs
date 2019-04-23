use std::net::TcpStream;
//use std::io::BufWriter;

use crate::request::Request;
use crate::response::Response;
use crate::error::Result;

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
                    _ => 0,
                }
            }
            _ => self.request.url.get_port(),
        };
        self.stream = Some(TcpStream::connect((host, port))?);
        if let Some(v) = &mut self.stream {
            self.request.send(v).unwrap();
            self.response.parse(v).unwrap(); 
        }
        Ok(())
    }
}
