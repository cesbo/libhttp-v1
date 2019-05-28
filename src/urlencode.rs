use std::fmt;


#[inline]
fn is_rfc3986(b: u8) -> bool {
    match b {
        b'a' ..= b'z' => true,
        b'A' ..= b'Z' => true,
        b'0' ..= b'9' => true,
        b'-' | b'_' | b'.' | b'~' => true,
        _ => false,
    }
}


#[inline]
fn is_rfc3986_path(b: u8) -> bool {
    match b {
        b'a' ..= b'z' => true,
        b'A' ..= b'Z' => true,
        b'0' ..= b'9' => true,
        b'-' | b'_' | b'.' | b'~' | b'/' | b':' | b',' | b'=' => true,
        _ => false,
    }
}


pub struct UrlEncoder<'a> {
    inner: &'a str,
    is_path: bool,
}


impl<'a> UrlEncoder<'a> {
    #[inline]
    pub fn new(s: &'a str) -> UrlEncoder<'a> {
        UrlEncoder {
            inner: s,
            is_path: false,
        }
    }

    #[inline]
    pub fn new_path(s: &'a str) -> UrlEncoder<'a> {
        UrlEncoder {
            inner: s,
            is_path: true,
        }
    }
}


impl<'a> fmt::Display for UrlEncoder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        static HEXMAP: &[u8] = b"0123456789ABCDEF";
        let is_special = if self.is_path { is_rfc3986 } else { is_rfc3986_path };

        for &b in self.inner.as_bytes() {
            if is_special(b) {
                fmt::Write::write_char(f, char::from(b))?;
            } else {
                fmt::Write::write_char(f, '%')?;
                fmt::Write::write_char(f, char::from(HEXMAP[(b >> 4) as usize]))?;
                fmt::Write::write_char(f, char::from(HEXMAP[(b & 0x0F) as usize]))?;
            }
        }
        Ok(())
    }
}
