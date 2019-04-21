use crate::error::{
    Error,
    Result,
};


#[derive(Default, Debug)]
pub struct Url {
    scheme: String,
    prefix: String,
    host: String,
    port: Option<usize>,
    path: String,
    query: String,
    fragment: String,
}


pub struct HostPort {
    pub host: String,
    pub port: Option<u16>,
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
        let mut host = 0;
        let mut port = 0;
        let mut path = 0;
        let mut query = 0;
        let mut fragment = 0;
        if let Some(v) = inp.find("://") {
            skip = v + 3;
        }
        for (idx, part) in inp[skip ..].match_indices(|c: char| (c == '/' || c == '?' || c == '#' || c == '@' || c == ':')) {
            match part.as_bytes()[0] {
                b'@' if step < 1 => { host = idx + skip; step = 1; },
                b':' if step < 2 => { port = idx + skip; step = 2; },
                b'/' if step < 3 => { path = idx + skip; step = 3; },
                b'?' if step < 4 => { query = idx + skip; step = 4; },
                b'#' if step < 5 => { fragment = idx + skip; break; },
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
        if path > 0 {
            self.path += &inp[path .. tail];
            tail = path;
        } 
        if port > 0 {
            self.port += &inp[port .. tail];// Change to .parse() and input to option
            tail = port;
        } 
        if host > 0 {
            self.host += &inp[host .. tail];
            tail = host;
        } 
		if skip > 2 {
            self.scheme += &inp[0 .. skip - 3];
            if host == 0 {
                self.host += &inp[skip .. tail];
            } else {
                self.prefix += &inp[skip .. tail];
            }
        } else {
            self.path += &inp[0 .. tail];
        } 
        if self.port == None {
            self.port = match self.scheme.as_str() {
                "http" => Some(80),
                "https" => Some(443),
                _ => None,
            }
        }
    }
    
    #[inline]
    pub fn get_name(&self) -> &str {
        self.name.as_str() // Change to format!(... ...
    }
    
    #[inline]
    pub fn get_scheme(&self) -> &str {
        self.scheme.as_str()
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
}
