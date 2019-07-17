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
            }
        ).run();

    let mut client = HttpClient::new("http://test:pass@127.0.0.1:34000").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_simple() {
    Server::new("127.0.0.1:34001")
        .step(
            |request, _reader| {
                assert_eq!(request.header.get("authorization"), Some("Basic Z3Vlc3Q6Z3Vlc3Q="));
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 401 Unauthorized\r\n",
                    "WWW-Authenticate: Digest realm=\"test\", domain=\"/HTTP/Digest/\", nonce=\"9a52e5d50ca0f63e5b0b9188b1e32a15\"\r\n",
                    "Content-Length: 0\r\n",
                    "\r\n"
                ).as_bytes())
            }
        ).step(
            |request, _reader| {
                assert_eq!(request.header.get("authorization"), Some(concat!(
                    "Digest username=\"guest\", ",
                    "uri=\"/HTTP/Digest/\", ",
                    "realm=\"test\", ",
                    "nonce=\"9a52e5d50ca0f63e5b0b9188b1e32a15\", ",
                    "response=\"f3f7ece204d5358cdc207a3807e384ab\"")));
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "Content-Length: 0\r\n",
                    "\r\n"
                ).as_bytes())
            }
        ).run();

    let mut client = HttpClient::new("http://guest:guest@127.0.0.1:34001/HTTP/Digest/").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_qop_auth() {
    let mut client = HttpClient::new("http://guest:guest@httpbin.org/digest-auth/auth/guest/guest").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_auth() {
    let mut client = HttpClient::new("http://guest:guest@httpbin.org/digest-auth/auth/guest/guest").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}
