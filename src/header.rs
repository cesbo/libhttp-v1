use std::collections::HashMap;


pub fn headers_case(inp: &str) -> String {
    let mut ret = String::new();
    for part in inp.split('-') {
        if ! ret.is_empty() {
            ret += "-";
        }
        if ! part.is_empty() {
            ret += &part[.. 1].to_uppercase();
            ret += &part[1 ..];
        }
    }
    ret
}

pub fn pars_heades_line(headers: &mut HashMap<String, String>, buffer: &str) {
    if let Some(flag) = buffer.find(":") {   
        let header = &buffer[.. flag].trim();
        let data = &buffer[flag + 1 ..].trim();
        headers.insert(header.to_lowercase().to_string(), data.to_string());
    }
}
