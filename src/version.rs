use std::fmt;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HttpVersion {
    HTTP10,
    HTTP11,
    RTSP10,
}


impl Default for HttpVersion {
    #[inline]
    fn default() -> HttpVersion { HttpVersion::HTTP11 }
}


impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpVersion::HTTP11 => fmt::Write::write_str(f, "HTTP/1.1"),
            HttpVersion::HTTP10 => fmt::Write::write_str(f, "HTTP/1.0"),
            HttpVersion::RTSP10 => fmt::Write::write_str(f, "RTSP/1.0"),
        }
    }
}


impl From<&str> for HttpVersion {
    fn from(s: &str) -> HttpVersion {
        if s.eq_ignore_ascii_case("http/1.1") {
            HttpVersion::HTTP11
        } else if s.eq_ignore_ascii_case("rtsp/1.0") {
            HttpVersion::RTSP10
        } else {
            HttpVersion::HTTP10
        }
    }
}
