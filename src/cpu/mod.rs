
mod execute;
mod stack;
mod ops;

use std::num::Wrapping;
use flags::*;
use ram::Ram;

fn seg(segment: Wrapping<u16>, offset: Wrapping<u16>) -> usize {
    let Wrapping(s) = segment;
    let Wrapping(o) = offset;
    ((s as usize) << 4) + (o as usize)
}

#[derive(Debug)]
pub struct CPU {
     pub gpr: [Wrapping<u16>; 8],
     pub seg: [Wrapping<u16>; 4],
     pub ip: Wrapping<u16>,
     pub flags: Flags,
     pub ram: Ram,
}                                                                              

impl CPU {
     pub fn new() -> CPU {
        CPU {
            gpr: [Wrapping(0); 8], seg: [Wrapping(0); 4],
            ip: Wrapping(0), flags: OF, ram: Ram::new(65536),
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

