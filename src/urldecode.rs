use std::{
    char,
    convert::TryFrom,
    fmt,
};


#[inline]
fn hex2byte(b: &[u8]) -> Option<u8> {
    if b.len() < 2 { return None }

    let n0 = char::from(b[0]).to_digit(16)?;
    let n1 = char::from(b[1]).to_digit(16)?;
    Some(((n0 << 4) + n1) as u8)
}



/// Decodes URL-encoded string
/// Supports RFC 3985 and HTML5 `+` symbol
///
/// ## Usage
///
/// ```
/// use std::convert::TryFrom;
/// use http::UrlDecoder;
///
/// static PATH: &str = "%2Fpath%2F%F0%9F%8D%94%2F";
///
/// assert_eq!(
///     String::try_from(UrlDecoder::new(PATH)).unwrap().as_str(),
///     "/path/🍔/");
/// ```
pub struct UrlDecoder<'a> {
    inner: &'a str,
}


impl<'a> UrlDecoder<'a> {
    /// Allocate UrlDecoder
    #[inline]
    pub fn new(s: &'a str) -> UrlDecoder<'a> {
        UrlDecoder {
            inner: s,
        }
    }
}


impl<'a> TryFrom<UrlDecoder<'a>> for String {
    type Error = fmt::Error;

    fn try_from(u: UrlDecoder<'a>) -> Result<String, fmt::Error> {
        let buf = u.inner.as_bytes();
        let len = buf.len();
        let mut skip = 0;

        let mut result = String::with_capacity(len);

        let mut bytes = 0;
        let mut utf8: u32 = 0;

        while skip < len {
            let b = buf[skip];
            skip += 1;

            if b == b'%' {
                let b = hex2byte(&buf[skip ..]).ok_or(fmt::Error)?;
                skip += 2;

                if b & 0x80 == 0 {
                    // ASCII
                    result.push(char::from(b));
                } else if bytes > 0 {
                    // UTF8 trailing bytes
                    if b & 0xC0 != 0x80 { return Err(fmt::Error) }

                    utf8 = (utf8 << 6) | u32::from(b & 0x3F);
                    bytes = bytes - 1;
                    if bytes == 0 {
                        let b = unsafe { char::from_u32_unchecked(utf8) };
                        result.push(b);
                    }
                } else if b & 0xE0 == 0xC0 {
                    // UTF8 first byte for 2 byte code
                    utf8 = u32::from(b & 0x1F);
                    bytes = 1;
                } else if b & 0xF0 == 0xE0 {
                    // UTF8 first byte for 3 byte code
                    utf8 = u32::from(b & 0x0F);
                    bytes = 2;
                } else if b & 0xF8 == 0xF0 {
                    // UTF8 first byte for 4 byte code
                    utf8 = u32::from(b & 0x07);
                    bytes = 3;
                } else {
                    return Err(fmt::Error);
                }
            } else if b == b'+' {
                result.push(' ');
            } else {
                result.push(char::from(b));
            }
        }

        Ok(result)
    }
}
