use std::collections::HashMap;
use std::io::{
    self,
    BufRead,
    Write
};

use crate::tools;
use crate::url::Url;


#[derive(Debug)]
pub struct Request {
    method: String,
    pub url: Url,
    version: String,
    headers: HashMap<String, String>,
}


impl Default for Request {
    fn default() -> Request {
        Request {
            method: String::new(),
            url: Url::new(""),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
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

    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> io::Result<()> {
        let mut first_line = true;
        let mut buffer = String::new();
        loop {
            buffer.clear();
            if reader.read_line(&mut buffer)? == 0 { break }
            let s = buffer.trim();
            if s.is_empty() { break }
            if first_line {
                for (step, part) in s.split_whitespace().enumerate() {
                    match step {
                        0 => self.method = part.to_string(),
                        1 => self.url = Url::new(part),
                        2 => self.version = part.to_string(),
                        _ => break,
                     }
                }
                first_line = false;
            } else {
                if let Some(flag) = s.find(':') {
                    self.headers.insert(
                        s[.. flag].trim_end().to_lowercase(),
                        s[flag + 1 ..].trim_start().to_string()
                    );
                }
            }
        }
        Ok(())
    }

    pub fn send<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        let path = self.url.get_path();
        let path = if path.is_empty() { "/" } else { path };

        writeln!(dst,"{} {}{} {}\r",
            self.method,
            path,
            self.url.get_query(),
            self.version)?;

        write!(dst, "Host: {}", self.url.get_host())?;
        let port = self.url.get_port();
        if port != 0 {
            write!(dst, ":{}", port)?;
        }
        writeln!(dst, "\r")?;

        for (key, value) in self.headers.iter() {
            tools::header_write(dst, key, value)?;
        }

        writeln!(dst, "\r")
    }

    #[inline]
    pub fn set_header<R, S>(&mut self, key: R, value: S)
    where
        R: AsRef<str>,
        S: ToString,
    {
        self.headers.insert(key.as_ref().to_lowercase(), value.to_string());
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
