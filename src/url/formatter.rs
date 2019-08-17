// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use std::fmt;

use super::{
    Url,
    UrlEncoder,
};

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

    /// Complete URL with scheme, host, port, path, and query
    Url(&'a Url),
}


impl<'a> fmt::Display for UrlFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UrlFormatter::Address(url) => {
                write!(f, "{}", &url.host)?;
                if url.port != 0 {
                    write!(f, ":{}", url.port)?;
                }
            }

            UrlFormatter::RequestUri(url) => {
                let path = if url.path.is_empty() { "/" } else { url.path.as_str() };
                let path = UrlEncoder::new_path(path);

                write!(f, "{}", path)?;
                if ! url.query.is_empty() {
                    write!(f, "?{}", &url.query)?;
                }
            }

            UrlFormatter::Url(url) => {
                write!(f, "{}://{}", &url.scheme, &url.host)?;
                if url.port != 0 {
                    write!(f, ":{}", url.port)?;
                }

                if ! url.path.is_empty() {
                    let path = UrlEncoder::new_path(url.path.as_str());
                    write!(f, "{}", path)?;
                }

                if ! url.query.is_empty() {
                    write!(f, "?{}", &url.query)?;
                }
            }
        }

        Ok(())
    }
}
