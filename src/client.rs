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
        println!(" ------------ Start  ------------- ");
        self.stream = Some(TcpStream::connect(&self.request.url.get_name())?);
        if let Some(v) = &mut self.stream {
            println!(" ------------ Some is in  ------------- ");
            self.request.send(v).unwrap();
            self.response.parse(v).unwrap();
            println!("{:#?}", self.response); 
        } else {
            println!(" ------------ Fail: None! ------------- ");
        }
        Ok(())
    }
}
