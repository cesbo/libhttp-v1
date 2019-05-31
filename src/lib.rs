#[macro_use]
extern crate error_rules;

mod auth;

mod header;
pub use crate::header::Header;

mod request;
pub use crate::request::{
    Request,
    RequestError,
};

mod response;
pub use crate::response::{
    Response,
    ResponseError,
};

mod stream;
pub use crate::stream::{
    HttpStream,
    HttpStreamError,
};

mod client;
pub use crate::client::{
    HttpClient,
    HttpClientError,
};

mod urldecode;
pub use crate::urldecode::UrlDecoder;

mod urlencode;
pub use crate::urlencode::UrlEncoder;

mod url;
pub use crate::url::{
    Url,
    UrlFormatter,
    UrlError,
};

mod urlquery;
pub use crate::urlquery::UrlQuery;

mod ssl_error;
