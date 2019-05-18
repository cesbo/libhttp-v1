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


/// URL-encodes string
/// Supports RFC 3985. For better compatibility encodes space as `%20` (HTML5 `+` not supported)
pub fn urlencode(buf: &str) -> String {
    static HEXMAP: &[u8] = b"0123456789ABCDEF";
    let mut result = String::new();
    for &b in buf.as_bytes() {
        if is_rfc3986(b) {
            result.push(char::from(b));
        } else {
            result.push('%');
            result.push(char::from(HEXMAP[(b >> 4) as usize]));
            result.push(char::from(HEXMAP[(b & 0x0F) as usize]));
        }
    }
    result
}
