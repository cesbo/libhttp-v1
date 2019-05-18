use std::io::{
    self,
    BufRead,
    Read,
    Write,
};

use failure::{
    bail,
    Error,
    Fail,
};

use crate::request::Request;
use crate::response::Response;
use crate::stream::HttpStream;


#[derive(Debug, Fail)]
#[fail(display = "HttpClient: {}", 0)]
struct HttpClientError(Error);


impl From<Error> for HttpClientError {
    #[inline]
    fn from(e: Error) -> HttpClientError { HttpClientError(e) }
}


impl From<io::Error> for HttpClientError {
    #[inline]
    fn from(e: io::Error) -> HttpClientError { HttpClientError(e.into()) }
}


impl From<&str> for HttpClientError {
    #[inline]
    fn from(e: &str) -> HttpClientError { HttpClientError(failure::err_msg(e.to_owned())) }
}


/// HTTP client
///
/// Usage:
///
/// ```
/// use std::io::Read;
/// use http::HttpClient;
///
/// fn main() {
///     let mut client = HttpClient::new();
///     client.request.init("https://example.com");
///     client.request.set_header("user-agent", "libhttp");
///     client.send().unwrap();
///     client.receive().unwrap();
///     let mut body = String::new();
///     client.read_to_string(&mut body).unwrap();
/// }
/// ```
#[derive(Default)]
pub struct HttpClient {
    /// HTTP request
    pub request: Request,
    /// received HTTP response
    pub response: Response,
    /// HTTP stream
    stream: HttpStream,
}


impl HttpClient {
    /// Allocates new http client
    #[inline]
    pub fn new() -> Self { HttpClient::default() }

    /// Connects to destination host, sends request line and headers
    /// Prepares HTTP stream for writing data
    pub fn send(&mut self) -> Result<(), Error> {
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
            _ => bail!(HttpClientError::from("invalid protocol")),
        };

        self.stream.connect(tls, host, port).map_err(HttpClientError::from)?;
        self.request.send(&mut self.stream).map_err(HttpClientError::from)?;
        self.stream.flush().map_err(HttpClientError::from)?;

        Ok(())
    }

    /// Flushes writing buffer, receives response line and headers
    /// Prepares HTTP stream for reading data
    pub fn receive(&mut self) -> Result<(), Error> {
        self.stream.flush().map_err(HttpClientError::from)?;
        self.response.parse(&mut self.stream).map_err(HttpClientError::from)?;
        self.stream.configure(&self.response).map_err(HttpClientError::from)?;

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
