fn basic(&mut request, prefix: &str) {
    request.set("authorization", format!("Basic {}", encode(prefix)));
}