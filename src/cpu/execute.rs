
use cpu::CPU;
use cpu::ops::*;
use ram::RAM;
use register::*;

macro_rules! wrap_sum {
    ($t:ident $(+ $u:ident)*) => ($t$(.wrapping_add($y))*);
}

impl CPU {
    pub fn execute_op(&mut self, ram: &mut RAM) {
        let op = self.ram_read_b(ram, self.cs, self.ip);
        match op {
            0x01 => {
                self.ip = self.ip.wrapping_add(1);
                let modrm = self.ram_read_b(ram, self.cs, self.ip);
                let register_from = ((modrm >> 3) & 0x7) as usize;

                let value1 = self.get_gpr(register_from);

                let modrm_ix = modrm >> 6;
                let modrm_to = modrm & 0x7;

                let mut value2;

                match (modrm_ix, modrm_to) {
                    (0, 0) => {
                        value2 = self.ram_read_w(ram, self.ds, wrap_sum!(self.bx + self.si));
                    }
                    (0, 1) => {
                        value2 = self.ram_read_w(ram, self.ds, wrap_sum!(self.bx + self.di));
                    }
                    // ...
                    (0, 6) => {
                        let value2_addr = self.ram_read_w(ram, self.cs, wrap_sum(self.ip + 1));
                        value2 = self.ram_read_w(ram, self.ds, value2_addr);
                        self.ip = self.ip.wrapping_add(2);
                    }
                    (1, 0) => {
                        // `as i8 as u16` sign extends u8 into u16.
                        let disp = self.ram_read_b(ram, self.cs, wrap_sum!(self.ip + 1)) as i8 as u16;
                        value2 = self.ram_read_w(ram, self.ds, wrap_sum!(self.bx + self.si + disp));
                        self.ip = self.ip.wrapping_add(1);
                    }
                    // ...
                    _ => unimplemented!(),
                }

            }
            _ => panic!("#UD: 0x{:02x}", op),
        }
        self.ip = self.ip.wrapping_add(1);
    }
}

