use std::io;


/// Writes header key and value into dst
/// Header key will be capitalized - first character and all characters after delimeter (`-`)
pub (crate) fn header_write<W: io::Write>(dst: &mut W, key: &str, value: &str) -> io::Result<()> {
    for (step, part) in key.split('-').enumerate() {
        if step > 0 {
            write!(dst, "-")?;
        }
        if ! part.is_empty() {
            write!(dst, "{}", &part[.. 1].to_uppercase())?;
            write!(dst, "{}", &part[1 ..])?;
        }
    }
    writeln!(dst, ": {}\r", value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_write() {
        let mut result = Vec::<u8>::new();
        header_write(&mut result, "x-forwarded-for", "test").unwrap();
        assert_eq!(&result, b"X-Forwarded-For: test\r\n");
    }
}
