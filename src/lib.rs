// Copyright (C) 2019-2020 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU


#[macro_use]
mod error;
mod version;
mod header;
mod request;
mod response;
mod client;
mod url;
mod parser;


pub use crate::{
    error::Result,

    version::HttpVersion,
    header::Header,
    request::Request,
    response::Response,


    client::{
        HttpClient,
        USER_AGENT,
    },

    url::{
        Url,
        UrlFormatter,
        UrlSetter,
        UrlDecoder,
        UrlEncoder,
        UrlQuery,
    },
};
