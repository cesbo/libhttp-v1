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
use crate::url::Url;


#[derive(Debug, Fail)]
#[fail(display = "Request: {}", 0)]
struct RequestError(Error);


impl From<Error> for RequestError {
    #[inline]
    fn from(e: Error) -> RequestError { RequestError(e) }
}


impl From<io::Error> for RequestError {
    #[inline]
    fn from(e: io::Error) -> RequestError { RequestError(e.into()) }
}


impl From<&str> for RequestError {
    #[inline]
    fn from(e: &str) -> RequestError { RequestError(format_err!("{}", e)) }
}


/// Parser and formatter for HTTP request line and headers
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
            method: "GET".to_owned(),
            url: Url::default(),
            version: "HTTP/1.1".to_owned(),
            headers: HashMap::new(),
        }
    }
}


impl Request {
    /// Allocates new request object
    #[inline]
    pub fn new() -> Self { Request::default() }

    /// Sets request url
    pub fn init(&mut self, url: &str) -> Result<(), Error> {
        self.url.set(url).map_err(RequestError::from)?;
        Ok(())
    }

    /// Reads and parses request line and headers
    /// Reads until empty line found
    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> Result<(), Error> {
        let mut first_line = true;
        let mut buffer = String::new();
        loop {
            buffer.clear();
            let r = reader.read_line(&mut buffer).map_err(RequestError::from)?;

            let s = buffer.trim();
            if s.is_empty() {
                ensure!(!first_line && r != 0, RequestError::from("unexpected eof"));
                break;
            }

            if first_line {
                first_line = false;

                self.method.clear();

                let skip = s.find(char::is_whitespace).ok_or_else(|| RequestError::from("invalid format"))?;
                self.method.push_str(&s[.. skip]);
                let s = s[skip + 1 ..].trim_start();
                let skip = s.find(char::is_whitespace).unwrap_or_else(|| s.len());
                self.url.set(&s[.. skip]).map_err(RequestError::from)?;

                if s.len() > skip {
                    let s = s[skip + 1 ..].trim_start();
                    if ! s.is_empty() {
                        self.version.clear();
                        self.version.push_str(s);
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
        let path = self.url.get_path();
        let path = if path.is_empty() { "/" } else { path };

        writeln!(dst,"{} {}{} {}\r",
            self.method,
            path,
            self.url.get_query(),
            self.version)?;

        writeln!(dst, "Host: {}\r", self.url.get_address())?;

        for (key, value) in self.headers.iter() {
            tools::header_write(dst, key, value)?;
        }

        writeln!(dst, "\r")
    }

    /// Writes request line and headers to dst
    #[inline]
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<(), Error> {
        self.io_send(dst).map_err(RequestError::from)?;
        Ok(())
    }

    /// Sets request header
    /// key should be in lowercase
    #[inline]
    pub fn set_header<R, S>(&mut self, key: R, value: S)
    where
        R: AsRef<str>,
        S: ToString,
    {
        self.headers.insert(key.as_ref().to_lowercase(), value.to_string());
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

    /// Returns reference to the request header value value corresponding to the key
    /// key should be in lowercase
    #[inline]
    pub fn get_header(&self, key: &str) -> Option<&String> { self.headers.get(key) }
}
