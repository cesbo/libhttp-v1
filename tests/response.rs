use http::Response;

const TEST1: &str = "HTTP/1.1 200 Ok\r\n\
    Server: libhttp\r\n\
    Date: Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";
    
const TEST2: &str = "RTSP/1.0 200 Ok\r\n\
    Date: Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";

const CODE200: usize = 200;    

#[test]
fn response_read() {    
    let mut response = Response::new();
    response.read(TEST1.as_bytes()).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), &CODE200);
    assert_eq!(response.get_reason(), "Ok");
    assert_eq!(response.get_header("server").unwrap(), "libhttp");
    assert_eq!(response.get_header("date").unwrap(), "Mon, 08 Apr 2019 10:42:12 GMT");
}

#[test]
fn response_send() {
    let mut response = Response::new();
    response.set("Date", "Mon, 08 Apr 2019 10:42:12 GMT");
    response.set_version("RTSP/1.0");
    response.set_code(&CODE200);
    response.set_reason("Ok");
    let mut dst: Vec<u8> = Vec::new();
    response.send(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), TEST2.as_bytes());
}

