use std::{
    fmt,
    convert::TryFrom,
    collections::HashMap,
};

use crate::{
    UrlDecoder,
    UrlEncoder,
};


/// Strings in query format - key-value tuples separated by '&'
///
/// Usage:
///
/// ```
/// use http::UrlQuery;
///
/// fn main() {
///     let query = UrlQuery::new("key1=value1&key2=value2").unwrap();
///     assert_eq!(query.get("key1").unwrap(), "value1");
///     assert_eq!(query.get("key2").unwrap(), "value2");
/// }
/// ```
#[derive(Default)]
pub struct UrlQuery(HashMap<String, String>);


impl fmt::Debug for UrlQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


impl UrlQuery {
    pub fn new(query: &str) -> Result<UrlQuery, fmt::Error> {
        let mut map = HashMap::new();

        for data in query.split('&').filter(|s| !s.is_empty()) {
            let mut i = data.splitn(2, '=');
            let key = i.next().unwrap().trim();
            if key.is_empty() { continue }
            let key = String::try_from(UrlDecoder::new(key))?;
            let value = i.next().unwrap_or("").trim();
            let value = String::try_from(UrlDecoder::new(value))?;
            map.insert(key, value);
        }

        Ok(UrlQuery(map))
    }

    /// Sets key-value
    #[inline]
    pub fn set<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: ToString,
    {
        self.0.insert(key.into(), value.to_string());
    }

    /// Returns string value
    #[inline]
    pub fn get<R: AsRef<str>>(&self, key: R) -> Option<&str> {
        self.0.get(key.as_ref()).map(std::string::String::as_str)
    }

    /// Returns pairs iterator
    #[inline]
    pub fn iter(&'_ self) -> UrlQueryIterator<'_> {
        self.into_iter()
    }
}


impl fmt::Display for UrlQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, (key, val)) in self.0.iter().enumerate() {
            if i > 0 {
                fmt::Write::write_char(f, '&')?;
            }
            UrlEncoder::new(key).fmt(f)?;
            fmt::Write::write_char(f, '=')?;
            UrlEncoder::new(val).fmt(f)?;
        }
        Ok(())
    }
}


impl<'a> IntoIterator for &'a UrlQuery {
    type Item = (&'a str, &'a str);
    type IntoIter = UrlQueryIterator<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        UrlQueryIterator {
            inner: self.0.iter(),
        }
    }
}

/// Iterator over query HashMap
pub struct UrlQueryIterator<'a> {
    inner: std::collections::hash_map::Iter<'a, String, String>,
}

impl<'a> Iterator for UrlQueryIterator<'a> {
    type Item = (&'a str, &'a str);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}
