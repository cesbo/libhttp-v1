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
        let mut i = inp.split('?');
        let path = i.next().unwrap();
        let query_fragment = i.next().unwrap();
        i = query_fragment.split('#');
        let query = i.next().unwrap();
        let fragment = i.next().unwrap();
        let v: Vec<&str> = path.split('/').collect();
        Url {
            scheme: v[0].replace(":",""),
            name: v[2].to_string(),
            path: url_path(&v),
            query: query.to_string(),
            fragment: fragment.to_string(),
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
