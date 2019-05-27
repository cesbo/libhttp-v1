#[macro_use]
extern crate error_rules;

mod auth;

mod header;
pub use crate::header::Header;

mod request;
pub use crate::request::{
    Request,
    Error as RequestError,
};

mod response;
pub use crate::response::{
    Response,
    Error as ResponseError,
};

mod stream;
pub use crate::stream::{
    HttpStream,
    Error as HttpStreamError,
};

mod client;
pub use crate::client::{
    HttpClient,
    Error as HttpClientError,
};

mod urldecode;
pub use crate::urldecode::{
    urldecode,
    Error as UrlDecodeError,
};

mod urlencode;
pub use crate::urlencode::urlencode;

mod url;
pub use crate::url::{
    Url,
    Error as UrlError,
};

mod urlquery;
pub use crate::urlquery::{
    UrlQuery,
    Error as UrlQueryError,
};
