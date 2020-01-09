// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

#[macro_use]
extern crate error_rules;

mod version;
pub use crate::version::HttpVersion;

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
    HttpClient,
    HttpClientError,
    USER_AGENT,
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
