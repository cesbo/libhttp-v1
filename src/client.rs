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
        let connect = format!("{}:{}", &self.request.url.get_host(), &self.request.url.get_port());
        self.stream = Some(TcpStream::connect(connect)?);
        if let Some(v) = &mut self.stream {
            self.request.send(v).unwrap();
            self.response.parse(v).unwrap(); 
        }
        Ok(())
    }
}
