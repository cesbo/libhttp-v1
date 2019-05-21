use std::fmt;


#[derive(Debug)]
pub struct UrlDecodeError;


impl fmt::Display for UrlDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UrlDecode: invalid hexadecimal code")
    }
}


/// Decodes URL-encoded string
/// Supports RFC 3985 and HTML5 `+` symbol
pub fn urldecode(buf: &str) -> Result<String, UrlDecodeError> {
    if buf.is_empty() { return Ok(String::new()) }

    let mut result: Vec<u8> = Vec::new();
    let buf = buf.as_bytes();
    let len = buf.len();
    let mut skip = 0;

    while skip < len {
        let b = buf[skip];
        skip += 1;
        match b {
            b'%' => {
                if len < skip + 2 { return Err(UrlDecodeError) }

                let n0 = char::from(buf[skip]).to_digit(16).ok_or(UrlDecodeError)?;
                skip += 1;
                let n1 = char::from(buf[skip]).to_digit(16).ok_or(UrlDecodeError)?;
                skip += 1;
                result.push(((n0 << 4) + n1) as u8);
            },
            b'+' => result.push(b' '),
            _ => result.push(b),
        }
    }

    Ok(unsafe {
        String::from_utf8_unchecked(result)
    })
}
