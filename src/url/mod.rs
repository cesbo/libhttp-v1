// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU


mod decoder;
mod encoder;
mod query;
mod formatter;

use {
    std::fmt,
    crate::Result,
};


pub use self::{
    decoder::UrlDecoder,
    encoder::UrlEncoder,
    query::UrlQuery,
    formatter::UrlFormatter,
};


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
    /// Allocates new object and parse url
    pub fn new<R: UrlSetter>(input: R) -> Result<Self> {
        let mut url = Url::default();
        input.set_url(&mut url)?;
        Ok(url)
    }

    /// Sets URL or change URL parts
    ///
    /// Supports:
    ///
    /// - complete address - starts with `scheme://`
    /// - full path - starts with `/`. Keeps scheme, prefix, host, and port unchanged
    /// - query - starts with `?`. Keeps all unchanged except query and fragment
    /// - relative path - starts with any other symbol.
    ///   Keeps scheme, prefix, host, and port unchanged.
    ///   Into base path appends relative path. Uses standard rules
    #[inline]
    pub fn set<R: UrlSetter>(&mut self, input: R) -> Result<()> {
        input.set_url(self)
    }

    fn sanitize_path(&mut self) {
        let mut skip = 0;
        let filter_path = |v: &&str| -> bool {
            match *v {
                "" | "." => false,
                ".." => { skip += 1; false }
                _ if skip == 0 => true,
                _ => { skip -= 1; false }
            }
        };

        let mut result = String::with_capacity(self.path.len());
        let concat_path = |v: &&str| {
            result.push('/');
            result.push_str(v);
        };

        // TODO: without Vec
        self.path
            .rsplit('/')
            .filter(filter_path)
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .for_each(concat_path);

        if self.path.ends_with('/') {
            result.push('/')
        }

        self.path = result;
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
    pub fn as_request_uri(&'_ self) -> UrlFormatter<'_> { UrlFormatter::RequestUri(self) }

    /// Returns URL formatter for address - host with port if defined
    #[inline]
    pub fn as_address(&'_ self) -> UrlFormatter<'_> { UrlFormatter::Address(self) }

    /// Returns URL formatter for complete URL
    #[inline]
    pub fn as_url(&'_ self) -> UrlFormatter<'_> { UrlFormatter::Url(self) }
}


/// Interface to set Url from different types
pub trait UrlSetter {
    fn set_url(&self, url: &mut Url) -> Result<()>;
}


impl UrlSetter for &Url {
    #[inline]
    fn set_url(&self, url: &mut Url) -> Result<()> {
        url.clone_from(self);
        Ok(())
    }
}


impl UrlSetter for &str {
    fn set_url(&self, url: &mut Url) -> Result<()> {
        let mut skip = 0;
        // step values:
        // 0 - prefix
        // 1 - addr (host:port)
        // 2 - /path
        // 3 - ?query
        // 4 - #fragment
        let mut step = 0;
        let mut prefix = None;
        let mut path = None;
        let mut query = None;
        let mut fragment = None;

        if self.is_empty() { return Ok(()) }
        ensure!(self.len() < 2048, "URL is to long");

        if let Some(v) = self.find("://") {
            url.scheme.clear();
            url.prefix.clear();
            url.host.clear();
            url.port = 0;
            url.path.clear();
            url.query.clear();
            url.fragment.clear();

            url.scheme.push_str(&self[0 .. v]);
            skip = v + 3;
        } else if self.starts_with('/') {
            path = Some(0);

            url.path.clear();
            url.query.clear();
            url.fragment.clear();

            step = 2;
        } else if self.starts_with('?') {
            query = Some(0);

            url.query.clear();
            url.fragment.clear();

            step = 3;
        } else {
            path = Some(0);

            match url.path.rfind('/') {
                Some(v) => url.path.truncate(v),
                None => url.path.clear(),
            };
            url.path.push('/');

            step = 3;
        }

        for (idx, part) in self[skip ..].match_indices(|c| {
            c == '/' || c == '?' || c == '#' || c == '@'
        }) {
            match part.as_bytes()[0] {
                b'@' if step < 1 => { prefix = Some(idx + skip); step = 1; },
                b'/' if step < 2 => { path = Some(idx + skip); step = 2; },
                b'?' if step < 3 => { query = Some(idx + skip); step = 3; },
                b'#' if step < 4 => { fragment = Some(idx + skip); break; },
                _ => {},
            };
        }

        let mut tail = self.len();

        if let Some(fragment) = fragment {
            url.fragment.push_str(&self[fragment + 1 .. tail]);
            tail = fragment;
        }

        if let Some(query) = query {
            url.query.push_str(&self[query + 1 .. tail]);
            tail = query;
        }

        if let Some(path) = path {
            UrlDecoder::new(&self[path .. tail]).decode(&mut url.path)?;
            url.sanitize_path();
            tail = path;
        }

        if let Some(prefix) = prefix {
            url.prefix.push_str(&self[skip .. prefix]);
            skip = prefix + 1;
        }

        if skip != 0 {
            let mut addr = self[skip .. tail].splitn(2, ':');
            url.host = addr.next().unwrap().to_string();
            if let Some(port) = addr.next() {
                url.port = port.parse::<u16>().unwrap_or(0);
                ensure!(url.port != 0, "invalid port value");
            }
        }

        Ok(())
    }
}


impl UrlSetter for String {
    #[inline]
    fn set_url(&self, url: &mut Url) -> Result<()> {
        self.as_str().set_url(url)
    }
}


impl UrlSetter for &String {
    #[inline]
    fn set_url(&self, url: &mut Url) -> Result<()> {
        self.as_str().set_url(url)
    }
}


impl UrlSetter for &UrlQuery {
    fn set_url(&self, url: &mut Url) -> Result<()> {
        url.query.clear();
        url.fragment.clear();

        fmt::write(&mut url.query, format_args!("{}", self))?;

        Ok(())
    }
}
