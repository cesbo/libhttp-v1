use std::{
    fmt,
    io::{
        self,
        Write,
    },
    collections::HashMap,
};


/// Set of the headers for HTTP request and response
#[derive(Default)]
pub struct Header(HashMap<String, String>);


impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(f) }
}


impl Header {
    /// Parses header line
    pub fn parse(&mut self, line: &str) {
        if let Some(skip) = line.find(':') {
            let key = line[.. skip].trim_end();
            if ! key.is_empty() {
                let key = key.to_lowercase();
                let value = line[skip + 1 ..].trim_start().to_string();
                self.0.insert(key, value);
            }
        }
    }

    /// Writes header key and value into dst
    /// Header key will be capitalized - first character and all characters after delimeter (`-`)
    pub fn send<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        for (key, value) in self.0.iter() {
            for (step, part) in key.split('-').enumerate() {
                if step != 0 {
                    write!(dst, "-")?;
                }
                if ! part.is_empty() {
                    write!(dst, "{}", &part[.. 1].to_uppercase())?;
                    write!(dst, "{}", &part[1 ..])?;
                }
            }
            writeln!(dst, ": {}\r", value)?;
        }

        Ok(())
    }

    /// Sets header value
    /// key should be in lowercase
    #[inline]
    pub fn set<R, S>(&mut self, key: R, value: S)
    where
        R: AsRef<str>,
        S: ToString,
    {
        self.0.insert(key.as_ref().to_string(), value.to_string());
    }

    /// Returns reference to the header value value corresponding to the key
    /// key should be in lowercase
    #[inline]
    pub fn get<R>(&self, key: R) -> Option<&str>
    where
        R: AsRef<str>,
    {
        self.0.get(key.as_ref()).map(std::string::String::as_str)
    }

    /// Removes all headers
    #[inline]
    pub fn clear(&mut self) { self.0.clear() }
}
