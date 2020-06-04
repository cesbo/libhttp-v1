//! HTTP Core: HTTP IO Buffer

use std::fmt;


const DEFAULT_BUF_SIZE: usize = 8 * 1024;


/// Read/Write buffer
pub struct HttpBuffer {
    pub buf: [u8; DEFAULT_BUF_SIZE],
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
            buf: [0; DEFAULT_BUF_SIZE],
            pos: 0,
            cap: 0,
        }
    }
}
