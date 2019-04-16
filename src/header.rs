use std::collections::HashMap;
use std::io::Write;

use crate::error::Result;


pub fn write_key<W: Write>(key: &str, dst: &mut W) -> Result<()> {
    for (step, part) in key.split('-').enumerate() {
        if step > 0 {
            write!(dst, "-")?;
        }
        if ! part.is_empty() {
            write!(dst, "{}", &part[.. 1].to_uppercase())?;
            write!(dst, "{}", &part[1 ..])?;
        }
    }
    Ok(())
}

pub fn parse(headers: &mut HashMap<String, String>, buffer: &str) {
    if let Some(flag) = buffer.find(':') {   
        let header = &buffer[.. flag].trim();
        let data = &buffer[flag + 1 ..].trim();
        headers.insert(header.to_lowercase(), data.to_string());
    }
}
