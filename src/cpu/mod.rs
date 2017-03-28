
mod execute;
mod stack;
mod ops;

use std::fmt;
use flags::*;
use ram::RAM;
use register;


#[derive(Debug)]
pub struct CPU {
    pub ax: u16, pub cx: u16, pub dx: u16, pub bx: u16,
    pub si: u16, pub di: u16, pub sp: u16, pub bp: u16,
    pub es: u16, pub cs: u16, pub ss: u16, pub ds: u16,
    pub ip: u16, pub flags: Flags,
}                                                                              

impl CPU {
    pub fn new() -> CPU {
        CPU {
            ax: 0, cx: 0, dx: 0, bx: 0, si: 0, di: 0, sp: 0, bp: 0,
            es: 0, cs: 0, ss: 0, ds: 0,
            ip: 0, flags: Flags::empty(),
        }
    }

    pub fn seg_to_phy(segment: u16, offset: u16) -> usize {
        (segment as usize) * 16 + offset as usize
    }

    pub fn get_gpr(&self, index: usize) -> u16 {
        match index {
            0 => self.ax,
            1 => self.cx,
            2 => self.dx,
            3 => self.bx,
            4 => self.si,
            5 => self.di,
            6 => self.sp,
            7 => self.bp,
            _ => panic!("Out of range: {} is not a gpr", index),
        }
    }

    pub fn get_gpr8(&self, index: usize) -> u8 {
        match index {
            0 => register::low(self.ax),
            1 => register::low(self.cx),
            2 => register::low(self.dx),
            3 => register::low(self.bx),
            4 => register::high(self.ax),
            5 => register::high(self.cx),
            6 => register::high(self.dx),
            7 => register::high(self.bx),
            _ => panic!("Out of range: {} is not a gpr", index),
        }
    }
    
    pub fn set_gpr(&mut self, index: usize, value: u16) {
        match index {
            0 => self.ax = value,
            1 => self.cx = value,
            2 => self.dx = value,
            3 => self.bx = value,
            4 => self.si = value,
            5 => self.di = value,
            6 => self.sp = value,
            7 => self.bp = value,
            _ => panic!("Out of range: {} is not a gpr", index),
        }
    }

    pub fn set_gpr8(&mut self, index: usize, value: u8) {
        match index {
            0 => register::set_low(&mut self.ax, value),
            1 => register::set_low(&mut self.cx, value),
            2 => register::set_low(&mut self.dx, value),
            3 => register::set_low(&mut self.bx, value),
            4 => register::set_high(&mut self.ax, value),
            5 => register::set_high(&mut self.cx, value),
            6 => register::set_high(&mut self.dx, value),
            7 => register::set_high(&mut self.bx, value),
            _ => panic!("Out of range: {} is not a gpr", index),
        }
    }

    pub fn get_seg_reg(&self, index: usize) -> u16 {
        match index {
            0 => self.es,
            1 => self.cs,
            2 => self.ss,
            3 => self.ds,
            _ => panic!("Out of range: {} is not a seg reg", index),
        }
    }

    pub fn set_seg_reg(&mut self, index: usize, value: u16) {
        match index {
            0 => self.es = value,
            1 => self.cs = value,
            2 => self.ss = value,
            3 => self.ds = value,
            _ => panic!("Out of range: {} is not a seg reg", index),
        }
    }

    pub fn ram_read_b(&self, ram: &RAM, segment: u16, offset: u16) -> u8 {
        ram.read_b(CPU::seg_to_phy(segment, offset))
    }

    pub fn ram_read_w(&self, ram: &RAM, segment: u16, offset: u16) -> u16 {
        ram.read_w(CPU::seg_to_phy(segment, offset))
    }

    pub fn ram_write_b(&self, ram: &mut RAM, segment: u16, offset: u16, value: u8) {
        ram.write_b(CPU::seg_to_phy(segment, offset), value)
    }

    pub fn ram_write_w(&self, ram: &mut RAM, segment: u16, offset: u16, value: u16) {
        ram.write_w(CPU::seg_to_phy(segment, offset), value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cpu() {
        // Something
    }
}

