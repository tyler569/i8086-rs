
use std::fmt;

pub struct RAM {
    data: Vec<u8>,
}

impl RAM {
    pub fn new(len: usize) -> RAM {
        RAM { data: vec![0; len] }
    }

    pub fn write_b(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn read_b(&self, addr: usize) -> u8 {
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

impl fmt::Debug for RAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RAM[u8; {}]", self.data.len())
    }
}

