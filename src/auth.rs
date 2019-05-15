use std::collections::HashMap;
use openssl::hash::{
    Hasher, 
    MessageDigest,
};
use base64::encode;

use crate::request::Request;
use crate::response::Response;


pub fn basic(request: &mut Request) {
    request.set("authorization", format!("Basic {}", encode(request.url.get_prefix())));
}


pub fn digest(response: &mut Response, request: &mut Request) {
    let mut realm = "";
    let mut nonce = "";
    let uri = request.url.get_path();
    let mut i = request.url.get_prefix().splitn(2, ':');
    let username = i.next().unwrap_or("");
    let password = i.next().unwrap_or("");
    let header = match response.get_header("www-authenticate") {
        Some(v) => v.as_str(),
        _ => "",
    };
    for data in header[7 ..].split(',') {
        let mut i = data.splitn(2, '=');
        let key = i.next().unwrap();
        if key.is_empty() {
            continue;
        }
        let value = i.next().unwrap_or("");
        if key.trim().eq_ignore_ascii_case("realm") {
            realm = value.trim().trim_matches('\"')
        };
        if key.trim().eq_ignore_ascii_case("nonce") {
            nonce = value.trim().trim_matches('\"')
        };     
    }

    let mut h = DigesHaser::new();
    h.add_str(username);
    h.add_str(realm);
    h.add_str(password);
    let ha1 = h.finish();
    
    h.add_str(request.get_method());
    h.add_str(uri);
    let ha2 = h.finish();

    h.add_byte(&ha1);
    h.add_str(nonce);
    h.add_byte(&ha2);
    let hresponse = h.out_string();

    let authorization_head = format!(concat!("Digest ",
        "username=\"{}\", ",
        "realm=\"{}\", ",
        "nonce=\"{}\", ",
        "uri=\"{}\", ",
        "response=\"{}\""),
        username, realm, nonce, uri, hresponse);
    request.set("authorization", authorization_head);
}

struct DigesHaser {
    h: Hasher,
    empty: bool,
}

impl DigesHaser {
    pub fn new() -> Self { 
        DigesHaser {
            h: Hasher::new(MessageDigest::md5()).unwrap(),
            empty: true,
        }
    }

    pub fn add_str(&mut self, s: &str) {
        self.colon();
        self.h.update(s.as_bytes()).unwrap();
    }

    pub fn add_byte(&mut self, h: &[u8]) {
        static HEXMAP: &[u8] = b"0123456789abcdef";
        for b in h.as_ref() {
            self.h.update(&[
                HEXMAP[(b >> 4) as usize],
                HEXMAP[(b & 0x0F) as usize],
            ]).unwrap();
        }
    }

    fn colon(&mut self) {
        match self.empty {
            true => self.empty = false,
            false => self.h.update(b":").unwrap(),
        };
    }

    pub fn finish(&mut self) -> openssl::hash::DigestBytes {
        self.empty = true;
        self.h.finish().unwrap()
    }

    pub fn out_string(&self) -> String {
        // TODO - write this!
        "".to_string()
    }
}
