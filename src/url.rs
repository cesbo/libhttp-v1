use std::io::BufRead;


#[derive(Default, Debug)]
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


//#[derive(Debug)]
impl Url {
    pub fn new(inp: &str) -> Self {
		let mut query = "";
		let mut fragment = "";
		let mut path = "";
		let mut i = inp.split('?');
        let mut pre_path = i.next().unwrap_or("");
		if pre_path.find("#") != None {
		    let mut path_fragment = pre_path.split('#');
			path = path_fragment.next().unwrap_or("");
			fragment = path_fragment.next().unwrap_or("");
		} else {
		    path = pre_path;
			let query_fragment = i.next().unwrap_or("");
            i = query_fragment.split('#');
            query = i.next().unwrap_or("");
		    fragment = i.next().unwrap_or("");
		}
        let v: Vec<&str> = path.split('/').collect();
		let name = match v.len() {
		    1 | 2 => "",
			_ => v[2],
		};
        Url {
            scheme: v[0].replace(":","").to_string(),
            name: name.to_string(),
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
