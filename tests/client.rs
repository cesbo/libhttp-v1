use std::io::{
    self,
    Write,
    Read,
    BufRead,
};

use http::HttpClient;


const HELLO_WORLD: &[u8] = b"Hello, world!";


#[test]
fn test_get_content_length() {
    let mut client = HttpClient::new("https://cesbo.com").unwrap();
    client.get().unwrap();
    io::copy(&mut client, &mut io::sink()).unwrap();
}


#[test]
fn test_get_eof() {
    let mut client = HttpClient::new("http://127.0.0.1:9090/get").unwrap();
    client.send().unwrap();
    client.receive().unwrap();

    let mut body = String::new();
    client.read_to_string(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_bytes());
}


#[test]
fn test_post_length() {
    let mut client = HttpClient::new("http://127.0.0.1:9090/post-length").unwrap();
    client.request.set_method("POST");
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
    let mut client = HttpClient::new("http://127.0.0.1:9090/post-chunked").unwrap();
    client.request.set_method("POST");
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
    let mut client = HttpClient::new("http://127.0.0.1:9090/get-chunked-lf-only").unwrap();
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
    let mut client = HttpClient::new("http://127.0.0.1:9090/get-chunked-wo-trailer").unwrap();
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
    let mut client = HttpClient::new("https://httpbin.org/base64/SGVsbG8sIHdvcmxkIQ==").unwrap();
    client.send().unwrap();
    client.receive().unwrap();

    let mut body = String::new();
    client.read_to_string(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_bytes());
}


#[test]
fn test_get_expired_ssl() {
    let mut client = HttpClient::new("https://expired.badssl.com/").unwrap();
    match client.send() {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_get_expired_ssl(): {}", e),
    }
}


#[test]
fn test_get_timeout() {
    let mut client = HttpClient::new("http://httpbin.org/delay/5").unwrap();
    client.send().unwrap();
    match client.receive() {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_get_timeout(): {}", e),
    }
}


#[test]
fn test_invalid_url() {
    match HttpClient::new("http://127.0.0.1:9090/test%QQ") {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_invalid_url(): {}", e),
    }
}


#[test]
fn test_404_without_body() {
    let mut client = HttpClient::new("http://127.0.0.1:9090/404").unwrap();
    assert!(client.get().is_err());
    assert_eq!(404, client.response.get_code());
}


#[test]
fn test_fill_buf() {
    let mut client = HttpClient::new("http://127.0.0.1:9090/get").unwrap();
    client.get().unwrap();

    let buf = client.fill_buf().unwrap();
    assert_eq!(buf, HELLO_WORLD);

    let buf = client.fill_buf().unwrap();
    assert_eq!(buf, HELLO_WORLD);
}


#[test]
fn test_content_length_less() {
    let mut client = HttpClient::new("http://127.0.0.1:9090/test_content_length_less").unwrap();
    client.get().unwrap();

    let mut buf = Vec::new();
    io::copy(&mut client, &mut buf).unwrap();
    assert_eq!(buf.len(), HELLO_WORLD.len() - 1);
}


#[test]
fn test_content_length_more() {
    let mut client = HttpClient::new("http://127.0.0.1:9090/test_content_length_more").unwrap();
    client.get().unwrap();

    let mut buf = Vec::new();
    io::copy(&mut client, &mut buf).unwrap();
    assert_eq!(buf.len(), HELLO_WORLD.len());
}


#[test]
fn test_content_length_exact() {
    let mut client = HttpClient::new("http://127.0.0.1:9090/test_content_length_exact").unwrap();
    client.get().unwrap();

    let mut buf = Vec::new();
    io::copy(&mut client, &mut buf).unwrap();
    assert_eq!(buf.len(), HELLO_WORLD.len());
}
