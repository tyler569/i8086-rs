
use cpu::CPU;
use ram::RAM;

impl CPU {
    pub fn push(&mut self, ram: &mut RAM, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.ram_write_w(ram, self.ss, self.sp, value);
    }

    pub fn pop(&mut self, ram: &mut RAM) -> u16 {
        let ret = self.ram_read_w(ram, self.ss, self.sp);
        self.sp = self.sp.wrapping_add(2);
        ret
    }
}

#[cfg(test)]
mod tests {
    use cpu::CPU;
    use ram::RAM;

    #[test]
    fn test_stack() {
        let mut c = CPU::new();
        let mut r = RAM::new(0x10000);
        c.push(&mut r, 0xFFAA);
        assert_eq!(c.sp, 0xFFFE);
        assert_eq!(c.pop(&mut r), 0xFFAA);
        assert_eq!(c.sp, 0);
    }
}

