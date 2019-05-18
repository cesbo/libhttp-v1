use std::io::{
    Write,
    Read,
    BufRead,
};

use http::HttpClient;


const HELLO_WORLD: &[u8] = b"Hello, world!";


#[test]
fn test_auth_basic() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://test:testpass@httpbin.org/basic-auth/test/testpass");
    client.request.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(200, client.response.get_code());
}

#[test]
fn test_auth_digest_simple() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://guest:guest@jigsaw.w3.org/HTTP/Digest/");
    client.request.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(401, client.response.get_code());
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(200, client.response.get_code());
}

#[test]
fn test_get_eof() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://127.0.0.1:9090/get");
    client.request.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();

    let mut body = String::new();
    client.stream.read_to_string(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_bytes());
}


#[test]
fn test_post_length() {
    let mut client = HttpClient::new();
    client.request.init("POST", "http://127.0.0.1:9090/post-length");
    client.request.set("user-agent", "libhttp");
    client.request.set("content-type", "text/plain");
    client.request.set("content-length", HELLO_WORLD.len().to_string());
    client.send().unwrap();
    client.stream.write(HELLO_WORLD).unwrap();
    client.receive().unwrap();

    let mut body = String::new();
    client.stream.read_to_string(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_bytes());
}


#[test]
fn test_post_chunked() {
    let mut client = HttpClient::new();
    client.request.init("POST", "http://127.0.0.1:9090/post-chunked");
    client.request.set("user-agent", "libhttp");
    client.request.set("content-type", "text/plain");
    client.request.set("content-length", HELLO_WORLD.len().to_string());
    client.send().unwrap();
    client.stream.write(HELLO_WORLD).unwrap();
    client.receive().unwrap();

    let mut count = 0;
    for line in client.stream.lines() {
        match line {
            Ok(v) => {
                count += 1;
                assert_eq!(v.as_bytes(), HELLO_WORLD);
            },
            _ => unreachable!(),
        };
    }
    assert_eq!(count, 10);
}
