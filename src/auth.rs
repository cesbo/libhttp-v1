use std::fmt::Write;

use openssl::{
    hash::{
        Hasher,
        MessageDigest,
    },
    rand::rand_bytes,
};
use base64;

use crate::{
    request::Request,
    response::Response,
};


static HEXMAP: &[u8] = b"0123456789abcdef";


fn hex2hash<R: AsRef<[u8]>>(h: &mut Hasher, bytes: R) {
    for b in bytes.as_ref() {
        h.update(&[
            HEXMAP[(b >> 4) as usize],
            HEXMAP[(b & 0x0F) as usize],
        ]).unwrap();
    }
}


fn hex2string<R: AsRef<[u8]>>(bytes: R) -> String {
    let bytes = bytes.as_ref();
    let mut ret = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        ret.push(char::from(HEXMAP[(b >> 4) as usize]));
        ret.push(char::from(HEXMAP[(b & 0x0F) as usize]));
    }
    ret
}


/// Switch authentication type by request code
pub fn auth(request: &mut Request, response: &Response) {
    if request.url.get_prefix().is_empty() {
        return
    }

    let token = response.header.get("www-authenticate").unwrap_or("").trim();
    if token.is_empty() {
        basic(request);
        return
    }

    let mut i = token.splitn(2, char::is_whitespace);
    let mode = i.next().unwrap();
    let token = i.next().unwrap_or("");

    if mode.eq_ignore_ascii_case("digest") {
        digest(request, token);
        return
    }

    if mode.eq_ignore_ascii_case("basic") {
        basic(request);
        return
    }

    // TODO: error. unknown method
}


/// Basic access authentication (RFC 2617)
fn basic(request: &mut Request) {
    let value = base64::encode(request.url.get_prefix());
    let value = format!("Basic {}", value);
    request.header.set("authorization", value);
}


/// Digest Access Authentication (RFC 2069)
fn digest(request: &mut Request, token: &str) {
    let mut result = String::with_capacity(1024);

    let mut realm = "";
    let mut nonce = "";
    let mut qop = "";

    let mut i = request.url.get_prefix().splitn(2, ':');
    let username = i.next().unwrap_or("");
    let password = i.next().unwrap_or("");

    write!(result, "Digest username=\"{}\", uri=\"{}\"", username, request.url.get_path()).unwrap();

    for data in token.split(',') {
        let data = data.trim();
        if data.is_empty() {
            continue
        }

        let mut i = data.splitn(2, '=');
        let key = i.next().unwrap().trim();
        if key.is_empty() {
            continue
        }

        let value = i.next().unwrap_or("").trim().trim_matches('"');
        if value.is_empty() {
            continue
        } else if key.eq_ignore_ascii_case("realm") {
            write!(result, ", realm=\"{}\"", value).unwrap();
            realm = value
        } else if key.eq_ignore_ascii_case("nonce") {
            write!(result, ", nonce=\"{}\"", value).unwrap();
            nonce = value
        } else if key.eq_ignore_ascii_case("qop") {
            write!(result, ", qop=\"{}\"", value).unwrap();
            qop = value
        } else if key.eq_ignore_ascii_case("opaque") {
            write!(result, ", opaque=\"{}\"", value).unwrap();
        }
    }

    let mut h = Hasher::new(MessageDigest::md5()).unwrap();

    [
        username,
        ":", realm,
        ":", password,
    ].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
    let ha1 = h.finish().unwrap();

    [
        request.get_method(),
        ":", request.url.get_path(),
    ].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
    let ha2 = h.finish().unwrap();

    hex2hash(&mut h, &ha1);
    [
        ":", nonce,
        ":",
    ].iter().for_each(|s| h.update(s.as_bytes()).unwrap());

    if qop.is_empty() {
        //
    } else if qop == "auth" {
        if request.nonce_count < 99_999_999 {
            request.nonce_count += 1;
        } else {
            request.nonce_count = 0;
        }
        let nonce_count = format!("{:>08}", request.nonce_count);

        let client_nonce = {
            let mut buf = [0; 4];
            rand_bytes(&mut buf).unwrap();
            hex2string(&buf)
        };

        write!(result, ", nc={}, cnonce=\"{}\"",
            &nonce_count, &client_nonce).unwrap();

        [
            &nonce_count,
            ":", &client_nonce,
            ":", qop,
            ":",
        ].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
    }

    hex2hash(&mut h, &ha2);

    let hr = h.finish().unwrap();
    let hresponse = hex2string(&hr);
    write!(result, ", response=\"{}\"", &hresponse).unwrap();

    request.header.set("authorization", result);
}
