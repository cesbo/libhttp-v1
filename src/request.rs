use std::collections::HashMap;
use std::io::{
    Read,
    BufRead,
    BufReader, 
    Write
};

use crate::url::Url;
use crate::header;
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

/*
impl Read for Request {
    fn read(&mut self, data: &mut [u8]) -> Result<usize, ReadError> {
        Ok(0)
    }
}
*/

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
    
    pub fn parse<R: Read>(&mut self, head: R) -> Result<()> {
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
                for (step, part) in buffer.split_whitespace().enumerate() {
                    match step {
                        0 => self.method += part,
                        1 => self.url =  Url::new(part),
                        2 => self.version = part.to_string(),
                        _ => break,
                     }
                }
            } else {
                header::parse(&mut self.headers, &buffer);
            }
            line += 1;
        }
        Ok(())
    }
    
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<()> {
        write!(dst,"{} {}{}", self.method, &self.url.get_path(), &self.url.get_query())?;
        writeln!(dst, "{} {}\r", &self.url.get_fragment(), self.version)?;
        writeln!(dst, "Host: {}\r", &self.url.get_name())?;
        for (param, value) in self.headers.iter() {
            header::write_key(param, dst)?;
            writeln!(dst, ": {}\r", value)?;
        } 
        writeln!(dst, "\r")?;
        Ok(())
    }

    #[inline]
    pub fn set<R, S>(&mut self, name: R, data: S)
    where
        R: AsRef<str>,
        S: Into<String>
    {
        self.headers.insert(name.as_ref().to_lowercase(), data.into());
    }
    
    #[inline]
    pub fn set_version(&mut self, version: &str)
    {
        self.version.clear();
        self.version.push_str(version);
    }
 
    #[inline]
    pub fn get_method(&self) -> &str {
        self.method.as_str()
    } 
    
    #[inline]
    pub fn get_version(&self) -> &str {
        self.version.as_str()
    } 
    
    #[inline]
    pub fn get_path(&self) -> &str {
        self.url.get_path()
    }
    
    #[inline]
    pub fn get_query(&self) -> &str {
        self.url.get_query()
    }
    
    #[inline]    
    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get(header)
    }
}

