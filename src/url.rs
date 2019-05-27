use crate::urldecode::{
    urldecode,
    Error as UrlDecodeError,
};


error_rules! {
    self => ("Url: {}", error),
    UrlDecodeError,
}


/// A parsed URL record
///
/// URL parts: `scheme://prefix@address/path?query#fragment`
/// All url parts are optional.
/// If path, query, and fragment are defined, then value contains their delimiter as well
#[derive(Default, Debug, PartialEq)]
pub struct Url {
    scheme: String,
    prefix: String,
    address: String,        // host:port
    host_len: usize,        // self.address[.. self.host_len]
    port: u16,
    path: String,
    query: String,
    fragment: String,
}


impl Url {
    /// Allocate new object and parse url
    pub fn new(u: &str) -> Result<Self> {
        let mut url = Url::default();
        url.set(u)?;
        Ok(url)
    }

    /// Parse and absolute or relative URL from string
    pub fn set(&mut self, inp: &str) -> Result<()> {
        let mut skip = 0;
        // step values:
        // 0 - prefix
        // 1 - addr (host:port)
        // 2 - /path
        // 3 - ?query
        // 4 - #fragment
        let mut step = 0;
        let mut prefix = 0;
        let mut path = 0;
        let mut query = 0;
        let mut fragment = 0;

        ensure!(!inp.is_empty(), "empty url");
        ensure!(inp.len() <= 2048, "length limit");

        if let Some(v) = inp.find("://") {
            self.scheme.clear();
            self.prefix.clear();
            self.address.clear();
            self.host_len = 0;
            self.port = 0;
            self.path.clear();
            self.query.clear();
            self.fragment.clear();

            self.scheme.push_str(&inp[0 .. v]);
            skip = v + 3;
        } else {
            // TODO: relative url
            ensure!(inp.starts_with('/'), "unexpected relative path");

            self.path.clear();
            self.query.clear();
            self.fragment.clear();

            step = 2;
        }

        for (idx, part) in inp[skip ..].match_indices(|c| {
            c == '/' || c == '?' || c == '#' || c == '@'
        }) {
            match part.as_bytes()[0] {
                b'@' if step < 1 => { prefix = idx + skip; step = 1; },
                b'/' if step < 2 => { path = idx + skip; step = 2; },
                b'?' if step < 3 => { query = idx + skip; step = 3; },
                b'#' if step < 4 => { fragment = idx + skip; break; },
                _ => {},
            };
        }
        let mut tail = inp.len();
        if fragment > 0 {
            self.fragment.push_str(&inp[fragment .. tail]);
            tail = fragment;
        }
        if query > 0 {
            self.query.push_str(&inp[query .. tail]);
            tail = query;
        }
        if path > 0 || skip == 0 {
            self.path = urldecode(&inp[path .. tail])?;
            tail = path;
        }
        if prefix > 0 {
            self.prefix.push_str(&inp[skip .. prefix]);
            skip = prefix + 1;
        }
        if skip != 0 {
            self.address.push_str(&inp[skip .. tail]);
            let address_len = self.address.len();
            self.host_len = self.address.find(':').unwrap_or(address_len);
            if address_len > self.host_len {
                self.port = self.address[self.host_len + 1 ..].parse::<u16>().unwrap_or(0);
                ensure!(self.port > 0, "invalid port");
            }
        }

        Ok(())
    }

    /// Returns url scheme
    #[inline]
    pub fn get_scheme(&self) -> &str { &self.scheme }

    /// Returns url prefix
    #[inline]
    pub fn get_prefix(&self) -> &str { &self.prefix }

    /// Returns url address
    #[inline]
    pub fn get_address(&self) -> &str { &self.address }

    /// Returns url host
    #[inline]
    pub fn get_host(&self) -> &str { &self.address[.. self.host_len] }

    /// Returns url port
    #[inline]
    pub fn get_port(&self) -> u16 { self.port }

    /// Returns url path
    #[inline]
    pub fn get_path(&self) -> &str { &self.path }

    /// Returns url query
    #[inline]
    pub fn get_query(&self) -> &str { &self.query }

    /// Returns url fragment
    #[inline]
    pub fn get_fragment(&self) -> &str { &self.fragment }
}
