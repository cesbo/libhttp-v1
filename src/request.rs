use std::{
    fmt,
    io::{
        self,
        BufRead,
        Write,
    },
};

use crate::{
    header::Header,
    url::{
        Url,
        UrlError,
    },
};


#[derive(Debug)]
pub struct RequestError(String);


impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Request: {}", self.0) }
}


impl From<io::Error> for RequestError {
    fn from(e: io::Error) -> RequestError { RequestError(e.to_string()) }
}


impl From<&str> for RequestError {
    fn from(e: &str) -> RequestError { RequestError(e.to_string()) }
}


impl From<UrlError> for RequestError {
    fn from(e: UrlError) -> RequestError { RequestError(e.to_string()) }
}


/// Parser and formatter for HTTP request line and headers
#[derive(Debug)]
pub struct Request {
    method: String,
    pub url: Url,
    version: String,
    pub header: Header,
    pub (crate) nonce_count: usize
}


impl Default for Request {
    fn default() -> Request {
        Request {
            method: "GET".to_owned(),
            url: Url::default(),
            version: "HTTP/1.1".to_owned(),
            header: Header::default(),
            nonce_count: 0,
        }
    }
}


impl Request {
    /// Allocates new request object
    #[inline]
    pub fn new() -> Self { Request::default() }

    /// Reads and parses request line and headers
    /// Reads until empty line found
    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> Result<(), RequestError> {
        let mut first_line = true;
        let mut buffer = String::new();
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

                self.method.clear();

                let skip = s.find(char::is_whitespace).ok_or_else(|| "invalid format")?;
                self.method.push_str(&s[.. skip]);
                let s = s[skip + 1 ..].trim_start();
                let skip = s.find(char::is_whitespace).unwrap_or_else(|| s.len());
                self.url.set(&s[.. skip])?;

                if s.len() > skip {
                    let s = s[skip + 1 ..].trim_start();
                    if ! s.is_empty() {
                        self.version.clear();
                        self.version.push_str(s);
                    }
                }
            } else {
                self.header.parse(s);
            }
        }

        Ok(())
    }

    fn io_send<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        let path = self.url.get_path();
        let path = if path.is_empty() { "/" } else { path };

        writeln!(dst,"{} {}{} {}\r",
            self.method,
            path,
            self.url.get_query(),
            self.version)?;

        writeln!(dst, "Host: {}\r", self.url.get_address())?;
        self.header.send(dst)?;

        writeln!(dst, "\r")
    }

    /// Writes request line and headers to dst
    #[inline]
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<(), RequestError> {
        self.io_send(dst)?;
        Ok(())
    }

    /// Sets request method
    /// method should be in uppercase
    /// Default: `GET`
    #[inline]
    pub fn set_method(&mut self, method: &str) {
        self.method.clear();
        self.method.push_str(method);
    }

    /// Sets protocol version
    /// Default: `HTTP/1.1`
    #[inline]
    pub fn set_version(&mut self, version: &str) {
        self.version.clear();
        self.version.push_str(version);
    }

    /// Returns request method
    #[inline]
    pub fn get_method(&self) -> &str { self.method.as_str() }

    /// Returns request version
    #[inline]
    pub fn get_version(&self) -> &str { self.version.as_str() }
}
