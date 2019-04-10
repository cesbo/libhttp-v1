use http::Response;

const TEST1: &str = "HTTP/1.1 200 Ok\r\n\
    Server: libhttp\r\n\
    Date: Mon, 08 Apr 2019 10:42:12 GMT\r\n\
    \r\n";
    
#[test]
fn response_read() {
    let code200: usize = 200;
    let mut response = Response::new();
    response.read(TEST1.as_bytes()).unwrap();
    assert_eq!(response.get_version(), "HTTP/1.1");
    assert_eq!(response.get_code(), &code200);
}
