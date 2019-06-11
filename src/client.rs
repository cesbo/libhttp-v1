use std::io::{
    self,
    BufRead,
    Read,
    Write,
};

use crate::{
    http_auth,
    Request,
    RequestError,
    Response,
    ResponseError,
    HttpStream,
    HttpStreamError,
    UrlError,
};


#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error_from("HttpClient IO: {}", 0)]
    Io(io::Error),
    #[error_from("HttpClient: {}", 0)]
    Request(RequestError),
    #[error_from("HttpClient: {}", 0)]
    Response(ResponseError),
    #[error_from("HttpClient: {}", 0)]
    HttpStream(HttpStreamError),
    #[error_from("HttpClient: {}", 0)]
    Url(UrlError),
    #[error_kind("HttpClient: invalid protocol")]
    InvalidProtocol,
    #[error_kind("HttpClient: redirect location not defined")]
    InvalidRedirectLocation,
    #[error_kind("HttpClient: request failed: {} {}", 0, 1)]
    RequestFailed(usize, String),
}


pub type Result<T> = std::result::Result<T, HttpClientError>;


pub const USER_AGENT: &str = concat!("libhttp/", env!("CARGO_PKG_VERSION"));


/// HTTP client
///
/// Usage:
///
/// ```
/// use std::io::Read;
/// use http::HttpClient;
///
/// let mut client = HttpClient::new("https://example.com").unwrap();
/// client.send().unwrap();
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
    stream: HttpStream,
}


impl HttpClient {
    /// Allocates new http client and prepares HTTP request
    pub fn new<R: AsRef<str>>(url: R) -> Result<Self> {
        let mut client = HttpClient::default();
        client.init(url)?;
        Ok(client)
    }

    /// Prepares HTTP request
    pub fn init<R: AsRef<str>>(&mut self, url: R) -> Result<()> {
        self.request.url.set(url)?;
        self.request.header.clear();
        self.request.header.set("host", self.request.url.as_address());
        self.request.header.set("user-agent", USER_AGENT);
        Ok(())
    }

    /// Close connection
    /// Method should not used manually
    #[inline]
    pub fn close(&mut self) { self.stream.close() }

    /// Connects to destination host, sends request line and headers
    /// Prepares HTTP stream for writing data
    pub fn send(&mut self) -> Result<()> {
        let mut tls = false;
        let host = self.request.url.get_host();
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
            _ => return Err(HttpClientError::InvalidProtocol)
        };

        self.stream.connect(tls, host, port)?;
        self.request.send(&mut self.stream)?;
        self.stream.flush()?;

        Ok(())
    }

    /// Flushes writing buffer, receives response line and headers
    /// Prepares HTTP stream for reading data
    pub fn receive(&mut self) -> Result<()> {
        self.stream.flush()?;
        self.response.parse(&mut self.stream)?;

        let no_content = {
            let code = self.response.get_code();
            code < 200 ||
            code == 204 ||
            code == 304 ||
            self.request.get_method() == "HEAD"
        };

        self.stream.configure(no_content, &self.response)?;

        Ok(())
    }

    /// Prepares for HTTP redirect to given location
    pub fn redirect(&mut self) -> Result<()> {
        self.stream.skip_body()?;

        let location = self.response.header.get("location").unwrap_or("");
        if location.is_empty() {
            return Err(HttpClientError::InvalidRedirectLocation)
        }

        // TODO: keep-alive
        self.stream.close();
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
                    self.stream.skip_body()?;
                    // TODO: check url prefix
                    attempt_auth += 1;
                }
                301 | 302 if attempt_redirect < 3 => {
                    self.redirect()?;
                    attempt_redirect += 1;
                    attempt_auth = 0;
                }
                code => {
                    self.stream.skip_body()?;
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
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}


impl BufRead for HttpClient {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.stream.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.stream.consume(amt)
    }
}


impl Write for HttpClient {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}
