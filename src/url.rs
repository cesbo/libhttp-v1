use std::collections::HashMap;

use crate::tools;


#[inline]
fn is_rfc3986(b: u8) -> bool {
    match b {
        b'a' ..= b'z' => true,
        b'A' ..= b'Z' => true,
        b'0' ..= b'9' => true,
        b'-' => true,
        b'_' => true,
        b'.' => true,
        b'~' => true,
        _ => false,
    }
}


/// Decodes URL-encoded string
/// Supports RFC 3985 and HTML5 `+` symbol
#[inline]
pub fn urldecode(buf: &str) -> String {
    let mut result: Vec<u8> = Vec::new();
    let buf = buf.as_bytes();
    let mut skip = 0;
    let len = buf.len();
    while skip < len {
        let b = buf[skip];
        skip += 1;
        match b {
            b'%' => {
                let next = skip + 2;
                if let Err(_) = tools::hex2bin(&mut result, &buf[skip .. next]) {
                    result.push(b'-');
                }
                skip = next;
            },
            b'+' => result.push(b' '),
            _ => result.push(b),
        }
    }
    unsafe {
        String::from_utf8_unchecked(result)
    }
}


const HEXMAP: &[u8] = b"0123456789ABCDEF";


/// URL-encodes string
/// Supports RFC 3985. For better compatibility encodes space as `%20` (HTML5 `+` not supported)
#[inline]
pub fn urlencode(buf: &str) -> String {
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


/// Parses strings in query format - key-value tuples separated by '&',
/// with a '=' between the key and the value.
/// Such as url-query, urlencoded post body
#[inline]
pub fn parse_query(query: &str) -> HashMap<String, String> {
    let mut ret = HashMap::new();
    for data in query.split('&') {
        let mut i = data.splitn(2, '=');
        let key = i.next().unwrap();
        if key.is_empty() {
            continue;
        }
        let key = urldecode(key);
        let value = urldecode(i.next().unwrap_or(""));
        ret.insert(key, value);
    }
    ret
}


/// A parsed URL record
///
/// URL parts: `scheme://prefix@host:port/path?query#fragment`
/// All url parts are optional.
/// If path, query, and fragment are defined, then value contains their delimiter as well
/// If port not defined then value will be 0
#[derive(Default, Debug, PartialEq)]
pub struct Url {
    scheme: String,
    prefix: String,
    host: String,
    port: u16,
    path: String,
    query: String,
    fragment: String,
}


impl Url {
    /// Allocate new object and parse url
    pub fn new(u: &str) -> Self {
        let mut url = Url::default();
        url.set(u);
        url
    }

    /// Parse and absolute or relative URL from string
    pub fn set(&mut self, inp: &str) {
        let mut skip = 0;
        // step values:
        // 0 - prefix
        // 1 - host:port
        // 2 - /path
        // 3 - ?query
        // 4 - #fragment
        let mut step = 0;
        let mut prefix = 0;
        let mut path = 0;
        let mut query = 0;
        let mut fragment = 0;
        if let Some(v) = inp.find("://") {
            self.scheme += &inp[0 .. v];
            skip = v + 3;
        } else {
            step = 2;
        }
        for (idx, part) in inp[skip ..].match_indices(|c: char| {
            c == '/' || c == '?' || c == '#' || c == '@'
        }) {
            match part.as_bytes()[0] {
                b'@' if step < 1 => { prefix = idx + skip; step = 1; },
                b'/' if step < 2 => { path = idx + skip; step = 2; },
                b'?' if step < 3 => { query = idx + skip; step = 3; },
                b'#' if step < 4 => { fragment = idx + skip; break; },
                _ => {},
            };
        }
        let mut tail = inp.len();
        if fragment > 0 {
            self.fragment += &inp[fragment .. tail];
            tail = fragment;
        }
        if query > 0 {
            self.query += &inp[query .. tail];
            tail = query;
        }
        if path > 0 || skip == 0 {
            self.path = urldecode(&inp[path .. tail]);
            tail = path;
        }
        if prefix > 0 {
            self.prefix += &inp[path .. tail];
            skip = prefix + 1;
        }
        if skip != 0 {
            let mut addr = inp[skip .. tail].splitn(2, ':');
            self.host = addr.next().unwrap().to_string();
            self.port = addr.next().and_then(|v| v.parse::<u16>().ok()).unwrap_or(0);
        }
    }

    /// Returns url scheme
    #[inline]
    pub fn get_scheme(&self) -> &str {
        self.scheme.as_str()
    }

    /// Returns url prefix
    #[inline]
    pub fn get_prefix(&self) -> &str {
        self.prefix.as_str()
    }

    /// Returns url host
    #[inline]
    pub fn get_host(&self) -> &str {
        self.host.as_str()
    }

    /// Returns url port
    #[inline]
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Returns url path
    #[inline]
    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }

    /// Returns url query
    #[inline]
    pub fn get_query(&self) -> &str {
        self.query.as_str()
    }

    /// Returns url fragment
    #[inline]
    pub fn get_fragment(&self) -> &str {
        self.fragment.as_str()
    }
}
