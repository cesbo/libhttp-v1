use std::collections::HashMap;
use openssl::hash::{
    Hasher, 
    MessageDigest,
};
use base64::encode;

use crate::request::Request;
use crate::response::Response;


/// This mod implement HTTP authorization
///
/// Basic access authentication (RFC 2617)
pub fn basic(request: &mut Request) {
    request.set("authorization", format!("Basic {}", encode(request.url.get_prefix())));
}


/// Digest Access Authentication (RFC 2069)
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

    let mut h = Hasher::new(MessageDigest::md5()).unwrap();
    h.update(username.as_bytes()).unwrap();
    h.update(b":").unwrap();
    h.update(realm.as_bytes()).unwrap();
    h.update(b":").unwrap();
    h.update(password.as_bytes()).unwrap();
    let ha1 = h.finish().unwrap();

    h.update(request.get_method().as_bytes()).unwrap();
    h.update(b":").unwrap();
    h.update(uri.as_bytes()).unwrap();
    let ha2 = h.finish().unwrap();

    update_hex(ha1.as_ref(), &mut h);
    h.update(b":").unwrap();
    h.update(nonce.as_bytes()).unwrap();
    h.update(b":").unwrap();
    update_hex(ha2.as_ref(), &mut h);

    let hr = h.finish().unwrap();
    let hresponse = hex2string(hr.as_ref());
        
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


fn update_hex(bytes: &[u8], h: &mut Hasher) {
    for b in bytes {
        h.update(&[
            HEXMAP[(b >> 4) as usize],
            HEXMAP[(b & 0x0F) as usize],
        ]).unwrap();
    }
}


pub fn hex2string(bytes: &[u8]) -> String {
    let mut ret = String::new();
    for b in bytes {
        ret.push(char::from(HEXMAP[(b >> 4) as usize]));
        ret.push(char::from(HEXMAP[(b & 0x0F) as usize]));
    }
    ret
}
