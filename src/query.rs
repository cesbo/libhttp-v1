use std::{
    fmt,
    collections::HashMap,
};

use crate::urldecode::{
    urldecode,
    Error as UrlDecodeError,
};


error_rules! {
    self => ("Query: {}", error),
    UrlDecodeError,
}


/// Strings in query format - key-value tuples separated by '&'
///
/// Usage:
///
/// ```
/// use http::Query;
///
/// fn main() {
///     let query = Query::new("key1=value1&key2=value2").unwrap();
///     assert_eq!(query.get("key1").unwrap(), "value1");
///     assert_eq!(query.get("key2").unwrap(), "value2");
/// }
/// ```
pub struct Query(HashMap<String, String>);


impl fmt::Debug for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


impl Query {
    pub fn new(query: &str) -> Result<Query> {
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

    /// Returns string value
    #[inline]
    pub fn get<R: AsRef<str>>(&self, key: R) -> Option<&str> {
        self.0.get(key.as_ref()).map(std::string::String::as_str)
    }

    /// Returns pairs iterator
    #[inline]
    pub fn iter<'a>(&'a self) -> QueryIterator<'a> {
        self.into_iter()
    }
}


impl<'a> IntoIterator for &'a Query {
    type Item = (&'a str, &'a str);
    type IntoIter = QueryIterator<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        QueryIterator {
            inner: self.0.iter(),
        }
    }
}

/// Iterator over query HashMap
pub struct QueryIterator<'a> {
    inner: std::collections::hash_map::Iter<'a, String, String>,
}

impl<'a> Iterator for QueryIterator<'a> {
    type Item = (&'a str, &'a str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}
