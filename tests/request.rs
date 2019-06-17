use std::io::BufReader;
use http::{
    HttpVersion,
    Request,
};

const TEST1: &str = "GET /path?query HTTP/1.1\r\n\
    Host: 127.0.0.1:8000\r\n\
    \r\n";

const TEST_BROKEN: &str = "GET /path?query HTTP/1.1\r\n\
    Host: 127.0.0.1:8000\r\n\
    User-Agent: lib";

const TEST2: &str = "GET rtsp://127.0.0.1:8000/path?query RTSP/1.0\r\n\
    Host: 127.0.0.1:8000\r\n\
    \r\n";

const TEST_TAB: &str = "POST \t\t\t\t\t /path?query     \t\t\t\t\t        RTSP/1.0\r\n\
    Host:\t127.0.0.1:8000\r\n\
    User-Agent:\t libhttp\r\n\
    \r\n";

const TEST_TAB_UNIX: &str = "POST \t\t\t\t\t /path?query     \t\t\t\t\t       RTSP/1.0\n\
    Host:\t127.0.0.1:8000\n\
    User-Agent:\t libhttp\n\
    \n";


#[test]
fn request_send() {
    let mut request = Request::new();
    request.url.set("rtsp://127.0.0.1:8000/path?query").unwrap();
    request.set_version(HttpVersion::RTSP10);
    request.header.set("Host", request.url.as_address());
    let mut dst: Vec<u8> = Vec::new();
    request.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST2.as_bytes());
}


#[test]
fn request_send_version() {
    let mut request = Request::new();
    request.url.set("http://127.0.0.1:8000/path?query").unwrap();
    request.header.set("Host", request.url.as_address());
    let mut dst: Vec<u8> = Vec::new();
    request.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST1.as_bytes());
}


#[test]
fn request_parse() {
    let mut request = Request::new();
    request.parse(&mut BufReader::new(TEST1.as_bytes())).unwrap();
    assert_eq!(request.get_method(), "GET");
    assert_eq!(request.header.get("host").unwrap(), "127.0.0.1:8000");
}

#[test]
fn request_broken() {
    let mut request = Request::new();
    assert!(request.parse(&mut BufReader::new(TEST_BROKEN.as_bytes())).is_err());
}

#[test]
fn request_tab() {
    let mut request = Request::new();
    request.parse(&mut BufReader::new(TEST_TAB.as_bytes())).unwrap();
    assert_eq!(request.get_method(), "POST");
    assert_eq!(request.get_version(), HttpVersion::RTSP10);
    assert_eq!(request.header.get("host").unwrap(), "127.0.0.1:8000");
    assert_eq!(request.header.get("user-agent").unwrap(), "libhttp");
}

#[test]
fn request_unix() {
    let mut request = Request::new();
    request.parse(&mut BufReader::new(TEST_TAB_UNIX.as_bytes())).unwrap();
    assert_eq!(request.get_method(), "POST");
    assert_eq!(request.get_version(), HttpVersion::RTSP10);
    assert_eq!(request.url.get_query(), "query");
    assert_eq!(request.url.get_path(), "/path");
    assert_eq!(request.header.get("host").unwrap(), "127.0.0.1:8000");
    assert_eq!(request.header.get("user-agent").unwrap(), "libhttp");
}
