use std::collections::HashMap;
use std::io::{
    self,
    BufRead,
    Write
};

use crate::tools;


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
                        0 => self.version = part.to_string(),
                        1 => self.code = part.parse().unwrap_or(0),
                        _ => self.reason += part,
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
        writeln!(dst, "{} {} {}\r",
            self.version,
            self.code,
            self.reason)?;

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
    pub fn set_reason(&mut self, version: &str) {
        self.reason.push_str(version);
    }

    #[inline]
    pub fn set_code(&mut self, code: usize) {
        self.code = code;
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
