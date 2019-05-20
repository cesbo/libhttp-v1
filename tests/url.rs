use http::Url;
use http::urlencode;
use http::urldecode;
use http::Query;


#[test]
fn test_parse_query() {
    let q = Query::new("data=1&string=5&testing_кирилица=&&link=%26%26%26http%3A%2F%2Ffoo%20bar%2F%D1%82%D0%B5%D1%81%D1%82%2F%F0%9F%8D%94%2F?&test=not test string &&").unwrap();
    let link = q.get("link").unwrap();
    assert_eq!(link, "&&&http://foo bar/тест/🍔/?");
}


#[test]
fn test_urlencode() {
    let s = urlencode("http://foo bar/тест/🍔/");
    assert_eq!(s.as_str(), "http%3A%2F%2Ffoo%20bar%2F%D1%82%D0%B5%D1%81%D1%82%2F%F0%9F%8D%94%2F");
}


#[test]
fn test_urldecode() {
    let s = urldecode("http%3A%2F%2Ffoo%20bar%2F%D1%82%D0%B5%D1%81%D1%82%2F%F0%9F%8D%94%2F").unwrap();
    assert_eq!(s.as_str(), "http://foo bar/тест/🍔/");
}


#[test]
fn test_pathdecode() {
    let url = Url::new("http://127.0.0.1/foo%20++bar%2F%D1%82%D0%B5%D1%81%D1%82%2F%F0%9F%8D%94%2F").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "/foo   bar/тест/🍔/");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "");
}


#[test]
fn test_10() {
    let url = Url::new("dvb://#adapter=1&tp=11044:v:44200&type=s2").unwrap();
    assert_eq!(url.get_scheme(), "dvb");
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#adapter=1&tp=11044:v:44200&type=s2");
}


#[test]
fn test_9() {
    let url = Url::new("udp://test:test@239.255.1.1:1234").unwrap();
    assert_eq!(url.get_address(), "239.255.1.1:1234");
    assert_eq!(url.get_scheme(), "udp");
    assert_eq!(url.get_host(), "239.255.1.1");
    assert_eq!(url.get_port(), 1234);
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "");
    assert_eq!(url.get_prefix(), "test:test");
}


#[test]
fn test_8() {
    let url = Url::new("udp://239.255.1.1:1234#pnr=6").unwrap();
    assert_eq!(url.get_scheme(), "udp");
    assert_eq!(url.get_host(), "239.255.1.1");
    assert_eq!(url.get_port(), 1234);
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#pnr=6");
}


#[test]
fn test_7() {
    let url = Url::new("http://127.0.0.1?qwery").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "?qwery");
    assert_eq!(url.get_fragment(), "");
}


#[test]
fn test_6() {
    let url = Url::new("http://127.0.0.1#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#fragment");
}


#[test]
fn test_5() {
    let url = Url::new("http://127.0.0.1?qwery#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "?qwery");
    assert_eq!(url.get_fragment(), "#fragment");
}


#[test]
fn test_4() {
    let url = Url::new("http://127.0.0.1/path%20test").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "/path test");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "");
}


#[test]
fn test_3() {
    let url = Url::new("http://127.0.0.1/path?qwery").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "?qwery");
    assert_eq!(url.get_fragment(), "");
}


#[test]
fn test_2() {
    let url = Url::new("http://127.0.0.1/path#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#fragment");
}


#[test]
fn test_1() {
    let url = Url::new("http://127.0.0.1/path?qwery#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_address(), "127.0.0.1");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "?qwery");
    assert_eq!(url.get_fragment(), "#fragment");
}


#[test]
fn test_url_full() {
    let url = Url::new("http://127.0.0.1:8000/path?query#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_address(), "127.0.0.1:8000");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "?query");
    assert_eq!(url.get_fragment(), "#fragment");
}


#[test]
fn test_url_whithout_query() {
    let url = Url::new("http://127.0.0.1:8000/path#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#fragment");
}


#[test]
fn test_url_whithout_fragment() {
    let url = Url::new("http://127.0.0.1:8000/path?query").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "?query");
    assert_eq!(url.get_fragment(), "");
}


#[test]
fn test_url_whithout_query_fragment() {
    let url = Url::new("http://127.0.0.1:8000/path").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "");
}


#[test]
fn test_url_whithout_path_query_fragment() {
    let url = Url::new("http://127.0.0.1:8000").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "");
}


#[test]
fn test_url_without_scheme() {
    let url = Url::new("/path?query").unwrap();
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "?query");
}
