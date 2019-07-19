use std::{
    thread,
    io::{
        Write,
        Read,
        BufRead,
    },
};

use http::HttpClient;

mod support;
use support::Server;


const HELLO_WORLD: &[u8] = b"Hello, world!";


#[test]
fn test_invalid_url() {
    match HttpClient::new("http://127.0.0.1/test%QQ") {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_invalid_url(): {}", e),
    }
}


#[test]
fn test_transfer_persist() {
    Server::new("127.0.0.1:33000")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "\r\n",
                    "Hello, world!"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33000").unwrap();
    client.get().unwrap();

    let mut body = Vec::with_capacity(64);
    client.read_to_end(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_slice());
}


#[test]
fn test_transfer_length() {
    Server::new("127.0.0.1:33001")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "Content-Length: 13\r\n",
                    "\r\n",
                    "Hello, world!"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33001").unwrap();
    client.get().unwrap();

    let mut body = Vec::with_capacity(64);
    client.read_to_end(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_slice());
}


#[test]
fn test_content_length_less() {
    Server::new("127.0.0.1:33002")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "Content-Length: 12\r\n",
                    "\r\n",
                    "Hello, world!"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33002").unwrap();
    client.get().unwrap();

    let mut body = Vec::with_capacity(64);
    client.read_to_end(&mut body).unwrap();
    assert_eq!(&HELLO_WORLD[.. 12], body.as_slice());
}


#[test]
fn test_content_length_more() {
    Server::new("127.0.0.1:33003")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "Content-Length: 14\r\n",
                    "\r\n",
                    "Hello, world!"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33003").unwrap();
    client.get().unwrap();

    let mut body = Vec::with_capacity(64);
    client.read_to_end(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_slice());
}


#[test]
fn test_post() {
    Server::new("127.0.0.1:33004")
        .step(
            |request, reader| {
                let mut buffer = [0; 64];
                let content_length = request.header.get("content-length").unwrap().parse().unwrap();
                reader.read_exact(&mut buffer[.. content_length]).unwrap();
                assert_eq!(HELLO_WORLD, &buffer[.. content_length]);
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "\r\n",
                    "Hello, world!"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33004").unwrap();
    client.request.set_method("POST");
    client.request.header.set("content-type", "text/plain");
    client.request.header.set("content-length", HELLO_WORLD.len());
    client.send().unwrap();
    client.write(HELLO_WORLD).unwrap();
    client.receive().unwrap();

    let mut body = Vec::with_capacity(64);
    client.read_to_end(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_slice());
}


#[test]
fn test_transfer_chunked() {
    Server::new("127.0.0.1:33005")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "Transfer-Encoding: chunked\r\n",
                    "\r\n",
                ).as_bytes())?;

                let s = unsafe { std::str::from_utf8_unchecked(HELLO_WORLD) };
                for _ in 0 .. 10 {
                    writer.write_fmt(format_args!("{:x}\r\n{}\r\n", s.len(), s)).unwrap();
                    writer.flush().unwrap();
                    thread::sleep(std::time::Duration::from_millis(10));
                }
                writer.write_all(b"0\r\n\r\n")
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33005").unwrap();
    client.get().unwrap();

    let mut body = Vec::with_capacity(256);
    client.read_to_end(&mut body).unwrap();

    let mut result = Vec::with_capacity(256);
    for _ in 0 .. 10 {
        result.extend_from_slice(HELLO_WORLD);
    }
    assert_eq!(result, body);
}


#[test]
fn test_get_chunked_lf_only() {
    Server::new("127.0.0.1:33006")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "Transfer-Encoding: chunked\r\n",
                    "\r\n",
                ).as_bytes())?;

                let s = unsafe { std::str::from_utf8_unchecked(HELLO_WORLD) };
                for _ in 0 .. 10 {
                    writer.write_fmt(format_args!("{:x}\n{}\n", s.len(), s)).unwrap();
                    writer.flush().unwrap();
                    thread::sleep(std::time::Duration::from_millis(10));
                }
                writer.write_all(b"0\n\n")
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33006").unwrap();
    client.get().unwrap();

    let mut body = Vec::with_capacity(256);
    client.read_to_end(&mut body).unwrap();

    let mut result = Vec::with_capacity(256);
    for _ in 0 .. 10 {
        result.extend_from_slice(HELLO_WORLD);
    }
    assert_eq!(result, body);
}


