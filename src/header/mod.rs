use std::{
    fmt,
    io::{
        self,
        Write,
    },
    collections::HashMap,
};

mod key;
pub use self::key::HeaderKey;

mod pair;
use self::pair::HeaderPair;


/// Set of the headers for HTTP request and response
#[derive(Default)]
pub struct Header(HashMap<HeaderKey, HeaderPair>);


impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(f) }
}


impl Header {
    /// Parses header line
    pub fn parse(&mut self, line: &str) {
        if let Some(skip) = line.find(':') {
            let key = line[.. skip].trim_end();
            if ! key.is_empty() {
                let value = line[skip + 1 ..].trim_start();
                self.0.insert(key.into(), HeaderPair::new(key, value));
            }
        }
    }

    /// Writes header key and value into dst
    pub fn send<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        for (_, pair) in self.0.iter() {
            write!(dst, "{}", pair)?;
        }

        Ok(())
    }

    /// Sets header value
    #[inline]
    pub fn set<K, V>(&mut self, key: K, val: V)
    where
        K: Into<String>,
        V: ToString,
    {
        let key = key.into();
        let hash = key.as_bytes().into();
        self.0.insert(hash, HeaderPair::new(key, val));
    }

    /// Returns reference to the header value value corresponding to the key
    /// Key is case insensitive
    #[inline]
    pub fn get<K>(&self, key: K) -> Option<&str>
    where
        K: Into<HeaderKey>,
    {
        self.0.get(&key.into()).map(|pair| pair.get_value())
    }

    /// Removes all headers
    #[inline]
    pub fn clear(&mut self) { self.0.clear() }
}
