use std::io::BufRead;


#[derive(Default, Debug)]
pub struct Url {
    scheme: String,
    name: String,
    path: String,
    query: String,
    fragment: String,
}


pub struct Coord_url {
    start: usize,
    end:  usize,
}


impl Url {
    pub fn new(inp: &str) -> Self {
        let mut skip = 0;
	let mut step = 0;
	let mut scheme: Coord_url = Coord_url { start:0, end: 0 };
        let mut path: Coord_url = Coord_url { start:0, end: 0 };
        let mut query: Coord_url = Coord_url { start:0, end: 0 };
        let mut fragment: Coord_url = Coord_url { start:0, end: 0 };
        let mut name: Coord_url = Coord_url { start:0, end: 0 };
        if let Some(v) = inp.find("://") {
            skip = v + 3;
	    scheme.end = v;
	    name.start = skip;
        }
        for (idx, part) in inp[skip ..].match_indices(|c: char| (c == '/' || c == '?' || c == '#' )) {
            match part.as_bytes()[0] {
                b'/' if step < 1 => { path.start = idx + skip; step = 1; },
                b'?' if step < 2 => { query.start = idx + skip; step = 2; },
                b'#' if step < 3 => { fragment.start = idx + skip; break; },
                _ => {},
            }; 
        }
	let mut tail = inp.len();
	if fragment.start > 0 {
	    fragment.end = tail;
            tail = fragment.start;
	    fragment.start = fragment.start + 1;
        }
        if query.start > 0 {
            query.end = tail;
            tail = query.start;
            query.start = query.start + 1;
        }
        if path.start > 0 {
            path.end = tail;
            tail = path.start;
        }
        name.end = tail;
        Url {
            scheme: (&inp[scheme.start .. scheme.end]).to_string(),
            name: (&inp[name.start .. name.end]).to_string(),
            path: (&inp[path.start .. path.end]).to_string(),
            query: (&inp[query.start .. query.end]).to_string(),
            fragment: (&inp[fragment.start .. fragment.end]).to_string(),
        }
    }
    
    #[inline]
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    
    #[inline]
    pub fn get_scheme(&self) -> &str {
        self.scheme.as_str()
    }
    
    #[inline]
    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }
    
    #[inline]
    pub fn get_query(&self) -> &str {
        self.query.as_str()
    }
    
    #[inline]
    pub fn get_fragment(&self) -> &str {
        self.fragment.as_str()
    }
}
