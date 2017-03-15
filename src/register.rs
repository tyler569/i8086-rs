
use std::{ops, fmt};

pub struct Register {
    pub low: u8,
    pub high: u8,
}

impl Register {
    pub fn new(value: u16) -> Register {
        let low = value as u8;
        let high = (value >> 8) as u8;
        Register { low: low, high: high }
    }

    pub fn get(&self) -> u16 {
        (self.high as u16) << 8 | (self.low as u16)
    }

    pub fn set(&mut self, value: u16) {
        self.low = value as u8;
        self.high = (value >> 8) as u8;
    }
}

impl ops::AddAssign<u16> for Register {
    fn add_assign(&mut self, other: u16) {
        let temp = self.get();
        self.set(temp + other);
    }
}

impl ops::SubAssign<u16> for Register {
    fn sub_assign(&mut self, other: u16) {
        let temp = self.get();
        self.set(temp - other);
    }
}

impl ops::BitAndAssign<u16> for Register {
    fn bitand_assign(&mut self, other: u16) {
        let temp = self.get();
        self.set(temp & other);
    }
}

impl ops::BitXorAssign<u16> for Register {
    fn bitxor_assign(&mut self, other: u16) {
        let temp = self.get();
        self.set(temp ^ other);
    }
}

impl ops::BitOrAssign<u16> for Register {
    fn bitor_assign(&mut self, other: u16) {
        let temp = self.get();
        self.set(temp | other);
    }
}

impl ops::ShlAssign<u16> for Register {
    fn shl_assign(&mut self, other: u16) {
        let temp = self.get();
        self.set(temp << other);
    }
}

impl ops::ShrAssign<u16> for Register {
    fn shr_assign(&mut self, other: u16) {
        let temp = self.get();
        self.set(temp >> other);
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Is this all?
        write!(f, "{}", self.get())
    }
}

bitflags! {
    flags Flags: u16 {
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
        write!("{}", "X...S.A.I.Z");
    }
}

