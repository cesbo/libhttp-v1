use std::fmt;


const DEFAULT_BUF_SIZE: usize = 8 * 1024;


/// Read/Write buffer
pub struct HttpBuffer {
    pub buf: Box<[u8]>,
    pub pos: usize,
    pub cap: usize,
}


impl HttpBuffer {
    pub fn clear(&mut self) {
        self.pos = 0;
        self.cap = 0;
    }
}


impl fmt::Debug for HttpBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HttpBuffer")
            .field("pos", &self.pos)
            .field("cap", &self.cap)
            .finish()
    }
}


impl Default for HttpBuffer {
    fn default() -> HttpBuffer {
        HttpBuffer {
            buf: {
                let mut v = Vec::with_capacity(DEFAULT_BUF_SIZE);
                unsafe { v.set_len(DEFAULT_BUF_SIZE) };
                v.into_boxed_slice()
            },
            pos: 0,
            cap: 0,
        }
    }
}
