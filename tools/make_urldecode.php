<?php
/*
 This script make test for url
*/
$rez = "use http::Url;";
$test_arr = array( 
    "http://foo bar/тест/🍔/",
);

foreach ($test_arr as $k => $v) {
    $code = str_replace("+","%20", urlencode($v));
    $rez .= "
#[test]
fn urlencode_test_".$k."() {
    let url = Url::new(\"\");
    let s = url.urlencode(\"".$v."\");
    assert_eq!(s.as_str(), \"".$code."\");
}

#[test]
fn urldecode_test_".$k."() {
    let url = Url::new(\"\");
    let s = match url.urldecode(\"".$code."\"){
        Some(v) => v,
        None => \"\".to_string(),
    };
    assert_eq!(s, \"".$v."\");
}";
}

print $rez."\n";