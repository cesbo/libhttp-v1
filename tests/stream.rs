use std::io::Write;
use http::HttpStream;


#[test]
fn test_write_simulate() {
    let mut pos = 0;    /* self.wbuf.pos */
    let mut cap = 0;    /* self.wbuf.cap */
    let len = 32;   /* self.wbuf.buf.len() */

    let test_data: &[usize] = &[10, 10, 10, /* flush */ 10];
    for (i, &buf_len) in test_data.iter().enumerate() {
        if cap + buf_len > len {
            assert_eq!(i, 3);
            assert_eq!(pos, 0);
            assert_eq!(cap, 30);
            pos = 0;
            cap = 0;
        }

        if buf_len >= len {
            ()
        } else {
            cap += buf_len
        }
    }

    assert_eq!(pos, 0);
    assert_eq!(cap, 10);
}


#[test]
fn test_is_ready() {
    let mut stream = HttpStream::default();
    assert!(stream.is_ready());
    stream.connect(true, "example.com", 443).unwrap();
    stream.write_all(concat!("GET / HTTP/1.0\r\n",
        "Host: example.com\r\n",
        "User-Agent: libhttp\r\n",
        "\r\n").as_bytes()).unwrap();
    stream.flush().unwrap();
    assert!(!stream.is_ready());
    stream.close();
}
