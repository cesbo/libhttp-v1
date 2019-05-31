use std::io::{
    self,
    BufRead,
    Write,
};

use crate::{
    Header,
    Url,
    UrlError,
};


#[derive(Debug, Error)]
pub enum RequestError {
    #[error_from("Request IO: {}", 0)]
    Io(io::Error),
    #[error_from("Request: {}", 0)]
    Url(UrlError),
    #[error_kind("Request: unexpected eof")]
    UnexpectedEof,
    #[error_kind("Request: invalid format")]
    InvalidFormat,
}


pub type Result<T> = std::result::Result<T, RequestError>;


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
    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        let mut first_line = true;
        let mut buffer = String::new();

        self.header.clear();
        self.method.clear();
        self.version.clear();

        loop {
            buffer.clear();
            let r = reader.read_line(&mut buffer)?;

            let s = buffer.trim();
            if s.is_empty() {
                if first_line || r == 0 { return Err(RequestError::UnexpectedEof) }
                break;
            }

            if first_line {
                first_line = false;

                let skip = s.find(char::is_whitespace).ok_or_else(|| RequestError::InvalidFormat)?;
                self.method.push_str(&s[.. skip]);
                let s = s[skip + 1 ..].trim_start();
                let skip = s.find(char::is_whitespace).unwrap_or_else(|| s.len());
                self.url.set(&s[.. skip])?;

                if s.len() > skip {
                    let s = s[skip + 1 ..].trim_start();
                    if ! s.is_empty() {
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
        writeln!(dst,"{} {} {}\r",
            self.method,
            self.url.as_request_uri(),
            self.version)?;

        self.header.send(dst)?;

        writeln!(dst, "\r")
    }

    /// Writes request line and headers to dst
    #[inline]
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<()> {
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
