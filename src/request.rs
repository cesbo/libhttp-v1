use std::collections::HashMap;
use std::io::{
    self,
    BufRead,
    Write
};

use crate::tools;
use crate::url::Url;


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
            method: String::new(),
            url: Url::default(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        }
    }
}


impl Request {
    /// Allocates new request object
    #[inline]
    pub fn new() -> Self { Request::default() }

    /// Sets request method and url
    /// method should be in uppercase
    pub fn init<S>(&mut self, method: S, url: &str)
    where
        S: Into<String>,
    {
        self.method = method.into();
        self.url.set(url);
    }

    /// Reads and parses request line and headers
    /// Reads until empty line found
    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> io::Result<()> {
        let mut first_line = true;
        let mut buffer = String::new();
        loop {
            buffer.clear();
            reader.read_line(&mut buffer)?;

            let s = buffer.trim();
            if s.is_empty() {
                if first_line {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "invalid request"));
                }
                break;
            }

            if first_line {
                first_line = false;

                self.method.clear();
                self.version.clear();

                for (step, part) in s.split_whitespace().enumerate() {
                    match step {
                        0 => self.method.push_str(part),
                        1 => self.url.set(part),
                        2 => self.version.push_str(part),
                        _ => break,
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

    /// Writes request line and headers to dst
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

    /// Sets protocol version
    /// Default: `HTTP/1.1`
    #[inline]
    pub fn set_version(&mut self, version: &str) {
        self.version.clear();
        self.version.push_str(version);
    }

    /// Returns request method
    #[inline]
    pub fn get_method(&self) -> &str {
        self.method.as_str()
    }

    /// Returns request version
    #[inline]
    pub fn get_version(&self) -> &str {
        self.version.as_str()
    }

    /// Returns reference to the request header value value corresponding to the key
    /// key should be in lowercase
    #[inline]
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}
