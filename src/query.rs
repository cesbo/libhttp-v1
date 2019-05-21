use std::{
    fmt,
    collections::HashMap,
};

use crate::urldecode::{
    urldecode,
    UrlDecodeError,
};


#[derive(Debug)]
pub struct ParseQueryError(String);


impl fmt::Display for ParseQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "ParseQuery: {}", self.0) }
}


impl From<UrlDecodeError> for ParseQueryError {
    fn from(e: UrlDecodeError) -> ParseQueryError { ParseQueryError(e.to_string()) }
}


/// Strings in query format - key-value tuples separated by '&',
/// with a '=' between the key and the value.
/// Such as url-query, urlencoded post body
pub struct Query(HashMap<String, String>);


impl fmt::Debug for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


impl Query {
    pub fn new(query: &str) -> Result<Query, ParseQueryError> {
        let mut map = HashMap::new();

        for data in query.split('&').filter(|s| !s.is_empty()) {
            let mut i = data.splitn(2, '=');
            let key = i.next().unwrap().trim();
            if key.is_empty() { continue }
            let key = urldecode(key)?;
            let value = i.next().unwrap_or("").trim();
            let value = urldecode(value)?;
            map.insert(key, value);
        }

        Ok(Query(map))
    }

    #[inline]
    pub fn get<R: AsRef<str>>(&self, key: R) -> Option<&str> {
        self.0.get(key.as_ref()).map(std::string::String::as_str)
    }
}
