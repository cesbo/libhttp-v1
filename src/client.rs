use std::net::TcpStream;
//use std::io::BufWriter;

use crate::request::Request;
use crate::response::Response;
use crate::error::Result;


pub struct HttpClient {
    pub response: Response,
    pub request: Request,
    stream: Option<TcpStream>,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            response: Response::new(),
            request: Request::new(),
            stream: None,
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        self.stream = Some(TcpStream::connect(&self.request.get_path())?); 
        Ok(())
    }
}
