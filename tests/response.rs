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

const TEST3: &str = "RTSP/1.0 200Ok\r\n\
    Date: Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";

const TEST4: &str = "RTSP/1.0 404   ";

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

const CODE200: usize = 200;
const CODE404: usize = 404;
const CODE0: usize = 0;

#[test]
fn response_parse() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST1.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), &CODE200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server").unwrap(), "libhttp");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}

#[test]
fn response_parse_spaces() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST_SPACES.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), &CODE200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server").unwrap(), "libhttp");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}

#[test]
fn response_parse_tabs() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST_TABS.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), &CODE200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server").unwrap(), "libhttp");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}

#[test]
fn response_parse_tabs_case() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST_TABS_CASE.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), &CODE200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server-name").unwrap(), "libhttp");
    assert_eq!(response.get_header("date_date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}

#[test]
fn response_parse_unix() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST_UNIX.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), &CODE200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server").unwrap(), "libhttp");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}

#[test]
fn response_parse_bad_code() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST3.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "RTSP/1.0");
    assert_eq!(response.get_code(), &CODE0);
    assert_eq!(response.get_reason(), "");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}


#[test]
fn response_parse_test4() {
    let mut response = Response::new();
    response.parse(&mut BufReader::new(TEST4.as_bytes())).unwrap();
    assert_eq!(response.get_version(), "RTSP/1.0");
    assert_eq!(response.get_code(), &CODE404);
}

#[test]
fn response_send() {
    let mut response = Response::new();
    response.set("Date", "Mon, 08 Apr 2019 10:42:12 GMT");
    response.set_version("RTSP/1.0");
    response.set_code(CODE200);
    response.set_reason("Ok");
    let mut dst: Vec<u8> = Vec::new();
    response.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST2.as_bytes());
}

#[test]
fn response_send_case() {
    let mut response = Response::new();
    response.set("date-start", "Mon, 08 Apr 2019 10:42:12 GMT");
    response.set_version("RTSP/1.0");
    response.set_code(CODE200);
    response.set_reason("Ok");
    let mut dst: Vec<u8> = Vec::new();
    response.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST_SEND_CASE.as_bytes());
}
