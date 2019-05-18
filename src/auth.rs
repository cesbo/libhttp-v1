use std::collections::HashMap;
use openssl::hash::{
    Hasher, 
    MessageDigest,
};
use openssl::rand::rand_bytes;
use base64::encode;

use crate::request::Request;
use crate::response::Response;


/// This mod implement HTTP authorization


/// Switch authentication type by request code
pub fn auth_switch(response: &mut Response, request: &mut Request) {
    if  ! &request.url.get_prefix().is_empty() {
        match *response.get_code() as i32 {
            401 => {
                let head = match &response.get_header("www-authenticate") {
                    Some(v) => v,
                    _ => "",
                };
                if head[.. 6].eq_ignore_ascii_case("digest") {
                    digest(response, request);
                }
            }
            _ => basic(request),
        }
    }
}


/// Basic access authentication (RFC 2617)
pub fn basic(request: &mut Request) {
    request.set("authorization", format!("Basic {}", encode(request.url.get_prefix())));
}


/// Digest Access Authentication (RFC 2069)
pub fn digest(response: &mut Response, request: &mut Request) {
    let mut realm = "";
    let mut nonce = "";
    let mut qop = "";
    let mut opaque = "";
    let mut md5_entitu_body = ""; //TODO write fn md5_entitu_body(body: ?) -> str {...}
    let mut nonce_count = String::new(); // TODO - write fn
    let mut client_nonce = String::new(); // TODO - write fn
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
        if key.trim().eq_ignore_ascii_case("qop") {
            qop = value.trim().trim_matches('\"')
        };
        if key.trim().eq_ignore_ascii_case("opaque") {
            opaque = value.trim().trim_matches('\"')
        };    
    }

    if ! qop.is_empty() {
        if request.nonce_count > 99999998 {
            request.nonce_count = 0;
        }
        request.nonce_count += 1;
        let mut deltax = 99999999;
        while deltax > request.nonce_count {
            deltax = deltax / 10;
            nonce_count += "0";
        }
        nonce_count += &request.nonce_count.to_string();
    }

    let mut buf = [0; 4];
    rand_bytes(&mut buf).unwrap();
    client_nonce = hex2string(&buf);

    let mut h = Hasher::new(MessageDigest::md5()).unwrap();
    [username, ":", realm, ":", password].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
    let ha1 = h.finish().unwrap();
    
    match qop {
        "auth-int" => {
            [request.get_method(), ":", uri, ":", md5_entitu_body].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
        }
        _ => {
            [request.get_method(), ":", uri].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
        }
    };
    let ha2 = h.finish().unwrap();

    update_hex(ha1.as_ref(), &mut h);
    match qop {
        "auth" | "auth-int" => {
            [ ":", nonce, ":", nonce_count.as_str(), ":", client_nonce.as_str(), ":", qop, ":"].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
        }
        _ => {
            [ ":", nonce, ":"].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
        }
    };
    update_hex(ha2.as_ref(), &mut h);

    let hr = h.finish().unwrap();
    let hresponse = hex2string(hr.as_ref());
        
    let mut authorization_head = format!(concat!("Digest ",
        "username=\"{}\", ",
        "realm=\"{}\", ",
        "nonce=\"{}\", ",
        "uri=\"{}\", ",
        "response=\"{}\""),
        username, realm, nonce, uri, hresponse);
    if qop == "auth" || qop == "auth-int" {
        authorization_head += &format! (", qop=\"{}\", nc={}, cnonce=\"{}\"",
            qop, nonce_count.as_str(), client_nonce.as_str());
    }
    if ! opaque.is_empty() {
        authorization_head += &format! (", opaque=\"{}\"", opaque);
    }
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

