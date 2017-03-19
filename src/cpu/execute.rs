
use std::num::Wrapping;
use cpu::{CPU, seg};
use cpu::ops::*;

impl CPU {
    pub fn execute_op(&mut self) {
        let op = self.ram.read_b(seg(self.cs, self.ip));
        match op {
            INC_R...INC_R + 7 => {
                let reg = op - INC_R;
                self.reg[reg] += Wrapping(1);
            }
            0x40 => *self.ax.val() += 1,
            0x41 => *self.cx.val() += 1,
            0x42 => *self.dx.val() += 1,
            0x43 => *self.bx.val() += 1,
            0x44 => *self.si.val() += 1,
            0x45 => *self.di.val() += 1,
            0x46 => *self.sp.val() += 1,
            0x47 => *self.bp.val() += 1,
            // DEC /r
            0x48 => *self.ax.val() -= 1,
            0x49 => *self.cx.val() -= 1,
            0x4A => *self.dx.val() -= 1,
            0x4B => *self.bx.val() -= 1,
            0x4C => *self.si.val() -= 1,
            0x4D => *self.di.val() -= 1,
            0x4E => *self.sp.val() -= 1,
            0x4F => *self.bp.val() -= 1,
            // PUSH /r
            0x50...0x57 =>

            _ => panic!("#UD"),
        }
        self.ip += Wrapping(1);
    }
}

