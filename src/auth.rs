use base64::encode;
use std::collections::HashMap;

use crate::request::Request;
use crate::response::Response;


pub fn basic(request: &mut Request) {
    request.set("authorization", format!("Basic {}", encode(request.url.get_prefix())));
}


pub fn digest(response: &mut Response) {
    let params = get_digest_params(&response);
}


fn get_digest_params(response: &Response) -> HashMap<String, String> {
    let mut ret = HashMap::new();
    let mut header = match response.get_header("www-authenticate") {
        Some(v) => v.as_str(),
        _ => return ret,
    };
    for data in header[.. 7].split(',') {
        let mut i = data.splitn(2, '=');
        let key = i.next().unwrap();
        if key.is_empty() {
            continue;
        }
        let value = i.next().unwrap_or("");
        ret.insert(key.trim().to_string(), value.trim().to_string()); 
    }
    ret
}