
use std::fmt;

const RAM_LEN: usize = 65536;

pub struct Ram {
    data: Box<[u8]>,
}

impl Ram {
    pub fn new() -> Ram {
        Ram { data: Box::new([0; RAM_LEN]) }
    }

    pub fn write_b(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn read_b(&mut self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn write_w(&mut self, addr: usize, value: u16) {
        self.data[addr] = value as u8;
        self.data[addr + 1] = (value >> 8) as u8;
    }

    pub fn read_w(&self, addr: usize) -> u16 {
        let mut ret: u16 = 0;
        ret |= self.data[addr] as u16;
        ret |= (self.data[addr + 1] as u16) << 8;
        ret
    }

    pub fn load(&mut self, addr: usize, data: &[u8]) {
        for (i, v) in data.iter().enumerate() {
            self.data[addr + i] = *v;
        }
    }

    pub fn dump(&self, addr: usize, len: usize) -> &[u8] {
        &self.data[addr..addr+len]
    }
}

impl fmt::Debug for Ram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ram[...{}]", RAM_LEN)
    }
}

