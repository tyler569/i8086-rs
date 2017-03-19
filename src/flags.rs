
use std::fmt;

bitflags! {
    pub flags Flags: u16 {
        const CF = 1 << 0,
        const PF = 1 << 2,
        const AF = 1 << 4,
        const ZF = 1 << 6,
        const SF = 1 << 7,
        const TF = 1 << 8,
        const IF = 1 << 9,
        const DF = 1 << 10,
        const OF = 1 << 11,
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "make X...S.A.I.Z")
    }
}

