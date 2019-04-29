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


#[test]
fn simple_post() {
    let mut client = HttpClient::new();
    client.request.init("POST", "http://127.0.0.1:9090/post");// make 500
    //client.request.init("POST", "http://127.0.0.1:9090");//make 200
    client.request.set_version("HTTP/1.1");
    client.request.set("User-Agent", "test");
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
