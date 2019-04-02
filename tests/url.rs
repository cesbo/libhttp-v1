extern crate http;
use http::url::Url;

#[test]
fn test_url() {
    let mut url = Url::new("http://127.0.0.1:8000/path?query#fragment");
    /*assert_eq!(url.scheme, "http");
    assert_eq!(url.name, "127.0.0.1:8000");
    assert_eq!(url.path, "/path");
    assert_eq!(url.query, "query");
    assert_eq!(url.fragment, "fragment");*/
}