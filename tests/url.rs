use std::convert::TryFrom;
use http::{
    Url,
    UrlEncoder,
    UrlDecoder,
    UrlQuery,
};


static ENCODED_URI: &str = "http:%2F%2Ffoo%20bar%2F%D1%82%D0%B5%D1%81%D1%82%2F%F0%9F%8D%94%2F";
static ENCODED_PATH: &str = "http://foo%20bar/%D1%82%D0%B5%D1%81%D1%82/%F0%9F%8D%94/";
static DECODED_URI: &str = "http://foo bar/тест/🍔/";


#[test]
fn test_query_parse() {
    let q = UrlQuery::new(concat!(
        "data=1&",
        "string=5&",
        "testing_кирилица=&",
        "&",
        "link=%26%26%26http%3A%2F%2Ffoo%20bar%2F%D1%82%D0%B5%D1%81%D1%82%2F%F0%9F%8D%94%2F?&",
        "test=not test string &",
        "&")).unwrap();
    let link = q.get("link").unwrap();
    assert_eq!(link, "&&&http://foo bar/тест/🍔/?");
}


#[test]
fn test_query_fmt() {
    let v1 = "test-🍔";
    let v2 = "Test & Value";

    let mut q = UrlQuery::default();
    q.set("data", v1);
    q.set("zzzz", v2);
    let r = q.to_string();
    let q = UrlQuery::new(&r).unwrap();
    let mut step = 0;
    for (k, v) in &q {
        step += 1;
        match k {
            "data" => assert_eq!(v, v1),
            "zzzz" => assert_eq!(v, v2),
            _ => unreachable!(),
        }
    }
    assert_eq!(step, 2);
}


#[test]
fn test_query_iter() {
    let q = UrlQuery::new("key1=value1&key2=value2").unwrap();
    let mut step = 0;
    for (k, v) in &q {
        step += 1;
        match k {
            "key1" => assert_eq!(v, "value1"),
            "key2" => assert_eq!(v, "value2"),
            _ => unreachable!(),
        }
    }
    assert_eq!(step, 2);
}


#[test]
fn test_urlencode() {
    let s = UrlEncoder::new(DECODED_URI).to_string();
    assert_eq!(s.as_str(), ENCODED_URI);
}


#[test]
fn test_urlencode_path() {
    let s = UrlEncoder::new_path(DECODED_URI).to_string();
    assert_eq!(s.as_str(), ENCODED_PATH);
}


#[test]
fn test_urldecode() {
    let s = String::try_from(UrlDecoder::new(ENCODED_URI)).unwrap();
    assert_eq!(s.as_str(), DECODED_URI);
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
    assert_eq!(url.get_fragment(), "adapter=1&tp=11044:v:44200&type=s2");
}


#[test]
fn test_prefix() {
    let url = Url::new("http://test:test@example.com").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.as_address().to_string().as_str(), "example.com");
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
    assert_eq!(url.get_fragment(), "pnr=6");
}


#[test]
fn test_7() {
    let url = Url::new("http://127.0.0.1?query").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "query");
    assert_eq!(url.get_fragment(), "");
    assert_eq!(url.as_request_uri().to_string().as_str(), "/?query");
}


#[test]
fn test_6() {
    let url = Url::new("http://127.0.0.1#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "fragment");
}


#[test]
fn test_5() {
    let url = Url::new("http://127.0.0.1?query#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "query");
    assert_eq!(url.get_fragment(), "fragment");
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
    let url = Url::new("http://127.0.0.1/path?query").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "query");
    assert_eq!(url.get_fragment(), "");
    assert_eq!(url.as_request_uri().to_string().as_str(), "/path?query");
}


#[test]
fn test_2() {
    let url = Url::new("http://127.0.0.1/path#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "fragment");
}


#[test]
fn test_1() {
    let url = Url::new("http://127.0.0.1/path?query#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.as_address().to_string().as_str(), "127.0.0.1");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "query");
    assert_eq!(url.get_fragment(), "fragment");
}


#[test]
fn test_url_full() {
    let url = Url::new("http://127.0.0.1:8000/path?query#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.as_address().to_string().as_str(), "127.0.0.1:8000");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "query");
    assert_eq!(url.get_fragment(), "fragment");
}


#[test]
fn test_url_whithout_query() {
    let url = Url::new("http://127.0.0.1:8000/path#fragment").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "fragment");
}


#[test]
fn test_url_whithout_fragment() {
    let url = Url::new("http://127.0.0.1:8000/path?query").unwrap();
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "query");
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
    assert_eq!(url.get_query(), "query");
}


#[test]
fn test_url_full_path() {
    let mut url = Url::new("https://example.com/test/").unwrap();
    url.set("/full/path/").unwrap();
    assert_eq!(url.get_path(), "/full/path/");
}


#[test]
fn test_url_relative_1() {
    let mut url = Url::new("https://example.com/test").unwrap();
    url.set("relative/path/").unwrap();
    assert_eq!(url.get_path(), "/relative/path/");
}


#[test]
fn test_url_relative_2() {
    let mut url = Url::new("https://example.com/test/").unwrap();
    url.set("relative/path/").unwrap();
    assert_eq!(url.get_path(), "/test/relative/path/");
}


#[test]
fn test_url_relative_3() {
    let mut url = Url::new("https://example.com/").unwrap();
    url.set("relative/path/").unwrap();
    assert_eq!(url.get_path(), "/relative/path/");
}


#[test]
fn test_url_relative_4() {
    let mut url = Url::new("https://example.com").unwrap();
    url.set("relative/path/").unwrap();
    assert_eq!(url.get_path(), "/relative/path/");
}


#[test]
fn test_url_relative_5() {
    let mut url = Url::new("https://example.com/test/xxxx/").unwrap();
    url.set("./relative/path/").unwrap();
    assert_eq!(url.get_path(), "/test/xxxx/relative/path/");
}


#[test]
fn test_url_relative_6() {
    let mut url = Url::new("https://example.com/test/xxxx").unwrap();
    url.set("./relative/path/").unwrap();
    assert_eq!(url.get_path(), "/test/relative/path/");
}


#[test]
fn test_url_relative_7() {
    let mut url = Url::new("https://example.com/test/xxxx/").unwrap();
    url.set("../relative/path/").unwrap();
    assert_eq!(url.get_path(), "/test/relative/path/");
}


#[test]
fn test_url_relative_8() {
    let mut url = Url::new("https://example.com/test/xxxx").unwrap();
    url.set("../relative/path/").unwrap();
    assert_eq!(url.get_path(), "/relative/path/");
}


#[test]
fn test_url_relative_query() {
    let mut url = Url::new("https://example.com/test").unwrap();
    url.set("?query=123").unwrap();
    assert_eq!(url.get_path(), "/test");
    assert_eq!(url.get_query(), "query=123");
}


#[test]
fn test_url_clone() {
    let main = Url::new("https://example.com/play/a001/index.m3u8").unwrap();
    let mut segment = Url::default();
    segment.clone_from(&main);
    segment.set("000000.ts").unwrap();
    assert_eq!(segment.get_host(), "example.com");
    assert_eq!(segment.get_path(), "/play/a001/000000.ts");
}


#[test]
fn test_url_set() {
    let main = Url::new("https://example.com/play/a001/index.m3u8").unwrap();
    let mut segment = Url::default();
    segment.set(&main).unwrap();
    segment.set("000000.ts").unwrap();
    assert_eq!(segment.get_host(), "example.com");
    assert_eq!(segment.get_path(), "/play/a001/000000.ts");
}
