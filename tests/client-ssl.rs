use std::io::Read;

use http::HttpClient;

mod support;
use support::HELLO_WORLD;


#[test]
fn test_get_ssl() {
    let mut client = HttpClient::new("https://httpbin.org/base64/SGVsbG8sIHdvcmxkIQ==").unwrap();
    client.get().unwrap();

    let mut body = Vec::with_capacity(64);
    client.read_to_end(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_slice());
}


#[test]
fn test_get_expired_ssl() {
    let mut client = HttpClient::new("https://expired.badssl.com/").unwrap();
    match client.send() {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_get_expired_ssl(): {}", e),
    }
}
