extern crate http;
use http::request::Request;


const TEST1: &str = "GET /path?query HTTP/1.1\r\n\
    Host: 127.0.0.1:8000\r\n\
    User-Agent: libhttp\r\n\
    \r\n";

#[test]
fn test_reader_send() {
    let mut request = Request::new();
    request.init("GET", "http://127.0.0.1:8000/path?query");
    request.set("User-Agent", "libhttp");
    println!("========================");
    let mut dst: Vec<u8> = Vec::new();
    request.send(&mut dst);
    println!("{:#?}", TEST1.as_bytes());
    //request.send(&mut dst).unwrap();
    //assert_eq!(&dst, TEST1.as_bytes());
    //assert_eq!(&dst, vec![0, 2, 4, 6]);
}
/*
#[test]
fn test_reader_read() {
    request = Request::new();
    request.read(TEST1.as_bytes()).unwrap();

    assert_eq!(request.get_method(), "GET");
    assert_eq!(request.get_path(), "/path?query");
    assert_eq!(request.headers.get("host"), "127.0.0.1:8000");
    assert_eq!(request.headers.get("user-agent"), "libhttp");
}
*/