#[test]
fn test_get_chunked_wo_trailer() {
    Server::new("127.0.0.1:33007")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "Transfer-Encoding: chunked\r\n",
                    "\r\n",
                ).as_bytes())?;

                let s = unsafe { std::str::from_utf8_unchecked(HELLO_WORLD) };
                for _ in 0 .. 10 {
                    writer.write_fmt(format_args!("{:x}\r\n{}\r\n", s.len(), s)).unwrap();
                    writer.flush().unwrap();
                    thread::sleep(std::time::Duration::from_millis(10));
                }
                writer.write_all(b"0\r\n")
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33007").unwrap();
    client.get().unwrap();

    let mut body = Vec::with_capacity(256);
    client.read_to_end(&mut body).unwrap();

    let mut result = Vec::with_capacity(256);
    for _ in 0 .. 10 {
        result.extend_from_slice(HELLO_WORLD);
    }
    assert_eq!(result, body);
}


#[test]
fn test_get_timeout() {
    Server::new("127.0.0.1:33008")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                thread::sleep(std::time::Duration::from_secs(5));
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "\r\n",
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33008").unwrap();
    match client.get() {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_get_timeout(): {}", e),
    }
}


#[test]
fn test_redirect() {
    Server::new("127.0.0.1:33009")
        .step(
            |request, _reader| {
                assert_eq!(request.url.get_path(), "/redirect/");
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 302 Found\r\n",
                    "Location: http://127.0.0.1:33010/ok/\r\n",
                    "Content-Length: 0\r\n",
                    "\r\n"
                ).as_bytes())
            })
        .run();

    Server::new("127.0.0.1:33010")
        .step(
            |request, _reader| {
                assert_eq!(request.url.get_path(), "/ok/");
                Ok(())
            },
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "\r\n"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33009/redirect/").unwrap();
    client.get().unwrap();
    assert_eq!(200, client.response.get_code());
}


#[test]
fn test_404_without_body() {
    Server::new("127.0.0.1:33011")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 404 Not Found\r\n",
                    "\r\n",
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33011").unwrap();
    assert!(client.get().is_err());
    assert_eq!(404, client.response.get_code());
}


#[test]
fn test_fill_buf() {
    Server::new("127.0.0.1:33012")
        .step(
            |_request, _reader| Ok(()),
            |writer| {
                writer.write_all(concat!(
                    "HTTP/1.1 200 Ok\r\n",
                    "\r\n",
                    "Hello, world!"
                ).as_bytes())
            })
        .run();

    let mut client = HttpClient::new("http://127.0.0.1:33012").unwrap();
    client.get().unwrap();

    let buf = client.fill_buf().unwrap();
    assert_eq!(buf, HELLO_WORLD);

    let buf = client.fill_buf().unwrap();
    assert_eq!(buf, HELLO_WORLD);
}


#[test]
fn test_get_ssl() {
    let mut client = HttpClient::new("https://httpbin.org/base64/SGVsbG8sIHdvcmxkIQ==").unwrap();
    client.send().unwrap();
    client.receive().unwrap();

    let mut body = Vec::with_capacity(64);
    client.read_to_end(&mut body).unwrap();
    assert_eq!(HELLO_WORLD, body.as_slice());
}


#[test]
fn test_get_expired_ssl() {
    let mut client = HttpClient::new("https://expired.badssl.com/").unwrap();
    match client.send() {
        Ok(_) => unreachable!(),
        Err(ref e) => println!("test_get_expired_ssl(): {}", e),
    }
}
