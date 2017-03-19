
use std::num::Wrapping;
use cpu::{seg, CPU};
use register::*;

impl CPU {
    pub fn push(&mut self, value: u16) {
        self.gpr[SP] -= Wrapping(2);
        self.ram.write_w(seg(self.seg[SS], self.gpr[SP]), value);
    }

    pub fn pop(&mut self) -> u16 {
        let ret = self.ram.read_w(seg(self.seg[SS], self.gpr[SP]));
        self.gpr[SP] += Wrapping(2);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::CPU;

    #[test]
    fn test_stack() {
        let mut c = CPU::new();
        c.push(0xFFAA);
        assert_eq!(c.gpr[SP], Wrapping(0xFFFE));
        assert_eq!(c.pop(), 0xFFAA);
        assert_eq!(c.gpr[SP], Wrapping(0));
    }
}

