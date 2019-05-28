use std::fmt::{
    self,
    Write,
};


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


/// Encodes string into URL format
/// Supports RFC 3985. For better compatibility encodes space as `%20`
///
/// ## Usage
///
/// ```
/// use http::UrlEncoder;
///
/// static PATH: &str = "/path/🍔/";
///
/// assert_eq!(
///     UrlEncoder::new_path(PATH).to_string().as_str(),
///     "/path/%F0%9F%8D%94/");
/// assert_eq!(
///     UrlEncoder::new(PATH).to_string().as_str(),
///     "%2Fpath%2F%F0%9F%8D%94%2F");
/// ```
pub struct UrlEncoder<'a> {
    inner: &'a str,
    is_path: bool,
}


impl<'a> UrlEncoder<'a> {
    /// Allocate UrlEncoder for encoding all special characters according to RFC 3985
    #[inline]
    pub fn new(s: &'a str) -> UrlEncoder<'a> {
        UrlEncoder {
            inner: s,
            is_path: false,
        }
    }

    /// Allocate UrlEncoder for encoding all special characters, except: `/`, `:`, `,`, `=`
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
        let is_special = if self.is_path { is_rfc3986_path } else { is_rfc3986 };

        for &b in self.inner.as_bytes() {
            if is_special(b) {
                f.write_char(char::from(b))?;
            } else {
                f.write_char('%')?;
                f.write_char(char::from(HEXMAP[(b >> 4) as usize]))?;
                f.write_char(char::from(HEXMAP[(b & 0x0F) as usize]))?;
            }
        }
        Ok(())
    }
}
