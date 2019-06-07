use std::{
    fmt,
    convert::TryFrom,
};

use crate::{
    UrlDecoder,
    UrlEncoder,
};


#[derive(Debug, Error)]
pub enum UrlError {
    #[error_from("Url: {}", 0)]
    Fmt(fmt::Error),
    #[error_kind("Url: length limit")]
    LengthLimit,
    #[error_kind("Url: unexpected relative path")]
    RelativePath,
    #[error_kind("Url: invalid port")]
    InvalidPort,
}


type Result<T> = std::result::Result<T, UrlError>;


/// A parsed URL record
///
/// URL parts: `scheme://prefix@address/path?query#fragment`
/// All url parts are optional.
/// If path, query, and fragment are defined, then value contains their delimiter as well
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Url {
    scheme: String,
    prefix: String,
    host: String,
    port: u16,
    path: String,
    query: String,
    fragment: String,
}


impl Url {
    /// Allocate new object and parse url
    pub fn new<R: AsRef<str>>(input: R) -> Result<Self> {
        let mut url = Url::default();
        url.set(input)?;
        Ok(url)
    }

    /// Set URL
    pub fn set<R: AsRef<str>>(&mut self, input: R) -> Result<()> {
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

        let input = input.as_ref();
        if input.is_empty() { return Ok(()) }
        if input.len() > 2048 { return Err(UrlError::LengthLimit) }

        if let Some(v) = input.find("://") {
            self.scheme.clear();
            self.prefix.clear();
            self.host.clear();
            self.port = 0;
            self.path.clear();
            self.query.clear();
            self.fragment.clear();

            self.scheme.push_str(&input[0 .. v]);
            skip = v + 3;
        } else {
            // TODO: relative url
            if ! input.starts_with('/') { return Err(UrlError::RelativePath) }

            self.path.clear();
            self.query.clear();
            self.fragment.clear();

            step = 2;
        }

        for (idx, part) in input[skip ..].match_indices(|c| {
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

        let mut tail = input.len();

        if fragment > 0 {
            self.fragment.push_str(&input[fragment + 1 .. tail]);
            tail = fragment;
        }

        if query > 0 {
            self.query.push_str(&input[query + 1 .. tail]);
            tail = query;
        }

        if path > 0 || skip == 0 {
            self.path = String::try_from(UrlDecoder::new(&input[path .. tail]))?;
            tail = path;
        }

        if prefix > 0 {
            self.prefix.push_str(&input[skip .. prefix]);
            skip = prefix + 1;
        }

        if skip != 0 {
            let mut addr = input[skip .. tail].splitn(2, ':');
            self.host = addr.next().unwrap().to_string();
            if let Some(port) = addr.next() {
                self.port = port.parse::<u16>().unwrap_or(0);
                if self.port == 0 { return Err(UrlError::InvalidPort) }
            }
        }

        Ok(())
    }

    /// Returns URL scheme
    #[inline]
    pub fn get_scheme(&self) -> &str { &self.scheme }

    /// Returns URL prefix
    #[inline]
    pub fn get_prefix(&self) -> &str { &self.prefix }

    /// Returns URL host
    #[inline]
    pub fn get_host(&self) -> &str { &self.host }

    /// Returns URL port
    #[inline]
    pub fn get_port(&self) -> u16 { self.port }

    /// Returns URL path
    #[inline]
    pub fn get_path(&self) -> &str { &self.path }

    /// Returns URL query
    #[inline]
    pub fn get_query(&self) -> &str { &self.query }

    /// Returns URL fragment
    #[inline]
    pub fn get_fragment(&self) -> &str { &self.fragment }

    /// Returns URL formatter for request_uri - encoded path with query string
    #[inline]
    pub fn as_request_uri(&'_ self) -> UrlFormatter<'_> {
        UrlFormatter::RequestUri(self)
    }

    /// Returns URL formatter for address - host with port if defined
    #[inline]
    pub fn as_address(&'_ self) -> UrlFormatter<'_> {
        UrlFormatter::Address(self)
    }
}


/// Convert Url into required format
pub enum UrlFormatter<'a> {
    /// Host with Port separated by colon. Or only Host if Port not defined
    ///
    /// Host only:
    ///
    /// ```
    /// # use http::Url;
    /// let url = Url::new("https://example.com").unwrap();
    /// assert_eq!(url.as_address().to_string().as_str(), "example.com");
    /// ```
    ///
    /// Host with Port:
    ///
    /// ```
    /// # use http::Url;
    /// let url = Url::new("https://example.com:8000").unwrap();
    /// assert_eq!(url.as_address().to_string().as_str(), "example.com:8000");
    /// ```
    Address(&'a Url),

    /// Path with Query seprated by `?`
    /// Only Path if Query not defined
    ///
    /// Only Path defined:
    ///
    /// ```
    /// # use http::Url;
    /// let url = Url::new("https://example.com/test").unwrap();
    /// assert_eq!(url.as_request_uri().to_string().as_str(), "/test");
    /// ```
    ///
    /// Path with Query:
    ///
    /// ```
    /// # use http::Url;
    /// let url = Url::new("https://example.com/test?query").unwrap();
    /// assert_eq!(url.as_request_uri().to_string().as_str(), "/test?query");
    /// ```
    ///
    /// Only Query defined:
    ///
    /// ```
    /// # use http::Url;
    /// let url = Url::new("https://example.com?query").unwrap();
    /// assert_eq!(url.as_request_uri().to_string().as_str(), "/?query");
    /// ```
    RequestUri(&'a Url),
}


impl<'a> fmt::Display for UrlFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UrlFormatter::Address(url) => {
                if url.port == 0 {
                    write!(f, "{}", &url.host)
                } else {
                    write!(f, "{}:{}", &url.host, url.port)
                }
            }

            UrlFormatter::RequestUri(url) => {
                let path = if url.path.is_empty() { "/" } else { url.path.as_str() };
                let path = UrlEncoder::new_path(path);

                if url.query.is_empty() {
                    write!(f, "{}", path)
                } else {
                    write!(f, "{}?{}", path, &url.query)
                }
            }
        }
    }
}
