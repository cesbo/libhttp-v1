// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use std::{
    io::{
        self,
        BufRead,
        Read,
        Write,
    },
};

use crate::{
    HttpVersion,
    Request,
    RequestError,
    Response,
    ResponseError,
    UrlError,
    UrlSetter,
};

mod auth;
use self::auth::http_auth;

pub (crate) mod transfer;
use self::transfer::{
    HttpTransfer,
    HttpTransferError,
};


#[derive(Debug, Error)]
#[error_prefix = "HttpClient"]
pub enum HttpClientError {
    #[error_from]
    Io(io::Error),
    #[error_from]
    Request(RequestError),
    #[error_from]
    Response(ResponseError),
    #[error_from]
    HttpTransfer(HttpTransferError),
    #[error_from]
    Url(UrlError),
    #[error_kind("invalid protocol")]
    InvalidProtocol,
    #[error_kind("redirect location not defined")]
    InvalidRedirectLocation,
    #[error_kind("request failed: {} {}", 0, 1)]
    RequestFailed(usize, String),
}


pub type Result<T> = std::result::Result<T, HttpClientError>;


const USER_AGENT: &str = concat!("libhttp/", env!("CARGO_PKG_VERSION"));


/// HTTP client
///
/// Basic Usage:
///
/// ```
/// use std::io::Read;
/// use http::HttpClient;
///
/// let mut client = HttpClient::new("https://example.com").unwrap();
/// client.get().unwrap();
/// let mut body = String::new();
/// client.read_to_string(&mut body).unwrap();
/// ```
///
/// Usage with request body:
///
/// ```
/// use std::io::{
///     Read,
///     Write,
/// };
/// use http::HttpClient;
///
/// let mut client = HttpClient::new("http://httpbin.org/post").unwrap();
/// client.request.set_method("POST");
/// client.request.header.set("Accept", "application/json");
/// // Send request
/// client.send().unwrap();
/// client.write_all(b"{\"data\": \"Hello, world!\"}").unwrap();
/// // Get response
/// client.receive().unwrap();
/// let mut body = String::new();
/// client.read_to_string(&mut body).unwrap();
/// ```
#[derive(Default, Debug)]
pub struct HttpClient {
    /// HTTP request
    pub request: Request,
    /// received HTTP response
    pub response: Response,
    /// HTTP stream
    transfer: HttpTransfer,
}


impl HttpClient {
    /// Allocates new http client and prepares HTTP request
    pub fn new<R: UrlSetter>(url: R) -> Result<Self> {
        let mut client = HttpClient::default();
        client.init(url)?;
        Ok(client)
    }

    /// Prepares HTTP request
    pub fn init<R: UrlSetter>(&mut self, url: R) -> Result<()> {
        self.request.url.set(url)?;
        self.request.header.clear();
        self.request.header.set("Host", self.request.url.as_address());
        self.request.header.set("User-Agent", USER_AGENT);
        Ok(())
    }

    /// Close connection
    /// Method should not used manually
    #[inline]
    pub fn close(&mut self) { self.transfer.close() }

    /// Connects to destination host, sends request line and headers
    /// Prepares HTTP stream for writing data
    pub fn send(&mut self) -> Result<()> {
        let mut tls = false;
        let mut port = self.request.url.get_port();

        match self.request.url.get_scheme() {
            "http" => {
                if port == 0 {
                    port = 80;
                }
            },
            "https" => {
                if port == 0 {
                    port = 443;
                }
                tls = true;
            }
            "rtsp" => {
                if port == 0 {
                    port = 554;
                }
                self.request.set_version(HttpVersion::RTSP10);
            }
            _ => return Err(HttpClientError::InvalidProtocol)
        };

        let host = self.request.url.get_host();

        self.transfer.connect(tls, host, port)?;
        self.request.send(&mut self.transfer)?;
        self.transfer.flush()?;

        Ok(())
    }

    /// Flushes writing buffer, receives response line and headers
    /// Prepares HTTP stream for reading data
    pub fn receive(&mut self) -> Result<()> {
        self.transfer.flush()?;
        self.response.parse(&mut self.transfer)?;

        let no_content = {
            let code = self.response.get_code();
            code < 200 ||
            code == 204 ||
            code == 304 ||
            self.request.get_method() == "HEAD"
        };

        if let Some(connection) = self.response.header.get("connection") {
            if connection.eq_ignore_ascii_case("keep-alive") {
                self.transfer.set_connection_keep_alive();
            } else {
                self.transfer.set_connection_close();
            }
        } else if self.response.get_version() == HttpVersion::HTTP10 {
            self.transfer.set_connection_close();
        } else {
            self.transfer.set_connection_keep_alive();
        }

        if no_content {
            self.transfer.set_content_length(0);
            return Ok(());
        }

        if let Some(_encoding) = self.response.header.get("content-encoding") {
            // TODO:
        }

        if let Some(len) = self.response.header.get("content-length") {
            let len = len.parse().unwrap_or(0);
            self.transfer.set_content_length(len);
            return Ok(());
        }

        if let Some(encoding) = self.response.header.get("transfer-encoding") {
            for i in encoding.split(',').map(|v| v.trim()) {
                if i.eq_ignore_ascii_case("chunked") {
                    self.transfer.set_content_chunked();
                    return Ok(());
                }
            }
        }

        self.transfer.set_content_persist();

        Ok(())
    }

    /// Reads response body from receiving buffer and stream
    #[inline]
    pub fn skip_body(&mut self) -> Result<()> {
        io::copy(self, &mut io::sink())?;
        Ok(())
    }

    /// Prepares for HTTP redirect to given location
    pub fn redirect(&mut self) -> Result<()> {
        self.skip_body()?;

        let location = self.response.header.get("location").unwrap_or("");
        if location.is_empty() {
            return Err(HttpClientError::InvalidRedirectLocation)
        }

        if ! self.transfer.is_closed() {
            let limit = std::cmp::min(location.len(), 8); // 8 - enough for "https://"
            if location[.. limit].find("://").is_some() {
                self.transfer.close();
            }
        }

        self.request.url.set(location)?;

        Ok(())
    }

    /// Simple GET request with authentication and location forwarding
    ///
    /// Usage:
    ///
    /// ```
    /// use std::io::Read;
    /// use http::HttpClient;
    ///
    /// let mut client = HttpClient::new("https://example.com").unwrap();
    /// client.get().unwrap();
    /// let mut body = String::new();
    /// client.read_to_string(&mut body).unwrap();
    /// ```
    pub fn get(&mut self) -> Result<()> {
        let mut attempt_auth = 0;
        let mut attempt_redirect = 0;

        loop {
            http_auth(&mut self.request, &self.response);
            self.send()?;
            self.receive()?;

            match self.response.get_code() {
                200 | 204 => break,
                401 if attempt_auth < 2 => {
                    self.skip_body()?;
                    // TODO: check url prefix
                    attempt_auth += 1;
                }
                301 | 302 if attempt_redirect < 3 => {
                    self.redirect()?;
                    attempt_redirect += 1;
                    attempt_auth = 0;
                }
                code => {
                    self.skip_body()?;
                    return Err(HttpClientError::RequestFailed(
                        code, self.response.get_reason().to_owned()));
                }
            }
        }

        Ok(())
    }
}


impl Read for HttpClient {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.transfer.read(buf) }
}


impl BufRead for HttpClient {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> { self.transfer.fill_buf() }

    #[inline]
    fn consume(&mut self, amt: usize) { self.transfer.consume(amt) }
}


impl Write for HttpClient {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.transfer.write(buf) }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { self.transfer.flush() }
}
