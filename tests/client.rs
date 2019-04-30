use std::str;

use http::HttpClient;

#[test]
fn simple_client() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://httpbin.org/get");
    client.request.set_version("HTTP/1.1");
    client.request.set("User-Agent", "libhttp");
    client.send().unwrap();
    client.receive();
    println!("{:#?}", client.request);
    println!("{:#?}", client.response);
}


#[test]
fn simple_post() {
    let mut client = HttpClient::new();
    client.request.init("POST", "http://127.0.0.1:9090/post");
    client.request.set_version("HTTP/1.1");
    client.request.set("User-Agent", "test");
    client.send().unwrap();
    client.receive();
    println!("{:#?}", client.request);
    println!("{:#?}", client.response);
}
