use http::tools;


#[test]
fn test_bin2hex() {
    let foo: &[u8] = &[0x12, 0x34, 0x0a, 0xb0];
    let mut result = String::new();
    tools::bin2hex(&mut result, foo);
    assert_eq!(result, "12340ab0");
}


#[test]
fn test_hex2bin() {
    let foo = "12340ab0";
    let mut result = Vec::<u8>::new();
    match tools::hex2bin(&mut result, foo.as_bytes()) {
        Ok(_) => {},
        _ => unreachable!(),
    };
    assert_eq!(result, &[0x12, 0x34, 0x0a, 0xb0]);
}


#[test]
fn test_hex2bin_err1() {
    let foo = "12340ab";
    let mut result = Vec::<u8>::new();
    match tools::hex2bin(&mut result, foo.as_bytes()) {
        Err(tools::ParseHexError::Length) => {},
        _ => unreachable!(),
    };
    assert_eq!(result, &[0x12, 0x34, 0x0a]);
}


#[test]
fn test_hex2bin_err2() {
    let foo = "1234?ab0";
    let mut result = Vec::<u8>::new();
    match tools::hex2bin(&mut result, foo.as_bytes()) {
        Err(tools::ParseHexError::Format) => {},
        _ => unreachable!(),
    };
    assert_eq!(result, &[0x12, 0x34]);
}


#[test]
fn test_hex2bin_err3() {
    let foo = "12340?b0";
    let mut result = Vec::<u8>::new();
    match tools::hex2bin(&mut result, foo.as_bytes()) {
        Err(tools::ParseHexError::Format) => {},
        _ => unreachable!(),
    };
    assert_eq!(result, &[0x12, 0x34]);
}
