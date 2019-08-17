// Copyright (C) 2019 Cesbo OU <info@cesbo.com>
//
// This file is part of ASC/libhttp
//
// ASC/libhttp can not be copied and/or distributed without the express
// permission of Cesbo OU

use std::fmt;


#[derive(Debug)]
pub struct HeaderPair {
    key: String,
    val: String,
}


impl fmt::Display for HeaderPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}\r", &self.key, &self.val)
    }
}


impl HeaderPair {
    pub fn new<K, V>(key: K, val: V) -> HeaderPair
    where
        K: Into<String>,
        V: ToString,
    {
        HeaderPair {
            key: key.into(),
            val: val.to_string(),
        }
    }

    #[inline]
    pub fn get_value(&self) -> &str { self.val.as_str() }
}
