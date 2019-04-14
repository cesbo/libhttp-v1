use std::collections::HashMap;
use std::io::{ 
    Read, 
    BufRead, 
    BufReader, 
    Write
};

use crate::header;
use crate::error::{
    Error,
    Result,
};


#[derive(Default)]
pub struct Response {
    version: String,
    code: usize,
    reason: String,
    headers: HashMap<String, String>,
}


impl Response {
    pub fn new() -> Self {
        Response {
            version: String::new(),
            code: 0,
            reason: String::new(),
            headers: HashMap::new(),
        }
    }
    
    pub fn parse<R: Read>(&mut self, head: R) -> Result<()> {
        let mut line = 0;
        let mut reader = BufReader::new(head);
        let mut buffer = String::new();
        loop {
            buffer.clear();
            if let Err(e) = reader.read_line(&mut buffer) {
                return Err(Error::from(e));
            }
            let s = buffer.trim();
            if s.is_empty() {
                break;
            }
            if line == 0 {
                let mut step: usize = 0;
                for part in s.split_whitespace() {
                    match step {
                        0 => self.version += part,
                        1 => self.code = part.parse::<usize>().unwrap_or(0),
                        2 => self.reason += &part.trim(),
                        _ => break,
                    };
                    step += 1;
                }
            } else {
                header::pars_heades_line(&mut self.headers, &s);
            }
            line += 1;
        }
        Ok(())
    }
 
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<()> {
        writeln!(dst, "{} {} {}\r", self.version, self.code, self.reason)?;
        for (param, value) in self.headers.iter() {
            writeln!(dst, "{}: {}\r", header::headers_case(param), value)?;
        } 
        writeln!(dst, "\r")?;
        Ok(())
    }
    
    #[inline]
    pub fn set_version(&mut self, version: &str)
    {
        self.version.clear();
        self.version.push_str(version);
    }  
      
    #[inline]
    pub fn set_reason(&mut self, version: &str)
    {
        self.reason.push_str(version);
    }
              
    #[inline]
    pub fn set_code(&mut self, code: &usize)
    {
        self.code = *code;
    }
    
    #[inline]
    pub fn set<S>(&mut self, header_name: S, header_data: S)
    where
        S: Into<String> 
    {
        self.headers.insert(header_name.into().to_lowercase(), header_data.into());
    }
    
    #[inline]    
    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get(header)
    }
    
    #[inline]
    pub fn get_version(&self) -> &str {
        self.version.as_str()
    }
    
    #[inline] 
    pub fn get_code(&self) -> &usize {
        &(self.code)
    }
    
    #[inline] 
    pub fn get_reason(&self) -> &str {
        self.reason.as_str()
    }
}
