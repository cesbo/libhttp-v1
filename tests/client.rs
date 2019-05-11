use std::io::{
    Write,
    Read,
    BufRead,
};

use http::HttpClient;


const HELLO_WORLD: &[u8] = b"Hello, world!";


#[test]
fn test_get_eof() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://127.0.0.1:9090/get");
    client.request.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();

    let mut body = String::new();
    client.read_to_string(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_bytes());
}


#[test]
fn test_post_length() {
    let mut client = HttpClient::new();
    client.request.init("POST", "http://127.0.0.1:9090/post-length");
    client.request.set("user-agent", "libhttp");
    client.request.set("content-type", "text/plain");
    client.request.set("content-length", HELLO_WORLD.len());
    client.send().unwrap();
    client.write(HELLO_WORLD).unwrap();
    client.receive().unwrap();

    let mut body = String::new();
    client.read_to_string(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_bytes());
}


#[test]
fn test_post_chunked() {
    let mut client = HttpClient::new();
    client.request.init("POST", "http://127.0.0.1:9090/post-chunked");
    client.request.set("user-agent", "libhttp");
    client.request.set("content-type", "text/plain");
    client.request.set("content-length", HELLO_WORLD.len());
    client.send().unwrap();
    client.write(HELLO_WORLD).unwrap();
    client.receive().unwrap();

    let mut count = 0;
    for line in client.lines() {
        match line {
            Ok(v) => {
                count += 1;
                assert_eq!(v.as_bytes(), HELLO_WORLD);
            }
            _ => unreachable!(),
        };
    }
    assert_eq!(count, 10);
}


#[test]
fn test_get_chunked_lf_only() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://127.0.0.1:9090/get-chunked-lf-only");
    client.request.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();

    let mut count = 0;
    for line in client.lines() {
        match line {
            Ok(v) => {
                count += 1;
                assert_eq!(v.as_bytes(), HELLO_WORLD);
            }
            _ => unreachable!(),
        };
    }
    assert_eq!(count, 10);
}


#[test]
fn test_get_chunked_wo_trailer() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://127.0.0.1:9090/get-chunked-wo-trailer");
    client.request.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();

    let mut count = 0;
    for line in client.lines() {
        match line {
            Ok(v) => {
                count += 1;
                assert_eq!(v.as_bytes(), HELLO_WORLD);
            }
            _ => unreachable!(),
        };
    }
    assert_eq!(count, 10);
}


#[test]
fn test_get_ssl() {
    let mut client = HttpClient::new();
    client.request.init("GET", "https://httpbin.org/base64/SGVsbG8sIHdvcmxkIQ==");
    client.request.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();

    let mut body = String::new();
    client.read_to_string(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_bytes());
}


#[test]
fn test_get_expired_ssl() {
    let mut client = HttpClient::new();
    client.request.init("GET", "https://expired.badssl.com/");
    client.request.set("user-agent", "libhttp");
    match client.send() {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("{}", e),
    }
}


#[test]
fn test_get_timeout() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://httpbin.org/delay/5");
    client.request.set("user-agent", "libhttp");
    client.send().unwrap();
    match client.receive() {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("{}", e),
    }
}
