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
