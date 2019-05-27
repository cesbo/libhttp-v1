error_rules! {
    Error => ("{}", error),
    InvalidHex => ("invalid hexadecimal code"),
}


/// Decodes URL-encoded string
/// Supports RFC 3985 and HTML5 `+` symbol
pub fn urldecode(buf: &str) -> Result<String> {
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
                ensure!(len >= skip + 2, InvalidHex);

                let n0 = char::from(buf[skip]).to_digit(16).ok_or(InvalidHex)?;
                skip += 1;
                let n1 = char::from(buf[skip]).to_digit(16).ok_or(InvalidHex)?;
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
