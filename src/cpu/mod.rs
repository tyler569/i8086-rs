
mod execute;
mod stack;
mod ops;

use std::fmt;
use flags::*;
use ram::Ram;
use register::*;

fn seg(segment: u16, offset: u16) -> usize {
    ((segment as usize) << 4) + (offset as usize)
}

pub struct CPU {
     pub gpr: [u16; 8],
     pub seg: [u16; 4],
     pub ip: u16,
     pub flags: Flags,
     pub ram: Ram,
}                                                                              

impl CPU {
     pub fn new() -> CPU {
        CPU {
            gpr: [0; 8], seg: [0; 4], ip: 0, flags: Flags::empty(),
            ram: Ram::new(65536),
        }
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "AX:{:04X} CX:{:04X} DX:{:04X} BX:{:04X}", self.gpr[AX], self.gpr[CX], self.gpr[DX], self.gpr[BX])?;
        writeln!(f, "SI:{:04X} DI:{:04X} SP:{:04X} BP:{:04X}", self.gpr[SI], self.gpr[DI], self.gpr[SP], self.gpr[BP])?;
        writeln!(f, "CS:{:04X} DS:{:04X} ES:{:04X} SS:{:04X}", self.seg[CS], self.seg[DS], self.seg[ES], self.seg[SS])?;
        write!(f, "FLAGS:{} IP:{:04X}", self.flags, self.ip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {
        // Something
    }
}

