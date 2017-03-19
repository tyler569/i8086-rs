
use std::num::Wrapping;
use cpu::{seg, CPU};

impl CPU {
    pub fn push(&mut self, value: u16) {
        self.sp -= Wrapping(2);
        self.ram.write_w(seg(self.ss, self.sp), value);
    }

    pub fn pop(&mut self) -> u16 {
        let ret = self.ram.read_w(seg(self.ss, self.sp));
        self.sp += Wrapping(2);
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
        assert_eq!(c.sp, Wrapping(0xFFFE));
        assert_eq!(c.pop(), 0xFFAA);
        assert_eq!(c.sp, Wrapping(0));
    }
}

