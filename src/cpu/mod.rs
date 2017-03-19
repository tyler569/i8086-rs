
mod execute;
mod stack;
mod ops;

use std::num::Wrapping;
use flags::*;
use register::Register;
use ram::Ram;

fn seg(segment: Wrapping<u16>, offset: Wrapping<u16>) -> usize {
    let Wrapping(s) = segment;
    let Wrapping(o) = offset;
    ((s as usize) << 4) + (o as usize)
}

#[derive(Debug)]
pub struct CPU {
     pub ax: Register,
     pub cx: Register,
     pub dx: Register,
     pub bx: Register,
     pub si: Wrapping<u16>,
     pub di: Wrapping<u16>,
     pub sp: Wrapping<u16>,
     pub bp: Wrapping<u16>,
     pub cs: Wrapping<u16>,
     pub ds: Wrapping<u16>,
     pub es: Wrapping<u16>,
     pub ss: Wrapping<u16>,
     pub ip: Wrapping<u16>,
     pub flags: Flags,
     pub ram: Ram,
}                                                                              

impl CPU {
     pub fn new() -> CPU {
        CPU {
            ax: Register::new(0), cx: Register::new(0),
            dx: Register::new(0), bx: Register::new(0),
            si: Wrapping(0), di: Wrapping(0), sp: Wrapping(0), bp: Wrapping(0),
            cs: Wrapping(0), ds: Wrapping(0), es: Wrapping(0), ss: Wrapping(0),
            ip: Wrapping(0), flags: OF, ram: Ram::new(),
        }
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

