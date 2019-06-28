use std::{
    io::{
        self,
        Read,
        BufRead,
        Write,
    },
};

use crate::{
    HttpVersion,
    Response,
    transfer::{
        HttpTransfer,
        HttpTransferError,
    },
};


#[derive(Debug, Error)]
pub enum HttpStreamError {
    #[error_from]
    Io(io::Error),
    #[error_from]
    Transfer(HttpTransferError),
}


type Result<T> = std::result::Result<T, HttpStreamError>;


/// HTTP transport stream
#[derive(Debug, Default)]
pub struct HttpStream {
    transfer: HttpTransfer,
}


impl HttpStream {
    /// Close connection
    #[inline]
    pub fn close(&mut self) { self.transfer.close() }

    /// Opens a TCP connection to a remote host
    /// If connection already opened just clears read/write buffers
    #[inline]
    pub fn connect(&mut self, tls: bool, host: &str, port: u16) -> Result<()> {
        self.transfer.connect(tls, host, port)?;
        Ok(())
    }

    /// Checks response headers and set content parser behavior
    /// no_content - protocol specified response without content
    pub fn configure(&mut self, no_content: bool, response: &Response) -> Result<()> {
        if let Some(connection) = response.header.get("connection") {
            if connection.eq_ignore_ascii_case("keep-alive") {
                self.transfer.set_connection_keep_alive();
            } else {
                self.transfer.set_connection_close();
            }
        } else {
            if response.get_version() == HttpVersion::HTTP10 {
                self.transfer.set_connection_close();
            } else {
                self.transfer.set_connection_keep_alive();
            }
        }

        if no_content {
            self.transfer.set_content_length(0);
            return Ok(());
        }

        if let Some(len) = response.header.get("content-length") {
            let len = len.parse().unwrap_or(0);
            self.transfer.set_content_length(len);
            return Ok(());
        }

        if let Some(encoding) = response.header.get("transfer-encoding") {
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
}


impl Read for HttpStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.transfer.read(buf) }
}


impl BufRead for HttpStream {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> { self.transfer.fill_buf() }

    #[inline]
    fn consume(&mut self, amt: usize) { self.transfer.consume(amt) }
}


impl Write for HttpStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.transfer.write(buf) }

    #[inline]
    fn flush(&mut self) -> io::Result<()> { self.transfer.flush() }
}
