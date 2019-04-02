pub struct Url {
    name: String,
    path: String,
}

impl Url {
    pub fn new<S>(url: S) -> Self 
    where
        S:Into<String>
    {
        let v: Vec<&str> = url.into().split('/').collect();
        Url {
            name: v[2].to_string(),
            path: self.find_path(&v),
        }
    }
    
    fn find_path(&self, v: &Vec<&str>) -> String {
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
}
