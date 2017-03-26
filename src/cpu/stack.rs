
use cpu::CPU;
use cpu::seg;

impl CPU {
    pub fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        let tos = seg(self.ss, self.sp);
        self.ram.write_w(tos, value);
    }

    pub fn pop(&mut self) -> u16 {
        let tos = seg(self.ss, self.sp);
        let ret = self.ram.read_w(tos);
        self.sp = self.sp.wrapping_add(2);
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
        assert_eq!(c.sp, 0xFFFE);
        assert_eq!(c.pop(), 0xFFAA);
        assert_eq!(c.sp, 0);
    }
}

