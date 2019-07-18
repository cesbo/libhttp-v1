use std::fmt;


const HTTP10: &str = "HTTP/1.0";
const HTTP11: &str = "HTTP/1.1";
const RTSP10: &str = "RTSP/1.0";    // https://www.ietf.org/rfc/rfc2326.txt
const RTSP20: &str = "RTSP/2.0";    // https://www.ietf.org/rfc/rfc7826.txt


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HttpVersion {
    HTTP10,
    HTTP11,
    RTSP10,
    RTSP20,
}


impl Default for HttpVersion {
    #[inline]
    fn default() -> HttpVersion { HttpVersion::HTTP11 }
}


impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpVersion::HTTP11 => fmt::Write::write_str(f, HTTP11),
            HttpVersion::HTTP10 => fmt::Write::write_str(f, HTTP10),
            HttpVersion::RTSP10 => fmt::Write::write_str(f, RTSP10),
            HttpVersion::RTSP20 => fmt::Write::write_str(f, RTSP20),
        }
    }
}


impl From<&str> for HttpVersion {
    fn from(s: &str) -> HttpVersion {
        if s.eq_ignore_ascii_case(HTTP11) {
            HttpVersion::HTTP11
        } else if s.eq_ignore_ascii_case(RTSP10) {
            HttpVersion::RTSP10
        } else if s.eq_ignore_ascii_case(RTSP20) {
            HttpVersion::RTSP20
        } else {
            HttpVersion::HTTP10
        }
    }
}
