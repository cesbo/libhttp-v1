use std::io::BufRead;


#[derive(Default)]
pub struct Url {
    scheme: String,
    name: String,
    path: String,
    query: String,
    fragment: String,
}


fn url_path(v: &Vec<&str>) -> String {
    let mut flag: usize = 5;
    let mut result = String::new();
    for part in v {
        if flag > 1 {
            flag -= 1;
        }
        if flag == 1 {
            result.push_str(&format!("/{}", part));
        }
    }
    result
}


impl Url {
    pub fn new(inp: &str) -> Self {
        let path: Vec<&str> =  inp.split('?').collect();
        let query_fragment: Vec<&str> = path[1].split('#').collect();
        let v: Vec<&str> = path[0].split('/').collect();
        Url {
            scheme: v[0].replace(":",""),
            name: v[2].to_string(),
            path: url_path(&v),
            query: query_fragment[0].to_string(),
            fragment: query_fragment[1].to_string(),
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
