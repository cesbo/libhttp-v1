use std::io::{
    self,
    BufRead,
    Read,
    Write,
};

use crate::{
    auth::auth,
    request::{
        Request,
        Error as RequestError,
    },
    response::{
        Response,
        Error as ResponseError,
    },
    stream::{
        HttpStream,
        Error as HttpStreamError,
    },
};


error_rules! {
    self => ("HttpClient: {}", error),
    io::Error,
    RequestError,
    ResponseError,
    HttpStreamError,
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
///     client.request.url.set("https://example.com");
///     client.request.header.set("host", client.request.url.get_address());
///     client.request.header.set("user-agent", "libhttp");
///     client.send().unwrap();
///     client.receive().unwrap();
///     let mut body = String::new();
///     client.read_to_string(&mut body).unwrap();
/// }
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
    /// Allocates new http client
    #[inline]
    pub fn new() -> Self { HttpClient::default() }

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
            _ => bail!("invalid protocol"),
        };

        self.stream.connect(tls, host, port)?;
        auth(&mut self.request, &self.response);
        self.request.send(&mut self.stream)?;
        self.stream.flush()?;

        Ok(())
    }

    /// Flushes writing buffer, receives response line and headers
    /// Prepares HTTP stream for reading data
    pub fn receive(&mut self) -> Result<()> {
        self.stream.flush()?;
        self.response.parse(&mut self.stream)?;
        self.stream.configure(&self.response)?;

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
