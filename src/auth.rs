use std::collections::HashMap;
use openssl::hash::{
    hash, 
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
    for data in header[.. 7].split(',') {
        let mut i = data.splitn(2, '=');
        let key = i.next().unwrap();
        if key.is_empty() {
            continue;
        }
        let value = i.next().unwrap_or("");
        if key.eq_ignore_ascii_case("realm") {
            realm = value
        };
        if key.eq_ignore_ascii_case("nonce") {
            nonce = value
        };     
    }
    let ha1 = hash_md5(format!("{}:{}:{}", username, realm, password));
    let ha2 = hash_md5(format!("{}:{}", request.get_method(), uri));
    let hresponse = hash_md5(format!("{}:{}:{}", ha1, nonce, ha2));
    let authorization_head = format!(concat!("Digest ",
        "username=\"{}\", ",
        "uri=\"{}\", ",
        "response=\"{}\""),
        username, uri, hresponse);
    request.set("authorization", authorization_head);
}


fn hash_md5(s: String) -> String {  // TODO - fix return type if it need
    let data = s.as_bytes();
    let rez = hash(MessageDigest::md5(), data).unwrap();
    String::from_utf8_lossy(&rez).to_string()
}
