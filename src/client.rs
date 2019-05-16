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
    ResultExt,
};

use crate::request::Request;
use crate::response::Response;
use crate::stream::HttpStream;


#[derive(Debug, Fail)]
enum HttpClientError {
    #[fail(display = "Http Client Error")]
    Context,
    #[fail(display = "Http Client: invalid protocol")]
    InvalidProtocol,
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
///     client.request.init("GET", "https://example.com");
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
            _ => bail!(HttpClientError::InvalidProtocol),
        };

        self.stream.connect(tls, host, port).context(HttpClientError::Context)?;
        self.request.send(&mut self.stream).context(HttpClientError::Context)?;
        self.stream.flush().context(HttpClientError::Context)?;

        Ok(())
    }

    /// Flushes writing buffer, receives response line and headers
    /// Prepares HTTP stream for reading data
    pub fn receive(&mut self) -> Result<(), Error> {
        self.stream.flush().context(HttpClientError::Context)?;
        self.response.parse(&mut self.stream).context(HttpClientError::Context)?;
        self.stream.configure(&self.response).context(HttpClientError::Context)?;

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
