#[macro_use]
extern crate error_rules;

mod version;
pub use crate::version::HttpVersion;

mod auth;
pub use crate::auth::http_auth;

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

mod client;
pub use crate::client::{
    USER_AGENT,
    HttpClient,
    HttpClientError,
};

mod url;
pub use crate::url::{
    Url,
    UrlFormatter,
    UrlError,
    UrlSetter,
    UrlDecoder,
    UrlEncoder,
    UrlQuery,
};
