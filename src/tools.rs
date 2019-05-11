use std::{
    fmt,
    io,
    result,
};


pub (crate) fn header_write<W: io::Write>(dst: &mut W, key: &str, value: &str) -> io::Result<()> {
    for (step, part) in key.split('-').enumerate() {
        if step > 0 {
            write!(dst, "-")?;
        }
        if ! part.is_empty() {
            write!(dst, "{}", &part[.. 1].to_uppercase())?;
            write!(dst, "{}", &part[1 ..])?;
        }
    }
    writeln!(dst, ": {}\r", value)
}


#[derive(Debug)]
pub enum ParseHexError {
    Length,
    Format,
}


impl fmt::Display for ParseHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseHexError::Length => write!(f, "ParseHex: string must have an even length"),
            ParseHexError::Format => write!(f, "ParseHex: string must be hexadecimal"),
        }
    }
}


const HEXMAP: &[u8] = b"0123456789abcdef";


pub fn bin2hex(dst: &mut String, src: &[u8]) {
    src.iter().fold(dst, |acc, b| {
        acc.push(char::from(HEXMAP[(b >> 4) as usize]));
        acc.push(char::from(HEXMAP[(b & 0x0F) as usize]));
        acc
    });
}


pub fn hex2bin<R>(dst: &mut Vec<u8>, src: R) -> result::Result<(), ParseHexError>
where
    R: AsRef<str>,
{
    let src = src.as_ref().as_bytes();
    let len = src.len();
    let mut skip = 0;

    while skip + 2 <= len {
        let n0 = char::from(src[skip]).to_digit(16).ok_or(ParseHexError::Format)?;
        skip += 1;
        let n1 = char::from(src[skip]).to_digit(16).ok_or(ParseHexError::Format)?;
        skip += 1;
        dst.push(((n0 << 4) + n1) as u8);
    }

    if skip == len {
        Ok(())
    } else {
        Err(ParseHexError::Length)
    }
}
