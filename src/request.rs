use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, BufRead, BufReader, Write, BufWriter};

use crate::url::Url;
use crate::error::{
    Error,
    Result,
};


#[derive(Debug)]
pub struct Request {
    method: String,
    url: Url,
    version: String,
    headers: HashMap<String, String>,
}


impl Request {
    pub fn new() -> Self {
        Request {
            method: String::new(),
            url: Url::new(""),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        }
    }
    
    pub fn init<S>(&mut self, method: S, url: &str) 
    where
        S: Into<String>,
    {
        self.method = method.into();
        self.url =  Url::new(url);
    }
    
    pub fn set<S>(&mut self, header_name: S, header_data: S)
    where
        S: Into<String> 
    {
        self.headers.insert(header_name.into(), header_data.into());
    }
    
    pub fn set_version(&mut self, version: &str)
    {
        self.version.clear();
        self.version.push_str(version);
    }
    
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<()> {
        writeln!(dst, "{} {}{} {}\r", self.method, &self.url.get_path(), &self.url.get_query(), self.version)?;
        writeln!(dst, "Host: {}\r", &self.url.get_name())?;
        for (param, value) in self.headers.iter() {
            writeln!(dst, "{}: {}\r", param, value)?;
        } 
        writeln!(dst, "\r")?;
        Ok(())
    }
 
    pub fn get_method(&self) -> &str {
        self.method.as_str()
    }
    /*
    pub fn headers_get(&self, header: &str) -> &str {
        match self.headers.get(&header) {
            Some(&data) => {
                
            }
            _ => ""
        }
    }*/
    
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
                self.method += v.next().unwrap_or("");
	    }
	    if line > 0 {
	        if buffer.find(": ") == None {
	            continue;
	        }
	        let mut v = buffer.split(": ");
	        let header = v.next().unwrap_or("");
	        let data = v.next().unwrap_or("");
	        self.headers.insert(header.to_string(), data.to_string());
	    }
            line += 1;
        }
        println!("{:#?}", self);
        Ok(())
    }
}

