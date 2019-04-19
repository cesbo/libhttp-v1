use std::{fmt, io, result, str, num};


#[derive(Debug)]
pub enum Error {
    Syntax(usize, &'static str),
    Custom(&'static str),
    ParseBoolError(usize, str::ParseBoolError),
    ParseIntError(usize, num::ParseIntError),
    Io(io::Error),
}


pub type Result<T> = result::Result<T, Error>;


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(line, e) => write!(f, "Syntax Error at line {}: {}", line, e),
            Error::Custom(e) => write!(f, "Error: {}", e),
            Error::ParseBoolError(line, ref e) => write!(f, "Format Error at line {}: {}", line, e),
            Error::ParseIntError(line, ref e) => write!(f, "Format Error at line {}: {}", line, e),
            Error::Io(ref e) => io::Error::fmt(e, f),
        }
    }
}


impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

