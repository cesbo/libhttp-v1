use std::net::TcpStream;
use std::io::BufWriter;

use crate::request::Request;
use crate::response::Response;
use crate::error::Result;


pub struct HttpClient {
    response: Response,
    request: Request,
    stream: BufWriter<TcpStream>,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            response: Response::new(),
            request: Request::new(),
            stream: BufWriter::new(TcpStream::new()),
        }
    }
    pub fn connect(&self) -> Result<()> {
//      let mut stream = TcpStream::connect(&self.url.get_name())?;
        Ok(())
    }
}
