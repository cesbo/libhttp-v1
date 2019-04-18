use http::HttpClient;

#[test]
fn simple_client() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://httpbin.org:80/get");
    client.request.set("User-Agent", "libhttp");
    client.connect();
    println!("{:#?}", client.request);
    println!("{:#?}", client.response);
}
