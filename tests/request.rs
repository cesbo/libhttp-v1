use std::io::BufReader;
use http::Request;

const TEST1: &str = "GET /path?query HTTP/1.1\r\n\
    Host: 127.0.0.1:8000\r\n\
    \r\n";

const TEST_BROKEN: &str = "GET /path?query HTTP/1.1\r\n\
    Host: 127.0.0.1:8000\r\n\
    User-Agent: lib";

const TEST2: &str = "GET /path?query RTSP/1.0\r\n\
    Host: 127.0.0.1:8000\r\n\
    \r\n";

const TEST_TAB: &str = "POST \t\t\t\t\t /path?query     \t\t\t\t\t        RTSP/1.3\r\n\
    Host:\t127.0.0.1:8000\r\n\
    User-Agent:\t libhttp\r\n\
    \r\n";

const TEST_TAB_UNIX: &str = "POST \t\t\t\t\t /path?query     \t\t\t\t\t       RTSP/1.0\n\
    Host:\t127.0.0.1:8000\n\
    User-Agent:\t libhttp\n\
    \n";


#[test]
fn send() {
    let mut request = Request::new();
    request.url.set("http://127.0.0.1:8000/path?query").unwrap();
    request.set_version("RTSP/1.0");
    request.header.set("host", request.url.get_address());
    let mut dst: Vec<u8> = Vec::new();
    request.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST2.as_bytes());
}


#[test]
fn send_version() {
    let mut request = Request::new();
    request.url.set("http://127.0.0.1:8000/path?query").unwrap();
    request.header.set("host", request.url.get_address());
    let mut dst: Vec<u8> = Vec::new();
    request.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST1.as_bytes());
}


#[test]
fn parseer_parse() {
    let mut request = Request::new();
    request.parse(&mut BufReader::new(TEST1.as_bytes())).unwrap();
    assert_eq!(request.get_method(), "GET");
    assert_eq!(request.header.get("host").unwrap(), "127.0.0.1:8000");
}

#[test]
fn parseer_broken() {
    let mut request = Request::new();
    assert!(request.parse(&mut BufReader::new(TEST_BROKEN.as_bytes())).is_err());
}

#[test]
fn parseer_tab() {
    let mut request = Request::new();
    request.parse(&mut BufReader::new(TEST_TAB.as_bytes())).unwrap();
    assert_eq!(request.get_method(), "POST");
    assert_eq!(request.get_version(), "RTSP/1.3");
    assert_eq!(request.header.get("host").unwrap(), "127.0.0.1:8000");
    assert_eq!(request.header.get("user-agent").unwrap(), "libhttp");
}

#[test]
fn parseer_unix() {
    let mut request = Request::new();
    request.parse(&mut BufReader::new(TEST_TAB_UNIX.as_bytes())).unwrap();
    assert_eq!(request.get_method(), "POST");
    assert_eq!(request.get_version(), "RTSP/1.0");
    assert_eq!(request.url.get_query(), "?query");
    assert_eq!(request.url.get_path(), "/path");
    assert_eq!(request.header.get("host").unwrap(), "127.0.0.1:8000");
    assert_eq!(request.header.get("user-agent").unwrap(), "libhttp");
}
