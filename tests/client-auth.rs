use http::HttpClient;


#[test]
fn test_auth_basic() {
    let mut client = HttpClient::new("http://test:testpass@httpbin.org/basic-auth/test/testpass").unwrap();
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_simple() {
    let mut client = HttpClient::new("http://guest:guest@jigsaw.w3.org/HTTP/Digest/").unwrap();

    let mut attempt_auth = 0;
    let mut attempt_redirect = 0;

    loop {
        client.send().unwrap();
        client.receive().unwrap();

        match client.response.get_code() {
            200 => break,
            401 if attempt_auth < 2 => {
                client.flush().unwrap();
                // TODO: check url prefix
                attempt_auth += 1;
            }
            301 | 302 if attempt_redirect < 5 => {
                client.redirect().unwrap();
                attempt_redirect += 1;
                attempt_auth = 0;
            }
            _ => {
                client.flush().unwrap();
                panic!("failed to complete request: {} {}",
                    client.response.get_code(),
                    client.response.get_reason())
            }
        }
    }

    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_qop_auth() {
    let mut client = HttpClient::new("http://guest:test@httpbin.org/digest-auth/auth/guest/test").unwrap();
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(401, client.response.get_code());
    client.flush().unwrap();
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_auth_digest_auth() {
    let mut client = HttpClient::new("http://us:testpass@httpbin.org/digest-auth/auth/us/testpass").unwrap();
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(401, client.response.get_code());
    client.send().unwrap();
    client.receive().unwrap();
    assert_eq!(200, client.response.get_code());
}
