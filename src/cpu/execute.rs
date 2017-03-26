
use cpu::{CPU, seg};
use cpu::ops::*;
use register::*;

impl CPU {
    pub fn execute_op(&mut self) {
        let op_addr = seg(self.cs, self.ip);
        let op = self.ram.read_b(op_addr);
        match op {
            0x01 => {
                self.ip = self.ip.wrapping_add(1);
                let modrm_addr = seg(self.cs, self.ip);
                let modrm = self.ram.read_b(modrm_addr);
                let register_from = ((modrm >> 3) & 0x7) as usize;

                let value1 = self.get_gpr(register_from);

                let modrm_ix = modrm >> 6;
                let modrm_to = modrm & 0x7;

                let mut value2;

                match (modrm_ix, modrm_to) {
                    (0, 0) => {
                        let value2_addr = seg(self.ds, self.bx + self.si);
                        value2 = self.ram.read_w(value2_addr);
                    }
                    (0, 1) => {
                        let value2_addr = seg(self.ds, self.bx + self.di);
                        value2 = self.ram.read_w(value2_addr);
                    }
                    // ...
                    (0, 6) => {
                        let disp_addr = seg(self.cs, self.ip.wrapping_add(1)) as usize;
                        let value2_addr = self.ram.read_w(disp_addr) as usize;
                        value2 = self.ram.read_w(value2_addr);
                        self.ip = self.ip.wrapping_add(2);
                    }
                    (1, 0) => {
                        let disp_addr = seg(self.cs, self.ip.wrapping_add(1));
                        let disp = self.ram.read_b(disp_addr) as u16;
                        let value2_addr = seg(self.ds, self.bx + self.si + disp);
                        value2 = self.ram.read_w(value2_addr);
                        self.ip = self.ip.wrapping_add(1);
                    }
                    // ...
                    _ => unimplemented!(),
                }

            }
            _ => panic!("#UD: 0x{:02x}", op),
        }
        self.ip += 1;
    }
}

