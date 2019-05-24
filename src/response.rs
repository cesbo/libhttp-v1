use std::{
    fmt,
    io::{
        self,
        BufRead,
        Write,
    },
};

use crate::header::Header;


#[derive(Debug)]
pub struct ResponseError(String);


impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Response: {}", self.0) }
}


impl From<io::Error> for ResponseError {
    fn from(e: io::Error) -> ResponseError { ResponseError(e.to_string()) }
}


impl From<&str> for ResponseError {
    fn from(e: &str) -> ResponseError { ResponseError(e.to_string()) }
}


/// Parser and formatter for HTTP response line and headers
#[derive(Debug)]
pub struct Response {
    version: String,
    code: usize,
    reason: String,
    ///
    pub header: Header,
}


impl Default for Response {
    fn default() -> Response {
        Response {
            version: "HTTP/1.1".to_string(),
            code: 0,
            reason: String::default(),
            header: Header::default(),
        }
    }
}


impl Response {
    /// Allocates new request object
    #[inline]
    pub fn new() -> Self { Response::default() }

    /// Reads and parses response line and headers
    /// Reads until empty line found
    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> Result<(), ResponseError> {
        let mut first_line = true;
        let mut buffer = String::new();

        self.header.clear();
        self.version.clear();
        self.code = 0;
        self.reason.clear();

        loop {
            buffer.clear();
            let r = reader.read_line(&mut buffer)?;

            let s = buffer.trim();
            if s.is_empty() {
                if first_line || r == 0 { Err("unexpected eof")? }
                break;
            }

            if first_line {
                first_line = false;

                let skip = s.find(char::is_whitespace).ok_or_else(|| "invalid format")?;
                self.version.push_str(&s[.. skip]);
                let s = s[skip + 1 ..].trim_start();
                let skip = s.find(char::is_whitespace).unwrap_or_else(|| s.len());
                self.code = s[.. skip].parse().unwrap_or(0);
                if self.code < 100 || self.code >= 600 { Err("invalid status code")? }

                if s.len() > skip {
                    let s = s[skip + 1 ..].trim_start();
                    if ! s.is_empty() {
                        self.reason.push_str(s);
                    }
                }
            } else {
                self.header.parse(s);
            }
        }

        Ok(())
    }

    fn io_send<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        writeln!(dst, "{} {} {}\r",
            self.version,
            self.code,
            self.reason)?;

        self.header.send(dst)?;

        writeln!(dst, "\r")
    }

    /// Writes response line and headers to dst
    #[inline]
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<(), ResponseError> {
        self.io_send(dst)?;
        Ok(())
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
    pub fn set_code(&mut self, code: usize) { self.code = code }

    /// Sets response reason
    #[inline]
    pub fn set_reason(&mut self, version: &str) { self.reason.push_str(version) }

    /// Returns response version
    #[inline]
    pub fn get_version(&self) -> &str { self.version.as_str() }

    /// Returns response status code
    #[inline]
    pub fn get_code(&self) -> usize { self.code }

    /// Returns response reason
    #[inline]
    pub fn get_reason(&self) -> &str { self.reason.as_str() }
}
