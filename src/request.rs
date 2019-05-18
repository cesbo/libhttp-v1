use std::collections::HashMap;
use std::io::{
    BufRead,
    Write
};

use crate::url::Url;
use crate::header;
use crate::error::{
    Error,
    Result,
};


#[derive(Debug)]
pub struct Request {
    method: String,
    pub url: Url,
    version: String,
    headers: HashMap<String, String>,
    pub nonce_count: usize
}


impl Default for Request {
    fn default() -> Request {
        Request {
            method: String::new(),
            url: Url::new(""),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            nonce_count: 0,
        }
    }
}


impl Request {
    #[inline]
    pub fn new() -> Self { Request::default() }

    pub fn init<S>(&mut self, method: S, url: &str)
    where
        S: Into<String>,
    {
        self.method = method.into();
        self.url = Url::new(url);
    }

    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        let mut line = 0;
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
                        1 => self.url = Url::new(part),
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
        write!(dst,"{} ", self.method)?;
        self.url.write_request_url(dst).unwrap();
        writeln!(dst, " {}\r", self.version)?;
        write!(dst, "Host: ")?;
        self.url.write_header_host(dst).unwrap();
        writeln!(dst, "\r")?;
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
    pub fn set_version(&mut self, version: &str) {
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
    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get(header)
    }
}
