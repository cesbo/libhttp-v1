use std::collections::HashMap;
use std::io::{
    self,
    BufRead,
    Write
};

use crate::tools;


/// Parser and formatter for HTTP response line and headers
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
    /// Allocates new request object
    #[inline]
    pub fn new() -> Self { Response::default() }

    /// Reads and parses response line and headers
    /// Reads until empty line found
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

    /// Writes response line and headers to dst
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

    /// Sets response header
    /// key should be in lowercase
    #[inline]
    pub fn set_header<R, S>(&mut self, key: R, value: S)
    where
        R: AsRef<str>,
        S: ToString,
    {
        self.headers.insert(key.as_ref().to_lowercase(), value.to_string());
    }

    /// Sets protocol version
    /// Default: `HTTP/1.1`
    #[inline]
    pub fn set_version(&mut self, version: &str) {
        self.version.clear();
        self.version.push_str(version);
    }

    /// Sets response status code
    #[inline]
    pub fn set_code(&mut self, code: usize) {
        self.code = code;
    }

    /// Sets response reason
    #[inline]
    pub fn set_reason(&mut self, version: &str) {
        self.reason.push_str(version);
    }

    /// Returns reference to the response header value value corresponding to the key
    /// key should be in lowercase
    #[inline]
    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get(header)
    }

    /// Returns response version
    #[inline]
    pub fn get_version(&self) -> &str {
        self.version.as_str()
    }

    /// Returns response status code
    #[inline]
    pub fn get_code(&self) -> &usize {
        &(self.code)
    }

    /// Returns response reason
    #[inline]
    pub fn get_reason(&self) -> &str {
        self.reason.as_str()
    }
}
