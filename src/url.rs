use std::io::Write;
use std::collections::HashMap;

use crate::error::Result;


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


#[inline]
fn hex2nibble(b: u8) -> Option<u8> {
    match b {
        b'0' ..= b'9' => Some(b - b'0'),
        b'A' ..= b'F' => Some(b - b'A' + 10),
        b'a' ..= b'f' => Some(b - b'a' + 10),
        _ => None,
    }
}


#[inline]
fn hex2byte(buf: &[u8]) -> u8 {
    if buf.len() >= 2 {
        if let Some(n0) = hex2nibble(buf[0]) {
            if let Some(n1) = hex2nibble(buf[1]) {
                return n0 * 16 + n1;
            }
        }
    }
    b'-'
}


#[inline]
fn byte2hex(b: u8) -> char {
    if b < 0x0A {
        char::from(b'0' + b)
    } else {
        char::from(b'A' - 0x0A + b)
    }
}


#[inline]
pub fn urldecode(buf: &str) -> String {
    let mut result: Vec<u8> = Vec::new(); 
    let buf = buf.as_bytes();
    let mut skip = 0;
    let len = buf.len();
    while skip < len {
        let b = buf[skip];
        skip += 1;
        if b == b'%' {
            result.push(hex2byte(&buf[skip ..]));
            skip += 2;
        } else {
            result.push(b);
        }
    }
    unsafe { 
        String::from_utf8_unchecked(result)
    }
}


#[inline]
pub fn urlencode(buf: &str) -> String {
    let mut result = String::new();
    for &b in buf.as_bytes() {
        if is_rfc3986(b) {
            result.push(char::from(b));
        } else {
            result.push('%');
            result.push(byte2hex(b >> 4));
            result.push(byte2hex(b & 0x0F));
        }
    }
    result
}


#[inline]
pub fn pars_query(query: &str) -> HashMap<&str, &str> {
    let mut ret = HashMap::new();
    let mut name = "";
    for data in query.split('&') {
        for v in data.split('=') {
            if name.is_empty() {
                name = v;
            } else {
                ret.insert(name, v);
                name = "";
            }
        }
    }
    ret
}


#[derive(Default, Debug)]
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
    pub fn new(u: &str) -> Self {
        let mut url = Url::default();
        if ! u.is_empty() {
            url.set(u);
        }
        url
    }
    
    pub fn set(&mut self, inp: &str) {
        let mut skip = 0;
        let mut step = 0;
        let mut prefix = 0;
        let mut path = 0;
        let mut query = 0;
        let mut fragment = 0;
        if let Some(v) = inp.find("://") {
            self.scheme += &inp[0 .. v];
            skip = v + 3;
        }
        for (idx, part) in inp[skip ..].match_indices(|c: char| (c == '/' || c == '?' || c == '#' || c == '@')) {
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
            self.port = match addr.next() {
                Some(v) => match v.parse::<u16>() {
                    Ok(v) => v,
                    _ => 0,
                },
                None => 0,
            };
        }
    }

    #[inline]
    pub fn get_scheme(&self) -> &str {
        self.scheme.as_str()
    }
    
    #[inline]
    pub fn get_prefix(&self) -> &str {
        self.prefix.as_str()
    }

    #[inline]
    pub fn get_host(&self) -> &str {
        self.host.as_str()
    }

    #[inline]
    pub fn get_port(&self) -> u16 {
        self.port
    }
    
    #[inline]
    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }
    
    #[inline]
    pub fn get_query(&self) -> &str {
        self.query.as_str()
    }
    
    #[inline]
    pub fn get_fragment(&self) -> &str {
        self.fragment.as_str()
    }
    
    #[inline]
    pub fn write_request_url<W: Write>(&self, dst: &mut W) -> Result<()> {
        if self.path.is_empty() {
            write!(dst, "/{}", self.query)?;
        } else {
            write!(dst, "{}{}", self.path, self.query)?;
        }
        Ok(())
    }
    
    #[inline]
    pub fn write_header_host<W: Write>(&self, dst: &mut W) -> Result<()> {
        if self.port == 80 || self.port == 443 || self.port == 0 {
            write!(dst, "{}", self.host)?;
        } else {
            write!(dst, "{}:{}", self.host, self.port)?;
        }
        Ok(())
    }
}
