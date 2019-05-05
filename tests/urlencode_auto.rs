use http::Url;
#[test]
fn urlencode_test_0() {
    let url = Url::new("");
    let s = url.urlencode("test some sting");
    assert_eq!(s.as_str(), "test%20some%20sting");
}

#[test]
fn urldecode_test_0() {
    let url = Url::new("");
    let s = match url.urldecode("test%20some%20sting"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, "test some sting");
}
#[test]
fn urlencode_test_1() {
    let url = Url::new("");
    let s = url.urlencode("this is.. ?");
    assert_eq!(s.as_str(), "this%20is..%20%3F");
}

#[test]
fn urldecode_test_1() {
    let url = Url::new("");
    let s = match url.urldecode("this%20is..%20%3F"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, "this is.. ?");
}
#[test]
fn urlencode_test_2() {
    let url = Url::new("");
    let s = url.urlencode("where is %user% ! ? ? ?");
    assert_eq!(s.as_str(), "where%20is%20%25user%25%20%21%20%3F%20%3F%20%3F");
}

#[test]
fn urldecode_test_2() {
    let url = Url::new("");
    let s = match url.urldecode("where%20is%20%25user%25%20%21%20%3F%20%3F%20%3F"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, "where is %user% ! ? ? ?");
}
#[test]
fn urlencode_test_3() {
    let url = Url::new("");
    let s = url.urlencode("@#$%^&*((((((*&^%$#%user%!?   sad ??");
    assert_eq!(s.as_str(), "%40%23%24%25%5E%26%2A%28%28%28%28%28%28%2A%26%5E%25%24%23%25user%25%21%3F%20%20%20sad%20%3F%3F");
}

#[test]
fn urldecode_test_3() {
    let url = Url::new("");
    let s = match url.urldecode("%40%23%24%25%5E%26%2A%28%28%28%28%28%28%2A%26%5E%25%24%23%25user%25%21%3F%20%20%20sad%20%3F%3F"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, "@#$%^&*((((((*&^%$#%user%!?   sad ??");
}
#[test]
fn urlencode_test_4() {
    let url = Url::new("");
    let s = url.urlencode("-----^-");
    assert_eq!(s.as_str(), "-----%5E-");
}

#[test]
fn urldecode_test_4() {
    let url = Url::new("");
    let s = match url.urldecode("-----%5E-"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, "-----^-");
}
#[test]
fn urlencode_test_5() {
    let url = Url::new("");
    let s = url.urlencode(" ^-");
    assert_eq!(s.as_str(), "%20%5E-");
}

#[test]
fn urldecode_test_5() {
    let url = Url::new("");
    let s = match url.urldecode("%20%5E-"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, " ^-");
}
#[test]
fn urlencode_test_6() {
    let url = Url::new("");
    let s = url.urlencode(" ^-some");
    assert_eq!(s.as_str(), "%20%5E-some");
}

#[test]
fn urldecode_test_6() {
    let url = Url::new("");
    let s = match url.urldecode("%20%5E-some"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, " ^-some");
}
#[test]
fn urlencode_test_7() {
    let url = Url::new("");
    let s = url.urlencode(" ^-");
    assert_eq!(s.as_str(), "%20%5E-");
}

#[test]
fn urldecode_test_7() {
    let url = Url::new("");
    let s = match url.urldecode("%20%5E-"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, " ^-");
}
#[test]
fn urlencode_test_8() {
    let url = Url::new("");
    let s = url.urlencode("All_to_659811165565659449");
    assert_eq!(s.as_str(), "All_to_659811165565659449");
}

#[test]
fn urldecode_test_8() {
    let url = Url::new("");
    let s = match url.urldecode("All_to_659811165565659449"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, "All_to_659811165565659449");
}
#[test]
fn urlencode_test_9() {
    let url = Url::new("");
    let s = url.urlencode("test: string");
    assert_eq!(s.as_str(), "test%3A%20string");
}

#[test]
fn urldecode_test_9() {
    let url = Url::new("");
    let s = match url.urldecode("test%3A%20string"){
        Some(v) => v,
        None => "".to_string(),
    };
    assert_eq!(s, "test: string");
}
