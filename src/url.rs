use crate::error::{
    Error,
    Result,
};


#[derive(Default, Debug)]
pub struct Url {
    scheme: String,
    name: String,
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
        let mut path = 0;
        let mut query = 0;
        let mut fragment = 0;
        if let Some(v) = inp.find("://") {
            skip = v + 3;
        }
        for (idx, part) in inp[skip ..].match_indices(|c: char| (c == '/' || c == '?' || c == '#' )) {
            match part.as_bytes()[0] {
                b'/' if step < 1 => { path = idx + skip; step = 1; },
                b'?' if step < 2 => { query = idx + skip; step = 2; },
                b'#' if step < 3 => { fragment = idx + skip; break; },
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
		if skip > 2 {
            self.scheme += &inp[0 .. skip - 3];
			self.name += &inp[skip .. tail];
		} else {
		    self.path += &inp[0 .. tail];
		} 
    }

    pub fn get_host_port(&self) -> Result<(&str, u16)> {
        let host_port = {
            if let Some(s) = &self.name.find(':') {
                let host = &self.name[.. s];
                let port = match &self.name.as_str()[s + 1 ..].parse() {
                    Ok(v) => v,
                    _ => return Err(Error::Custom("wrong port value")),
                };
                (host, port)
            } else {
                let port = match self.scheme.as_str() {
                    "http" => 80,
                    "https" => 443,
                    _ => return Err(Error::Custom("port not defined")),
                };
                (&self.name, port)
            }
        };
        Ok(host_port)
    }
    
    #[inline]
    pub fn get_name(&self) -> &str {
        self.name.as_str()
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
