use std::io::{
    Write,
};

use http::HttpClient;

mod support;
use support::Server;


#[test]
fn test_auth_basic() {
    Server::new("127.0.0.1:34000")
        .step(
            |request, _reader| {
                assert_eq!(request.header.get("authorization"), Some("Basic dGVzdDpwYXNz"));
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "\r\n"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://test:pass@127.0.0.1:34000").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_simple() {
    Server::new("127.0.0.1:34001")
        .step(
            |request, _reader| {
                assert_eq!(request.header.get("authorization"), Some("Basic dGVzdDpwYXNz"));
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 401 Unauthorized\r\n",
                    "WWW-Authenticate: Digest realm=\"test\", ",
                        "domain=\"/digest/\", ",
                        "nonce=\"9a52e5d50ca0f63e5b0b9188b1e32a15\"\r\n",
                    "Content-Length: 0\r\n",
                    "\r\n"
                ).as_bytes())
            })
        .step(
            |request, _reader| {
                assert_eq!(request.header.get("authorization"), Some(concat!(
                    "Digest username=\"test\", ",
                    "uri=\"/digest/\", ",
                    "realm=\"test\", ",
                    "nonce=\"9a52e5d50ca0f63e5b0b9188b1e32a15\", ",
                    "response=\"a9478b930d1f653a6fe8c27898539a95\"")));
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "\r\n"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://test:pass@127.0.0.1:34001/digest/").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_qop_auth() {
    use openssl::hash::{
        hash,
        MessageDigest,
    };

    Server::new("127.0.0.1:34002")
        .step(
            |request, _reader| {
                assert_eq!(request.header.get("authorization"), Some("Basic dGVzdDpwYXNz"));
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 401 Unauthorized\r\n",
                    "WWW-Authenticate: Digest realm=\"test\", ",
                        "domain=\"/digest/\", ",
                        "nonce=\"9a52e5d50ca0f63e5b0b9188b1e32a15\", ",
                        "qop=\"auth\", ",
                        "opaque=\"e2c06bce29f679e83e0400373593ba62\", ",
                        "algorithm=MD5\r\n",
                    "Content-Length: 0\r\n",
                    "\r\n"
                ).as_bytes())
            })
        .step(
            |request, _reader| {
                let auth = request.header.get("authorization").unwrap();
                let skip = auth.find("cnonce=").unwrap();
                let cnonce = &auth[skip + 8 .. skip + 16];
                let response = format!(concat!("0c26c21ba22ffb6dda57c71ac64aa1d8:",
                    "9a52e5d50ca0f63e5b0b9188b1e32a15:",
                    "00000001:",
                    "{}:",
                    "auth:",
                    "9942091bc79111e32fecde3962416017"), cnonce);
                let response = hash(MessageDigest::md5(), response.as_bytes()).unwrap();
                let response = response.as_ref()
                    .iter()
                    .fold(String::with_capacity(32), |mut acc, &v| {
                        acc.push(std::char::from_digit(u32::from(v >> 4), 16).unwrap());
                        acc.push(std::char::from_digit(u32::from(v & 0x0F), 16).unwrap());
                        acc
                    });

                let result = format!(concat!("Digest username=\"test\", ",
                    "uri=\"/digest/\", ",
                    "realm=\"test\", ",
                    "nonce=\"9a52e5d50ca0f63e5b0b9188b1e32a15\", ",
                    "qop=\"auth\", ",
                    "opaque=\"e2c06bce29f679e83e0400373593ba62\", ",
                    "nc=00000001, ",
                    "cnonce=\"{}\", ",
                    "response=\"{}\""), cnonce, response);

                assert_eq!(request.header.get("authorization"), Some(result.as_str()));
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "\r\n"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://test:pass@127.0.0.1:34002/digest/").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}
