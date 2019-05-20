use openssl::hash::{
    Hasher,
    MessageDigest,
};
use openssl::rand::rand_bytes;
use base64;

use crate::request::Request;
use crate::response::Response;


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
    let mut ret = String::new();
    for b in bytes.as_ref() {
        ret.push(char::from(HEXMAP[(b >> 4) as usize]));
        ret.push(char::from(HEXMAP[(b & 0x0F) as usize]));
    }
    ret
}


/// This mod implement HTTP authorization


/// Switch authentication type by request code
pub fn auth(request: &mut Request, response: &Response) {
    if request.url.get_prefix().is_empty() {
        return
    }

    match response.get_code() {
        401 => {
            let head = match response.get_header("www-authenticate") {
                Some(v) => v,
                _ => return,
            };
            if head[.. 6].eq_ignore_ascii_case("digest") {
                digest(request, head);
            }
        }
        _ => basic(request),
    }
}


/// Basic access authentication (RFC 2617)
fn basic(request: &mut Request) {
    let value = base64::encode(request.url.get_prefix());
    let value = format!("Basic {}", value);
    request.set_header("authorization", value);
}


/// Digest Access Authentication (RFC 2069)
fn digest(request: &mut Request, head: &String) {
    let mut realm = "";
    let mut nonce = "";
    let mut qop = "";
    let mut opaque = "";

    let mut i = request.url.get_prefix().splitn(2, ':');
    let username = i.next().unwrap_or("");
    let password = i.next().unwrap_or("");

    for data in head[7 ..].split(',') {
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

    let mut h = Hasher::new(MessageDigest::md5()).unwrap();

    [
        username,
        ":",
        realm,
        ":",
        password,
    ].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
    let ha1 = h.finish().unwrap();

    [
        request.get_method(),
        ":",
        request.url.get_path(),
    ].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
    let ha2 = h.finish().unwrap();

    hex2hash(&mut h, &ha1);

    // TODO: tune
    let mut nonce_count = String::new(); // TODO - write fn
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
    let client_nonce = hex2string(&buf);

    match qop {
        "auth" => {
            [
                ":",
                nonce,
                ":",
                &nonce_count,
                ":",
                &client_nonce,
                ":",
                qop,
                ":",
            ].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
        }
        _ => {
            [
                ":",
                nonce,
                ":",
            ].iter().for_each(|s| h.update(s.as_bytes()).unwrap());
        }
    };
    hex2hash(&mut h, &ha2);

    let hr = h.finish().unwrap();
    let hresponse = hex2string(&hr);

    let mut value = format!(concat!("Digest ",
        "username=\"{}\", ",
        "realm=\"{}\", ",
        "nonce=\"{}\", ",
        "uri=\"{}\", ",
        "response=\"{}\""),
        username, realm, nonce, request.url.get_path(), hresponse);

    if ! qop.is_empty() {
        let qop = format!(concat!(", ",
            "qop=\"{}\", ",
            "nc={}, ",
            "cnonce=\"{}\""),
            qop, &nonce_count, &client_nonce);
        value.push_str(&qop);
    }

    if ! opaque.is_empty() {
        let opaque = format!(", opaque=\"{}\"", opaque);
        value.push_str(&opaque);
    }

    request.set_header("authorization", value);
}
