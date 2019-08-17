// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use std::{
    error,
    fmt,
    net::TcpStream,
};


/// Wrapper for OpenSSL errors
#[derive(Debug)]
pub struct SslError(openssl::error::ErrorStack);


impl error::Error for SslError {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { Some(&self.0) }
}


impl fmt::Display for SslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.errors().get(0)
            .and_then(openssl::error::Error::reason)
            .unwrap_or("");
        write!(f, "{}", s)
    }
}


impl From<openssl::error::ErrorStack> for SslError {
    #[inline]
    fn from(e: openssl::error::ErrorStack) -> SslError { SslError(e) }
}


/// Wrapper for OpenSSL TLS handshake error
#[derive(Debug)]
pub struct HandshakeError(openssl::ssl::HandshakeError<TcpStream>);


impl error::Error for HandshakeError {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { Some(&self.0) }
}


impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            openssl::ssl::HandshakeError::SetupFailure(ee) => {
                let s = ee.errors().get(0)
                    .and_then(openssl::error::Error::reason)
                    .unwrap_or("");
                write!(f, "{}", s)
            }
            openssl::ssl::HandshakeError::Failure(ee) => {
                let inner_error = ee.error();
                if let Some(io_ee) = inner_error.io_error() {
                    write!(f, "{}", io_ee)
                } else if let Some(ssl_ee) = inner_error.ssl_error() {
                    let s = ssl_ee.errors().get(0)
                        .and_then(openssl::error::Error::reason)
                        .unwrap_or("unknown");

                    let v = ee.ssl().verify_result();
                    if v.as_raw() == 0 {
                        write!(f, "{}", s)
                    } else {
                        write!(f, "{}: {}", s, v.error_string())
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unimplemented!(),
        }
    }
}


impl From<openssl::ssl::HandshakeError<TcpStream>> for HandshakeError {
    #[inline]
    fn from(e: openssl::ssl::HandshakeError<TcpStream>) -> HandshakeError { HandshakeError(e) }
}
