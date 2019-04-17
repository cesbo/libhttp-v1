use http::HttpClient;

#[test]
fn simple_client() {
    let mut client = HttpClient::new();
    client.request.init("GET", "http://httpbin.org/get");
    client.request.set("User-Agent", "libhttp");
	client.connect();
}