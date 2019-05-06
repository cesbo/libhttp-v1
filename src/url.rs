use std::io::Write;

use crate::error::Result;


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
            self.path += &inp[path .. tail];
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
    fn is_rfc3986(&self, b: u8) -> bool {
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
    fn byte2hex(&self, b: u8) -> char {
        if b < 0x0A {
            char::from(b'0' + b)
        } else {
            char::from(b'A' - 0x0A + b)
        }
    }

    fn pars_hex(&self, b: u8) -> Option<u8> {
        match b {
            b'0' ..= b'9' => Some(b - 48), 
            b'A' ..= b'F' => Some(b - 55), 
            b'a' ..= b'f' => Some(b - 87), 
            _ => None,
        }
    }

    #[inline]
    pub fn urlencode(&self, buf: &str) -> String {
        let mut result = String::new();
        for &b in buf.as_bytes() {
            if self.is_rfc3986(b) {
                result.push(char::from(b));
            } else {
                result.push('%');
                result.push(self.byte2hex(b >> 4));
                result.push(self.byte2hex(b & 0x0F));
            }
        }
        result
    }

    pub fn urldecode(&self, buf: &str) -> String {
        let mut result: Vec<u8> = Vec::new(); 
        let mut step = 0;
        let mut buffer = 0;
        for &b in buf.as_bytes() {
            if step > 0 {                
                step += 1;
                buffer +=  match self.pars_hex(b) {
                    Some(v) => v,
                    None => return "".to_string(),
                };
                if step == 2 {
                    buffer = buffer << 4;
                }
                if step == 3{
                    result.push(buffer);
                    buffer = 0;
                    step = 0;
                }
            } else if b == b'%' { 
                step = 1;
            } else {
                result.push(b);
            }
        }
        unsafe { 
            String::from_utf8_unchecked(result)
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
