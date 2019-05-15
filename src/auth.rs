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
    h.add(&[username, realm, password]);
    let ha1 = h.finish();
    h.add(&[request.get_method(), uri]);
    let ha2 = h.finish();
    //h.add(&[&ha1, nonce, &ha2]); TODO - fix this may be work
    h.add(&[&ha1]);
    h.add(&[nonce]);
    h.add(&[&ha2]);
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


static HEXMAP: &[u8] = b"0123456789abcdef";


pub struct DigesHaser {
    h: Hasher,
    empty: bool,
}


pub trait PushHex {
    fn push_hex(&self, h: &mut Hasher);
}


impl PushHex for &str {
    fn push_hex(&self, h: &mut Hasher) {
        h.update(self.as_bytes()).unwrap();
    }
}


impl PushHex for &openssl::hash::DigestBytes {
    fn push_hex(&self, h: &mut Hasher) {
        for b in self.as_ref() {
            h.update(&[
                HEXMAP[(b >> 4) as usize],
                HEXMAP[(b & 0x0F) as usize],
            ]).unwrap();
        }
    }
} 


impl DigesHaser {
    pub fn new() -> Self { 
        DigesHaser {
            h: Hasher::new(MessageDigest::md5()).unwrap(),
            empty: true,
        }
    }

    pub fn add<T>(&mut self, hvec: &[T]) 
    where
        T: PushHex    
    {
        match self.empty {
            true => self.empty = false,
            false => self.h.update(b":").unwrap(),
        };
        for a in hvec {
            a.push_hex(&mut self.h);
        }
    }

    pub fn finish(&mut self) -> openssl::hash::DigestBytes {
        self.empty = true;
        self.h.finish().unwrap()
    }

    pub fn out_string(&mut self) -> String {
        let rez = self.finish();
        let mut ret = String::new();
        for b in rez.as_ref() {
            ret.push(char::from(HEXMAP[(b >> 4) as usize]));
            ret.push(char::from(HEXMAP[(b & 0x0F) as usize]));
        }
        ret
    }
}