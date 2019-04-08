use std::collections::HashMap;
use std::io::{Read, BufRead, BufReader, Write};

use crate::url::Url;
use crate::error::{
    Error,
    Result,
};


#[derive(Default)]
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
        self.headers.insert(header_name.into().to_lowercase(), header_data.into());
    }
    
    pub fn set_version(&mut self, version: &str)
    {
        self.version.clear();
        self.version.push_str(version);
    }
    
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<()> {
        write!(dst,"{} {}{}", self.method, &self.url.get_path(), &self.url.get_query())?;
        writeln!(dst, "{} {}\r", &self.url.get_fragment(), self.version)?;
        writeln!(dst, "Host: {}\r", &self.url.get_name())?;
        for (param, value) in self.headers.iter() {
            writeln!(dst, "{}: {}\r", self.headers_case(param), value)?;
        } 
        writeln!(dst, "\r")?;
        Ok(())
    }
    
    fn headers_case(&self, inp: &str) -> String {
        let mut ret = String::new();
        for part in inp.split('-') {
            if ret != "" {
                ret += "-";
            }
            if ! part.is_empty() {
                ret += &part[.. 1].to_uppercase();
                ret += &part[1 ..];
            }
        }
        ret
    }
 
    pub fn get_method(&self) -> &str {
        self.method.as_str()
    }
    
    pub fn get_header(&self, header: &str) -> Option<&str> {
        match self.headers.get(header) {
            Some(data) => Some(data),
            _ => None,
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
                self.method += v.next().unwrap_or("");
            }
            if line > 0 {
                if buffer.find(": ") == None {
                    continue;
                }
                let mut v = buffer.split(": ");
                let header = v.next().unwrap_or("").to_lowercase();
                let data = v.next().unwrap_or("");
                self.headers.insert(header.to_string(), (data[.. (data.len() - 2)]).to_string());
            }
            line += 1;
        }
        Ok(())
    }
}

