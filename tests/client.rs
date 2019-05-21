use std::io::{
    self,
    Write,
    Read,
    BufRead,
};

use http::HttpClient;


const HELLO_WORLD: &[u8] = b"Hello, world!";


#[test]
fn test_auth_basic() {
    let mut client = HttpClient::new();
    client.request.url.set("http://test:testpass@httpbin.org/basic-auth/test/testpass").unwrap();
    client.request.header.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_simple() {
    let mut client = HttpClient::new();
    client.request.url.set("http://guest:guest@jigsaw.w3.org/HTTP/Digest/").unwrap();
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
fn test_auth_digest_qop_auth() {
    let mut client = HttpClient::new();
    client.request.url.set("http://guest:test@httpbin.org/digest-auth/auth/guest/test").unwrap();
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
    client.request.header.set("user-agent", "libhttp");
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
    client.request.url.set("http://127.0.0.1:9090/get").unwrap();
    client.request.header.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();

    let mut body = String::new();
    client.read_to_string(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_bytes());
}


#[test]
fn test_post_length() {
    let mut client = HttpClient::new();
    client.request.url.set("http://127.0.0.1:9090/post-length").unwrap();
    client.request.set_method("POST");
    client.request.header.set("user-agent", "libhttp");
    client.request.header.set("content-type", "text/plain");
    client.request.header.set("content-length", HELLO_WORLD.len());
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
    client.request.url.set("http://127.0.0.1:9090/post-chunked").unwrap();
    client.request.set_method("POST");
    client.request.header.set("user-agent", "libhttp");
    client.request.header.set("content-type", "text/plain");
    client.request.header.set("content-length", HELLO_WORLD.len());
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
    client.request.url.set("http://127.0.0.1:9090/get-chunked-lf-only").unwrap();
    client.request.header.set("user-agent", "libhttp");
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
    client.request.url.set("http://127.0.0.1:9090/get-chunked-wo-trailer").unwrap();
    client.request.header.set("user-agent", "libhttp");
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
    client.request.url.set("https://httpbin.org/base64/SGVsbG8sIHdvcmxkIQ==").unwrap();
    client.request.header.set("user-agent", "libhttp");
    client.send().unwrap();
    client.receive().unwrap();

    let mut body = String::new();
    client.read_to_string(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_bytes());
}


#[test]
fn test_get_expired_ssl() {
    let mut client = HttpClient::new();
    client.request.url.set("https://expired.badssl.com/").unwrap();
    client.request.header.set("user-agent", "libhttp");
    match client.send() {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_get_expired_ssl(): {}", e),
    }
}


#[test]
fn test_get_timeout() {
    let mut client = HttpClient::new();
    client.request.url.set("http://httpbin.org/delay/5").unwrap();
    client.request.header.set("user-agent", "libhttp");
    client.send().unwrap();
    match client.receive() {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_get_timeout(): {}", e),
    }
}


#[test]
fn test_invalid_url() {
    let mut client = HttpClient::new();
    match client.request.url.set("http://127.0.0.1:9090/test%QQ") {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_invalid_url(): {}", e),
    }
}
