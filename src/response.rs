// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use std::io::{
    self,
    BufRead,
    Write,
};

use crate::{
    Header,
    HttpVersion,
};


#[derive(Debug, Error)]
#[error_prefix = "Response"]
pub enum ResponseError {
    #[error_from]
    Io(io::Error),
    #[error_kind("invalid format")]
    InvalidFormat,
    #[error_kind("invalid status code")]
    InvalidStatus,
}


pub type Result<T> = std::result::Result<T, ResponseError>;


/// Parser and formatter for HTTP response line and headers
#[derive(Debug)]
pub struct Response {
    version: HttpVersion,
    code: usize,
    reason: String,
    ///
    pub header: Header,
}


impl Default for Response {
    fn default() -> Response {
        Response {
            version: HttpVersion::default(),
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
    pub fn parse<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        let mut first_line = true;
        let mut buffer = String::with_capacity(256);

        self.header.clear();
        self.code = 0;
        self.reason.clear();

        loop {
            buffer.clear();
            reader.read_line(&mut buffer)?;

            let s = buffer.trim();
            if s.is_empty() {
                break;
            }

            if first_line {
                first_line = false;

                let skip = s.find(char::is_whitespace).ok_or_else(|| ResponseError::InvalidFormat)?;
                self.version = s[.. skip].into();
                let s = s[skip + 1 ..].trim_start();
                let skip = s.find(char::is_whitespace).unwrap_or_else(|| s.len());
                self.code = s[.. skip].parse().unwrap_or(0);
                if self.code < 100 || self.code >= 600 { return Err(ResponseError::InvalidStatus) }

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
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<()> {
        self.io_send(dst)?;
        Ok(())
    }

    /// Sets protocol version
    /// Default: `HTTP/1.1`
    #[inline]
    pub fn set_version(&mut self, version: HttpVersion) { self.version = version }

    /// Sets response status code
    #[inline]
    pub fn set_code(&mut self, code: usize) { self.code = code }

    /// Sets response reason
    #[inline]
    pub fn set_reason(&mut self, version: &str) { self.reason.push_str(version) }

    /// Returns response version
    #[inline]
    pub fn get_version(&self) -> HttpVersion { self.version }

    /// Returns response status code
    #[inline]
    pub fn get_code(&self) -> usize { self.code }

    /// Returns response reason
    #[inline]
    pub fn get_reason(&self) -> &str { self.reason.as_str() }
}
