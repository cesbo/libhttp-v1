use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, BufRead, BufReader, Write, BufWriter};

use crate::error::{
    Error,
    Result,
};


#[derive(Debug)]
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
        println!("{:#?}", self);
    }
    
    pub fn send<W: Write>(&self, dst: &mut W) -> Result<()> {
        writeln!(dst, "{} {} HTTP/1.1\r", self.method, self.head_from_url(&self.url))?;
        writeln!(dst, "Host: {}\r", self.host_from_url(&self.url))?;
        for (param, value) in self.headers.iter() {
            writeln!(dst, "{}: {}\r", param, value)?;
        } 
        writeln!(dst, "\r")?;
        Ok(())
    }
    
    fn head_from_url(&self, url: &str) -> String {
        let v: Vec<&str> = url.split('/').collect();
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
    
    fn host_from_url(&self, url: &str) -> String {
        let v: Vec<&str> = url.split('/').collect();
        v[2].to_string()
    }
}

