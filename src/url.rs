use std::io::BufRead;


#[derive(Default)]
pub struct Url {
    name: String,
    path: String,
}


impl Url {
    pub fn new(inp: &str) -> Self {
        let mut url = Url::default();
        let v: Vec<&str> = inp.split('/').collect();
        url.name=v[2].to_string();
        url.path=url.find_path(&v);
        url
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
