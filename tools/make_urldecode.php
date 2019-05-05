<?php
/*
 This script forms tests/urlencode_auto.rs
*/
$rez = "use http::Url;";
$test_arr = array( 
    "test some sting",
    "this is.. ?",
    "where is %user% ! ? ? ?",
    "@#$%^&*((((((*&^%$#%user%!?   sad ??",
    "-----^-",
    " ^-",
    " ^-some",
    " ^-",
    "All_to_659811165565659449",
    "test: string",
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