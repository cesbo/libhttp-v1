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
            let r = reader.read_line(&mut buffer)?;

            let s = buffer.trim();
            if s.is_empty() {
                if first_line || r == 0 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "invalid response"));
                }
                break;
            }

            if first_line {
                first_line = false;

                self.version.clear();
                self.code = 0;
                self.reason.clear();

                let skip = match s.find(char::is_whitespace) {
                    Some(v) => v,
                    None => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid response format")),
                };
                self.version.push_str(&s[.. skip]);
                let s = s[skip + 1 ..].trim_start();
                let skip = s.find(char::is_whitespace).unwrap_or_else(|| s.len());
                self.code = s[.. skip].parse().unwrap_or(0);
                if self.code < 100 || self.code >= 600 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid response code"));
                }

                if s.len() > skip {
                    let s = s[skip + 1 ..].trim_start();
                    if ! s.is_empty() {
                        self.reason.push_str(s);
                    }
                }
            } else {
                if let Some(skip) = s.find(':') {
                    let key = s[.. skip].trim_end();
                    if ! key.is_empty() {
                        let value = s[skip + 1 ..].trim_start();
                        self.headers.insert(key.to_lowercase(), value.to_string());
                    }
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
    pub fn get_code(&self) -> usize {
        self.code
    }

    /// Returns response reason
    #[inline]
    pub fn get_reason(&self) -> &str {
        self.reason.as_str()
    }
}
