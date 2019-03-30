use std::collections::HashMap;


pub struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
}


impl Request {
    pub fn new() -> Self {
        Request {
            method: String::new(),
            url: String::new(),
            headers: HashMap::new(),
        }
    }
    
    pub fn init<S>(&mut self, method: S, url: S) 
    where
        S: Into<String>,
    {
        self.method = method.into();
        self.url = url.into();
    }
    
    pub fn set<S>(&mut self, header_name: S, header_data: S)
    where
        S: Into<String> 
    {
        self.headers.insert(header_name.into(), header_data.into());
    }
    
    pub fn send(&self, mut dst: Vec<u8>) {
        dst = vec![0, 2, 4, 6];
    }
}

