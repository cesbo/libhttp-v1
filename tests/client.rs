use std::io::Write;

use http::HttpClient;


#[test]
fn simple_client() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://httpbin.org/get");
    client.request.set_version("HTTP/1.1");
    client.request.set("User-Agent", "libhttp");
    client.send().unwrap();
    let _result = client.receive();
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
    let _result = client.receive();
    println!("{:#?}", client.request);
    println!("{:#?}", client.response);
}

#[test]
fn client_post() {
    let data = b"Hello, world!";
    let mut client = HttpClient::new();
    client.request.init("POST", "http://httpbin.org/post");
    client.request.set_version("HTTP/1.1");
    client.request.set("user-agent", "test");
    client.request.set("content-type", "text/plain");
    client.request.set("content-length", data.len().to_string());
    client.send().unwrap();
    println!("{:#?}", &client.request);
    client.write(data).unwrap();
    client.receive().unwrap();
    println!("{:#?}", &client.response);

    let length: usize = client.response.get_header("content-length").unwrap().parse().unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(length, 0);
    //client.read_exact(&mut buffer).unwrap();
}