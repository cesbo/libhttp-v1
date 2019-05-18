use std::fmt;
use std::collections::HashMap;

use failure::{
    Error,
    Fail,
};

use crate::urldecode::urldecode;


#[derive(Debug, Fail)]
#[fail(display = "ParseQuery: {}", 0)]
struct ParseQueryError(Error);


impl From<Error> for ParseQueryError {
    #[inline]
    fn from(e: Error) -> ParseQueryError { ParseQueryError(e) }
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
    pub fn new(query: &str) -> Result<Query, Error> {
        let mut map = HashMap::new();

        for data in query.split('&').filter(|s| !s.is_empty()) {
            let mut i = data.splitn(2, '=');
            let key = i.next().unwrap().trim();
            if key.is_empty() { continue }
            let key = urldecode(key).map_err(ParseQueryError::from)?;
            let value = i.next().unwrap_or("").trim();
            let value = urldecode(value).map_err(ParseQueryError::from)?;
            map.insert(key, value);
        }

        Ok(Query(map))
    }
}
