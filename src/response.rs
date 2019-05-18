use std::collections::HashMap;
use std::io::{
    BufRead,
    Write
};

use crate::header;
use crate::error::{
    Error,
    Result,
};


#[derive(Debug)]
pub struct Response {
    version: String,
    code: usize,
    reason: String,
    headers: HashMap<String, String>,
}


impl Default for Response {
    fn default() -> Response {
        Response {
            version: "HTTP/1.1".to_string(),
            code: 0,
            reason: String::new(),
            headers: HashMap::new(),
        }
    }
}


impl Response {
    #[inline]
    pub fn new() -> Self { Response::default() }

    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        let mut line = 0;
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
                let skip = s.find(char::is_whitespace).unwrap_or_else(|| s.len());
                self.version = (&s[.. skip]).to_string();
                let s = s[skip ..].trim_start();
                let skip = s.find(char::is_whitespace).unwrap_or_else(|| s.len());
                self.code = s[.. skip].parse::<usize>().unwrap_or(0);
                self.reason = s[skip ..].trim().to_string();
            } else {
                header::parse(&mut self.headers, &s);
            }
            line += 1;
        }
        Ok(())
    }

    pub fn send<W: Write>(&self, dst: &mut W) -> Result<()> {
        writeln!(dst, "{} {} {}\r", self.version, self.code, self.reason)?;
        for (param, value) in self.headers.iter() {
            header::write_key(param, dst)?;
            writeln!(dst, ": {}\r", value)?;
        }
        writeln!(dst, "\r")?;
        Ok(())
    }

    #[inline]
    pub fn set_version(&mut self, version: &str) {
        self.version.clear();
        self.version.push_str(version);
    }

    #[inline]
    pub fn set_reason(&mut self, version: &str) {
        self.reason.push_str(version);
    }

    #[inline]
    pub fn set_code(&mut self, code: usize) {
        self.code = code;
    }

    #[inline]
    pub fn set<S>(&mut self, key: S, value: S)
    where
        S: Into<String>
    {
        self.headers.insert(key.into().to_lowercase(), value.into());
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
    pub fn get_code(&self) -> usize {
        self.code as usize
    }

    #[inline]
    pub fn get_reason(&self) -> &str {
        self.reason.as_str()
    }
}
