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
    if buffer.find(": ") != None {    
        let mut v = buffer.split(": ");
        let header = v.next().unwrap_or("").to_lowercase();
        let data = v.next().unwrap_or("");
        headers.insert(header.to_string(), (data[.. (data.len() - 2)]).to_string());
    }
}
