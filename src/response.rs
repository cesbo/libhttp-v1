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
    version: String,
    code: usize,
}


impl Response {
    pub fn new() -> Self {
        Response {
            version: String::new(),
            code: 0,
        }
    }
    pub fn read<R: Read>(&mut self, head: R) -> Result<()> {
        let mut line = 0;
        let mut reader = BufReader::new(head);
        let mut buffer = String::new();
        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(v) => if v == 0 { break },
                Err(e) => return Err(Error::from(e)),
            };
            if line == 0 {
                let mut v = buffer.split(' ');
                self.version += v.next().unwrap_or("");
                self.code = (v.next().unwrap_or("")).parse::<usize>().unwrap_or(0);;
            }
            /*if line > 0 {
                header::pars_heades_line(&mut self.headers, &buffer);
            }*/
            line += 1;
        }
        Ok(())
    }
    
    pub fn get_version(&self) -> &str {
        self.version.as_str()
    }
    
    pub fn get_code(&self) -> &usize {
        &(self.code)
    }
}
