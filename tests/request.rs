use http::Request;

const TEST1: &str = "GET /path?query HTTP/1.1\r\n\
    Host: 127.0.0.1:8000\r\n\
    User-Agent: libhttp\r\n\
    \r\n";

const TEST2: &str = "GET /path?query RTSP/1.0\r\n\
    Host: 127.0.0.1:8000\r\n\
    User-Agent: libhttp\r\n\
    \r\n";


#[test]
fn test_reader_send() {
    let mut request = Request::new();
    request.init("GET", "http://127.0.0.1:8000/path?query");
    request.set("User-Agent", "libhttp");
    request.set_version("RTSP/1.0");
    let mut dst: Vec<u8> = Vec::new();
    request.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST2.as_bytes());
}

#[test]
fn test_reader_read() {
    let mut request = Request::new();
    request.read(TEST1.as_bytes()).unwrap();
    assert_eq!(request.get_method(), "GET");
    //assert_eq!(request.headers_get("host"), "127.0.0.1:8000");
    //assert_eq!(request.headers_get("user-agent"), "libhttp");
}

