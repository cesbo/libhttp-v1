use std::{
    char,
    fmt,
    convert::TryFrom,
};


/// Decodes URL-encoded string
/// Supports RFC 3985 and HTML5 `+` symbol
pub struct UrlDecoder<'a> {
    inner: &'a str,
}


impl<'a> UrlDecoder<'a> {
    #[inline]
    pub fn new(s: &'a str) -> UrlDecoder<'a> {
        UrlDecoder {
            inner: s,
        }
    }
}


impl<'a> fmt::Display for UrlDecoder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = self.inner.as_bytes();
        let len = buf.len();
        let mut skip = 0;

        let mut bytes = 0;
        let mut utf8: u32 = 0;

        while skip < len {
            let b = buf[skip];
            skip += 1;

            if b == b'%' {
                if skip + 2 > len { return Err(fmt::Error) }

                let n0 = char::from(buf[skip]).to_digit(16).ok_or(fmt::Error)?;
                skip += 1;
                let n1 = char::from(buf[skip]).to_digit(16).ok_or(fmt::Error)?;
                skip += 1;
                let b = ((n0 << 4) + n1) as u8;

                if b & 0x80 == 0 {
                    // ASCII
                    fmt::Write::write_char(f, char::from(b))?;
                } else if bytes > 0 {
                    // UTF8 trailing bytes
                    if b & 0xC0 != 0x80 { return Err(fmt::Error) }

                    utf8 = (utf8 << 6) | u32::from(b & 0x3F);
                    bytes = bytes - 1;
                    if bytes == 0 {
                        let b = char::from_u32(utf8).ok_or(fmt::Error)?;
                        fmt::Write::write_char(f, b)?;
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
                    return Err(fmt::Error)
                }
            } else if b == b'+' {
                fmt::Write::write_char(f, ' ')?;
            } else {
                fmt::Write::write_char(f, char::from(b))?;
            }
        }

        Ok(())
    }
}


impl<'a> TryFrom<UrlDecoder<'a>> for String {
    type Error = fmt::Error;

    #[inline]
    fn try_from(u: UrlDecoder<'a>) -> Result<String, fmt::Error> {
        let mut result = String::default();
        fmt::Write::write_fmt(&mut result, format_args!("{}", &u))?;
        Ok(result)
    }
}
