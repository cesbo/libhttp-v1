use std::io;

use http::HttpClient;


#[test]
fn test_auth_basic() {
    let mut client = HttpClient::new();
    client.request.url.set("http://test:testpass@httpbin.org/basic-auth/test/testpass").unwrap();
    client.request.header.set("host", client.request.url.as_address());
    client.request.header.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_simple() {
    let mut client = HttpClient::new();
    client.request.url.set("http://guest:guest@jigsaw.w3.org/HTTP/Digest").unwrap();
    client.request.header.set("host", client.request.url.as_address());
    client.request.header.set("user-agent", "libhttp");

    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(401, client.response.get_code());
    io::copy(&mut client, &mut io::sink()).unwrap();

    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(302, client.response.get_code());
    io::copy(&mut client, &mut io::sink()).unwrap();

    client.request.url.set("/HTTP/Digest/").unwrap();
    for _ in 0 .. 2 {
        client.send().unwrap();
        client.receive().unwrap();
        io::copy(&mut client, &mut io::sink()).unwrap();
        if client.response.get_code() == 200 { break }
    }
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_qop_auth() {
    let mut client = HttpClient::new();
    client.request.url.set("http://guest:test@httpbin.org/digest-auth/auth/guest/test").unwrap();
    client.request.header.set("host", client.request.url.as_address());
    client.request.header.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(401, client.response.get_code());
    io::copy(&mut client, &mut io::sink()).unwrap();
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_auth() {
    let mut client = HttpClient::new();
    client.request.url.set("http://us:testpass@httpbin.org/digest-auth/auth/us/testpass").unwrap();
    client.request.header.set("host", client.request.url.as_address());
    client.request.header.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(401, client.response.get_code());
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(200, client.response.get_code());
}
