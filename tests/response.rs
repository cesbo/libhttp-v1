use std::io::BufReader;
use http::Response;

const TEST1: &str = "HTTP/1.1 200 Ok\r\n\
    Server: libhttp\r\n\
    Date: Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";

const TEST2: &str = "RTSP/1.0 200 Ok\r\n\
    Date: Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";

const TEST_SEND_CASE: &str = "RTSP/1.0 200 Ok\r\n\
    Date-Start: Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";

const TEST_INVALID_CODE: &str = "RTSP/1.0 200Ok\r\n\
    \r\n";

const TEST_UNEXPECTED_EOF: &str = "RTSP/1.0 404   ";

const TEST_WO_STATUS: &str = "HTTP/1.1 200\r\n\
    Server: libhttp\r\n\
    \r\n";

const TEST_UNIX: &str = "HTTP/1.1 200 Ok\n\
    Server: libhttp\n\
    Date: Mon, 08 Apr 2019 10:42:12 GMT\n\
    \n";

const TEST_SPACES: &str = "HTTP/1.1      200 Ok\r\n\
    Server:     libhttp\r\n\
    Date:       Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";

const TEST_TABS: &str = "HTTP/1.1  \t\t\t    200 Ok\r\n\
    Server: \t    libhttp\r\n\
    Date:     \t  Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";

const TEST_TABS_CASE: &str = "HTTP/1.1  \t\t\t    200 Ok\r\n\
    Server-Name: \t    libhttp\r\n\
    Date_date:     \t  Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";

#[test]
fn response_parse() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST1.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), 200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server").unwrap(), "libhttp");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}


#[test]
fn response_parse_spaces() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST_SPACES.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), 200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server").unwrap(), "libhttp");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}


#[test]
fn response_parse_tabs() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST_TABS.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), 200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server").unwrap(), "libhttp");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}


#[test]
fn response_parse_tabs_case() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST_TABS_CASE.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), 200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server-name").unwrap(), "libhttp");
    assert_eq!(response.get_header("date_date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}


#[test]
fn response_parse_unix() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST_UNIX.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), 200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server").unwrap(), "libhttp");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}


#[test]
fn response_parse_bad_code() {
    let mut response = Response::new();
    assert!(response.parse(&mut BufReader::new(TEST_INVALID_CODE.as_bytes())).is_err());
}


#[test]
fn response_parse_unexpected_eof() {
    let mut response = Response::new();
    assert!(response.parse(&mut BufReader::new(TEST_UNEXPECTED_EOF.as_bytes())).is_err());
}


#[test]
fn response_parse_wo_status() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST_WO_STATUS.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), 200);
    assert!(response.get_reason().is_empty());
}


#[test]
fn response_send() {
    let mut response = Response::new();
    response.set_header("date", "Mon, 08 Apr 2019 10:42:12 GMT");
    response.set_version("RTSP/1.0");
    response.set_code(200);
    response.set_reason("Ok");
    let mut dst: Vec<u8> = Vec::new();
    response.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST2.as_bytes());
}


#[test]
fn response_send_case() {
    let mut response = Response::new();
    response.set_header("date-start", "Mon, 08 Apr 2019 10:42:12 GMT");
    response.set_version("RTSP/1.0");
    response.set_code(200);
    response.set_reason("Ok");
    let mut dst: Vec<u8> = Vec::new();
    response.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST_SEND_CASE.as_bytes());
}
