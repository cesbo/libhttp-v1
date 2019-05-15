use std::fmt;
use std::collections::HashMap;

use failure::{
    ensure,
    Error,
    Fail,
    ResultExt,
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


#[derive(Debug, Fail)]
#[fail(display = "UrlDecode: invalid hexadecimal code")]
pub struct UrlDecodeError;


/// Decodes URL-encoded string
/// Supports RFC 3985 and HTML5 `+` symbol
pub fn urldecode(buf: &str) -> Result<String, Error> {
    if buf.is_empty() {
        return Ok(String::new());
    }

    let mut result: Vec<u8> = Vec::new();
    let buf = buf.as_bytes();
    let len = buf.len();
    let mut skip = 0;

    while skip < len {
        let b = buf[skip];
        skip += 1;
        match b {
            b'%' => {
                ensure!(len >= skip + 2, UrlDecodeError);
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


#[derive(Debug, Fail)]
pub enum ParseQueryError {
    #[fail(display = "ParseQuery Error")]
    Context,
}


/// Strings in query format - key-value tuples separated by '&',
/// with a '=' between the key and the value.
/// Such as url-query, urlencoded post body
pub struct Query(HashMap<String, String>);


impl fmt::Debug for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


impl Query {
    pub fn new(query: &str) -> Result<Query, Error> {
        let mut map = HashMap::new();

        for data in query.split('&').filter(|s| !s.is_empty()) {
            let mut i = data.splitn(2, '=');
            let key = i.next().unwrap().trim();
            if key.is_empty() { continue }
            let key = urldecode(key).context(ParseQueryError::Context)?;
            let value = i.next().unwrap_or("").trim();
            let value = urldecode(value).context(ParseQueryError::Context)?;
            map.insert(key, value);
        }

        Ok(Query(map))
    }
}


#[derive(Debug, Fail)]
pub enum UrlError {
    #[fail(display = "Url: length limit")]
    LengthLimit,
    #[fail(display = "Url: empty url")]
    EmptyUrl,
    #[fail(display = "Url: unexpected relative path")]
    RelativeUrl,
    #[fail(display = "Url: invalid port")]
    InvalidPort,
    #[fail(display = "Url Error")]
    Context,
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
    pub fn new(u: &str) -> Result<Self, Error> {
        let mut url = Url::default();
        url.set(u)?;
        Ok(url)
    }

    /// Parse and absolute or relative URL from string
    pub fn set(&mut self, inp: &str) -> Result<(), Error> {
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

        ensure!(!inp.is_empty(), UrlError::EmptyUrl);
        ensure!(inp.len() < 2048, UrlError::LengthLimit);

        if let Some(v) = inp.find("://") {
            self.scheme += &inp[0 .. v];
            skip = v + 3;
        } else {
            // TODO: relative url
            ensure!(inp.starts_with('/'), UrlError::RelativeUrl);
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
            self.path = urldecode(&inp[path .. tail]).context(UrlError::Context)?;
            tail = path;
        }
        if prefix > 0 {
            self.prefix += &inp[path .. tail];
            skip = prefix + 1;
        }
        if skip != 0 {
            let mut addr = inp[skip .. tail].splitn(2, ':');
            self.host = addr.next().unwrap().to_string();
            if let Some(port) = addr.next() {
                self.port = port.parse::<u16>().unwrap_or(0);
                ensure!(self.port > 0, UrlError::InvalidPort);
            };
        }

        Ok(())
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
