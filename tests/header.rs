use http::Header;


#[test]
fn test_key_cap() {
    let mut result = Vec::<u8>::new();
    let mut h = Header::default();
    h.set("X-Forwarded-For", "test");
    h.send(&mut result).unwrap();
    assert_eq!(&result, b"X-Forwarded-For: test\r\n");
}
