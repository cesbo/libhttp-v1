use std::str;

use http::HttpClient;


#[test]
fn simple_client() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://httpbin.org/get");
    client.request.set_version("HTTP/1.1");
    client.request.set("User-Agent", "libhttp");
    let mut dst: Vec<u8> = Vec::new();
    client.request.send(&mut dst).unwrap();
    let s = match str::from_utf8(&dst) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("Header:------------------------");
    println!("{}",s);
    println!("-------------------------------");
    client.connect();
    println!("{:#?}", client.request);
    println!("{:#?}", client.response);
}
