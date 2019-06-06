use http::HttpClient;


#[test]
fn test_auth_basic() {
    let mut client = HttpClient::new("http://test:testpass@httpbin.org/basic-auth/test/testpass").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_simple() {
    let mut client = HttpClient::new("http://guest:guest@jigsaw.w3.org/HTTP/Digest/").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_qop_auth() {
    let mut client = HttpClient::new("http://guest:test@httpbin.org/digest-auth/auth/guest/test").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_auth() {
    let mut client = HttpClient::new("http://us:testpass@httpbin.org/digest-auth/auth/us/testpass").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}
