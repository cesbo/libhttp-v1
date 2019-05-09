use std::{
    fmt,
    io,
    result,
};

use openssl::error::ErrorStack as SslErrorStack;
use openssl::ssl::HandshakeError;


pub type Result<T> = result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    Custom(&'static str),
    Io(io::Error),
    Ssl(SslErrorStack),
    Handshake(HandshakeError<std::net::TcpStream>),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Custom(e) => write!(f, "HTTP Error: {}", e),
            Error::Io(ref e) => write!(f, "HTTP IO Error: {}", e),
            Error::Ssl(ref e) => {
                let s = e.errors().get(0)
                    .and_then(|ee| ee.reason())
                    .unwrap_or("");
                write!(f, "HTTP SSL Error: {}", s)
            }
            Error::Handshake(ref e) => {
                match e {
                    HandshakeError::SetupFailure(ee) => {
                        let s = ee.errors().get(0)
                            .and_then(|eee| eee.reason())
                            .unwrap_or("");
                        write!(f, "HTTP Handshake Setup Failure: {}", s)
                    }
                    HandshakeError::Failure(ee) => {
                        write!(f, "HTTP Handshake Failure: ")?;
                        let inner_error = ee.error();
                        if let Some(io_ee) = inner_error.io_error() {
                            write!(f, "{}", io_ee)?;
                        } else if let Some(ssl_ee) = inner_error.ssl_error() {
                            let s = ssl_ee.errors().get(0)
                                .and_then(|eee| eee.reason())
                                .unwrap_or("unknown");
                            write!(f, "{}", s)?;
                            let v = ee.ssl().verify_result();
                            if v.as_raw() != 0 {
                                write!(f, ": {}", v.error_string())?;
                            }
                        }
                        Ok(())
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
}


impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}


impl From<SslErrorStack> for Error {
    fn from(e: SslErrorStack) -> Self {
        Error::Ssl(e)
    }
}


impl From<HandshakeError<std::net::TcpStream>> for Error {
    fn from(e: HandshakeError<std::net::TcpStream>) -> Self {
        Error::Handshake(e)
    }
}
