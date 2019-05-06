
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
fn test_fill_chunked_simulate_1() {
    let buf = b"skip\r\n1234\r\nxxxxx";
    let mut pos = 6;
    let cap = buf.len();

    let mut b = String::new();
    while pos < cap && buf[pos] != b'\n' {
        b.push(char::from(buf[pos]));
        pos += 1;
    }
    pos += 1;

    let chunk_len = b.trim();
    assert_eq!(chunk_len, "1234");
    assert_eq!(&buf[pos .. cap], b"xxxxx");
}


#[test]
fn test_fill_chunked_simulate_2() {
    let mut step = 0;
    let buf_list: &[&[u8]] = &[ b"12", b"34 ; chunk-ext\r\nxxxxx" ];
    let mut pos = 0;

    let mut b = String::new();
    let mut b_skip = false;
    loop {
        let buf = buf_list[step];
        let cap = buf.len();
        while pos < cap && buf[pos] != b'\n' {
            if ! b_skip {
                if buf[pos] != b';' {
                    b.push(char::from(buf[pos]));
                } else {
                    b_skip = true;
                }
            }
            pos += 1;
        }
        if cap > pos {
            pos += 1; /* skip \n */
            break;
        }
        pos = 0;
        step += 1; /* inner.read */
    }

    let chunk_len = b.trim();
    assert_eq!(chunk_len, "1234");
}
