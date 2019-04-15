use http::Request;

const TEST1: &str = "GET /path?query HTTP/1.1\r\n\
    Host: 127.0.0.1:8000\r\n\
    User-Agent: libhttp\r\n\
    \r\n";
    
const TEST_BROKEN: &str = "GET /path?query HTTP/1.1\r\n\
    Host: 127.0.0.1:8000\r\n\
    User-Agent: lib";

const TEST2: &str = "GET /path?query RTSP/1.0\r\n\
    Host: 127.0.0.1:8000\r\n\
    User-Agent: libhttp\r\n\
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
    request.init("GET", "http://127.0.0.1:8000/path?query");
    request.set("User-Agent", "libhttp");
    request.set_version("RTSP/1.0");
    let mut dst: Vec<u8> = Vec::new();
    request.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST2.as_bytes());
}

#[test]
fn send_case() {
    let mut request = Request::new();
    request.init("GET", "http://127.0.0.1:8000/path?query");
    request.set("user-agent", "libhttp");
    request.set_version("RTSP/1.0");
    let mut dst: Vec<u8> = Vec::new();
    request.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST2.as_bytes());
}


#[test]
fn parseer_parse() {
    let mut request = Request::new();
    request.parse(TEST1.as_bytes()).unwrap();
    assert_eq!(request.get_method(), "GET");
    assert_eq!(request.get_header("host").unwrap(), "127.0.0.1:8000");
    assert_eq!(request.get_header("user-agent").unwrap(), "libhttp");
}

#[test]
fn parseer_broken() {
    let mut request = Request::new();
    request.parse(TEST_BROKEN.as_bytes()).unwrap();
    assert_eq!(request.get_method(), "GET");
    assert_eq!(request.get_header("host").unwrap(), "127.0.0.1:8000");
    assert_eq!(request.get_header("user-agent").unwrap(), "lib");
}

#[test]
fn parseer_tab() {
    let mut request = Request::new();
    request.parse(TEST_TAB.as_bytes()).unwrap();
    assert_eq!(request.get_method(), "POST");
    assert_eq!(request.get_version(), "RTSP/1.3");
    assert_eq!(request.get_header("host").unwrap(), "127.0.0.1:8000");
    assert_eq!(request.get_header("user-agent").unwrap(), "libhttp");
}


#[test]
fn parseer_unix() {
    let mut request = Request::new();
    request.parse(TEST_TAB_UNIX.as_bytes()).unwrap();
    assert_eq!(request.get_method(), "POST");
    assert_eq!(request.get_version(), "RTSP/1.0");
    assert_eq!(request.get_query(), "?query");
    assert_eq!(request.get_path(), "/path");
    assert_eq!(request.get_header("host").unwrap(), "127.0.0.1:8000");
    assert_eq!(request.get_header("user-agent").unwrap(), "libhttp");
}


