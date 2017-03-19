
use std::num::Wrapping;
use cpu::{CPU, seg};
use cpu::ops::*;
use register::*;

impl CPU {
    pub fn execute_op(&mut self) {
        let op = self.ram.read_b(seg(self.seg[CS], self.ip));
        match op {
            INC_R...0x47 => {
                let reg = (op - INC_R) as usize;
                self.gpr[reg] += Wrapping(1);
            }
            DEC_R...0x4F => {
                let reg = (op - DEC_R) as usize;
                self.gpr[reg] -= Wrapping(1);
            }
            PUSH_R...0x57 => {
                let reg = (op - PUSH_R) as usize;
                let Wrapping(value) = self.gpr[reg];
                self.push(value);
            }
            POP_R...0x5F => {
                let reg = (op - POP_R) as usize;
                let value = Wrapping(self.pop());
                self.gpr[reg] = value;
            }
            HLT => {}

            _ => panic!("#UD"),
        }
        self.ip += Wrapping(1);
    }
}

