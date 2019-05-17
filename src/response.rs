use std::collections::HashMap;
use std::io::{
    self,
    BufRead,
    Write
};

use failure::{
    ensure,
    format_err,
    Error,
    Fail,
};

use crate::tools;


#[derive(Debug, Fail)]
#[fail(display = "Response Error: {}", 0)]
struct ResponseError(Error);


impl From<Error> for ResponseError {
    #[inline]
    fn from(e: Error) -> ResponseError { ResponseError(e) }
}


impl From<io::Error> for ResponseError {
    #[inline]
    fn from(e: io::Error) -> ResponseError { ResponseError(e.into()) }
}


impl From<&str> for ResponseError {
    #[inline]
    fn from(e: &str) -> ResponseError { ResponseError(format_err!("{}", e)) }
}


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
    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> Result<(), Error> {
        let mut first_line = true;
        let mut buffer = String::new();
        loop {
            buffer.clear();
            let r = reader.read_line(&mut buffer).map_err(ResponseError::from)?;

            let s = buffer.trim();
            if s.is_empty() {
                ensure!(!first_line && r != 0, ResponseError::from("unexpected eof"));
                break;
            }

            if first_line {
                first_line = false;

                self.version.clear();
                self.code = 0;
                self.reason.clear();

                let skip = s.find(char::is_whitespace).ok_or_else(|| ResponseError::from("invalid format"))?;
                self.version.push_str(&s[.. skip]);
                let s = s[skip + 1 ..].trim_start();
                let skip = s.find(char::is_whitespace).unwrap_or_else(|| s.len());
                self.code = s[.. skip].parse().unwrap_or(0);
                ensure!(self.code >= 100 && self.code < 600, ResponseError::from("invalid status code"));

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

    fn io_send<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        writeln!(dst, "{} {} {}\r",
            self.version,
            self.code,
            self.reason)?;

        for (key, value) in self.headers.iter() {
            tools::header_write(dst, key, value)?;
        }

        writeln!(dst, "\r")
    }

    /// Writes response line and headers to dst
    #[inline]
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<(), Error> {
        self.io_send(dst).map_err(ResponseError::from)?;
        Ok(())
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
