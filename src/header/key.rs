// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use std::{
    hash::Hasher,
    collections::hash_map::DefaultHasher,
};


#[derive(Debug, Hash, Eq, PartialEq)]
pub struct HeaderKey(u64);


impl From<&[u8]> for HeaderKey {
    fn from(s: &[u8]) -> HeaderKey {
        let mut hasher = DefaultHasher::new();
        for b in s {
            hasher.write_u8(b.to_ascii_lowercase());
        }
        HeaderKey(hasher.finish())
    }
}


impl From<String> for HeaderKey {
    #[inline]
    fn from(s: String) -> HeaderKey { s.as_bytes().into() }
}


impl From<&String> for HeaderKey {
    #[inline]
    fn from(s: &String) -> HeaderKey { s.as_bytes().into() }
}


impl From<&str> for HeaderKey {
    #[inline]
    fn from(s: &str) -> HeaderKey { s.as_bytes().into() }
}
