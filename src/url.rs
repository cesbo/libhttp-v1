use crate::error::{
    Error,
    Result,
};


#[derive(Default, Debug)]
pub struct Url {
    name: String,
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
        if path > 0 {
            self.path += &inp[path .. tail];
            tail = path;
        } 
        if prefix > 0 {
            self.prefix += &inp[path .. tail];
            skip = prefix + 1;
        }
        if skip == 0 {
            self.path += &inp[skip .. tail];
        } else {
            let mut addr = inp[skip .. tail].splitn(2, ':');
            self.name += &inp[skip .. tail];
            self.host = addr.next().unwrap().to_string();
            self.port = match addr.next() {
                Some(v) => match v.parse::<u16>() {
                    Ok(v) => v,
                    _ => 0,
                },
                None => 0,
            };
        }
        if self.port == 0 && ! self.scheme.is_empty() {
            self.port = match self.scheme.as_str() {
                "http" => 80,
                "https" => 443,
                _ => 0,
            }
        }
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
}
