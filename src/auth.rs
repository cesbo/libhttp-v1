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


pub fn digest(response: &mut Response, stream: &mut stream, request: &mut Request) {
    let qop: i8;
    let realm: str;
    let nonce: str;
    let nonce_count = "00000001"; // TODO - function to calculate
    let client_nonce = "0a4f113b"; // TODO - function to calculate
    let uri = request.url.get_path();
    let mut i = request.url.get_prefix().splitn(2, ':');
    let username = i.next().unwrap("");
    let password = i.next().unwrap("");
    let mut header = match response.get_header("www-authenticate") {
        Some(v) => v.as_str(),
        _ => return ret,
    };
    for data in header[.. 7].split(',') {
        let mut i = data.splitn(2, '=');
        let key = i.next().unwrap();
        if key.is_empty() {
            continue;
        }
        let value = i.next().unwrap_or("");
        match key.trim().to_lowercase(){
            "qop" => {
                qop = match value.trim().to_lowercase() {
                    "auth" => 1,
                    "auth-int" => 2,
                    "auth,auth-int" => 3,
                    _ => 0,
                }
            },
            "realm" => realm = value,
            "nonce" => nonce = value,
        }        
    }
    let a1 = format!("{}:{}:{}", username, realm, password);
    let ha1 = hash_md5(a1);
    let a2 = match qop {
        1 => format("{}:{}", request.get_method(), uri),
        2 => format("{}:{}:{}", request.get_method(), uri, entity_body_md5(stream)),
        3 => format("{}:{}", request.get_method(), uri),
        _ => "",
    }
    let ha2 = hash_md5(a2);
    let response = match { 
        1, 3 => format("{}:{}:{}:{}:auth:{}", ha1, nonce, nonce_count, client_nonce, ha2);
        2 => format("{}:{}:{}:{}:auth-int:{}", ha1, nonce, nonce_count, client_nonce, ha2);
        _ => format("{}:{}:{}", ha1, nonce, ha2);
    }
    let hresponse = hash_md5(response);

    let authorization_head = b"Digest "; 
    write(authorization_head, "username=\"{}\"", username);
    //TODO other headlines 
    request.set("authorization", authorization_head);
}

fn entity_body_md5(stream: &mut stream) -> String { 
    let mut body = String::new();
    stream.read_to_string(&mut body).unwrap();
    hash_md5(body)
}

fn hash_md5(s: str) -> String {  // TODO - fix return type if it need
    hash(MessageDigest::md5(), s).unwrap()
}
