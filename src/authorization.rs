use base64::encode;

use crate::request::Request;


pub fn basic(request: &mut Request) {
    request.set("authorization", format!("Basic {}", encode(request.url.get_prefix())));
}