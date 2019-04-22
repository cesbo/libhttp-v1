use http::Url;

#[test]
fn test_10() {
    let url = Url::new("dvb://#adapter=1&tp=11044:v:44200&type=s2");
    assert_eq!(url.get_scheme(), "dvb");
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#adapter=1&tp=11044:v:44200&type=s2");
}
 
#[test]
fn test_9() {
    let name: &str = "239.255.1.1:1234";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("udp://239.255.1.1:1234");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "udp");
    assert_eq!(url.get_host(), "239.255.1.1");
    assert_eq!(url.get_port(), 1234);
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "");
}

#[test]
fn test_8() {
    let name: &str = "239.255.1.1:1234";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("udp://239.255.1.1:1234#pnr=6");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "udp");
    assert_eq!(url.get_host(), "239.255.1.1");
    assert_eq!(url.get_port(), 1234);
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#pnr=6");
}
   
#[test]
fn test_7() {
    let name: &str = "127.0.0.1";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1?qwery");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 80);
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "?qwery");
    assert_eq!(url.get_fragment(), "");
}

#[test]
fn test_6() {
    let name: &str = "127.0.0.1";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1#fragment");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 80);
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#fragment");
}

#[test]
fn test_5() {
    let name: &str = "127.0.0.1";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1?qwery#fragment");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 80);
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "?qwery");
    assert_eq!(url.get_fragment(), "#fragment");
}

#[test]
fn test_4() {
    let name: &str = "127.0.0.1";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1/path");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 80);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "");
}

#[test]
fn test_3() {
    let name: &str = "127.0.0.1";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1/path?qwery");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 80);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "?qwery");
    assert_eq!(url.get_fragment(), "");
}

#[test]
fn test_2() {
    let name: &str = "127.0.0.1";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1/path#fragment");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 80);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#fragment");
}

#[test]
fn test_1() {
    let name: &str = "127.0.0.1";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1/path?qwery#fragment");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 80);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "?qwery");
    assert_eq!(url.get_fragment(), "#fragment");
}

#[test]
fn test_url_full() {
    let name: &str = "127.0.0.1:8000";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1:8000/path?query#fragment");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "?query");
    assert_eq!(url.get_fragment(), "#fragment");
}

#[test]
fn test_url_whithout_query() {
    let name: &str = "127.0.0.1:8000";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1:8000/path#fragment");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "#fragment");
}


#[test]
fn test_url_whithout_fragment() {
    let name: &str = "127.0.0.1:8000";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1:8000/path?query");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "?query");
    assert_eq!(url.get_fragment(), "");
}

#[test]
fn test_url_whithout_query_fragment() {
    let name: &str = "127.0.0.1:8000";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1:8000/path");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "/path");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "");
}

#[test]
fn test_url_whithout_path_query_fragment() {
    let name: &str = "127.0.0.1:8000";
    let mut dst: Vec<u8> = Vec::new();
    let url = Url::new("http://127.0.0.1:8000");
    url.write_header_host(&mut dst).unwrap();
    assert_eq!(dst.as_slice(), name.as_bytes());
    assert_eq!(url.get_scheme(), "http");
    assert_eq!(url.get_host(), "127.0.0.1");
    assert_eq!(url.get_port(), 8000);
    assert_eq!(url.get_path(), "");
    assert_eq!(url.get_query(), "");
    assert_eq!(url.get_fragment(), "");
}
