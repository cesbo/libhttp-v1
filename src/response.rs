use std::io::{ 
    Read, 
    BufRead, 
    BufReader, 
    Write
};


use crate::error::{
    Error,
    Result,
};


pub struct Response {
    method: String,
}


impl Response {
    pub fn new() -> Self {
        Response {
            method: String::new(),
        }
    }
    pub fn read<R: Read>(&mut self, head: R) -> Result<()> {
        Ok(())
    }
}
