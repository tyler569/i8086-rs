
pub fn low(r: u16) -> u8 {
    r as u8
}

pub fn high(r: u16) -> u8 {
    (r >> 8) as u8
}

pub fn set_low(r: &mut u16, n: u8) {
    *r &= 0xFF00;
    *r |= n as u16;
}

pub fn set_high(r: &mut u16, n: u8) {
    *r &= 0x00FF;
    *r |= (n as u16) << 8;
}

/*
pub const AX: usize = 0;
pub const CX: usize = 1;
pub const DX: usize = 2;
pub const BX: usize = 3;
pub const SI: usize = 4;
pub const DI: usize = 5;
pub const SP: usize = 6;
pub const BP: usize = 7;
pub const ES: usize = 0;
pub const CS: usize = 1;
pub const SS: usize = 2;
pub const DS: usize = 3;


#[derive(Debug, Clone, Copy)]
pub enum Gpr {
    AX = 0, CX = 1, DX = 2, BX = 3,
    SI = 4, DI = 5, SP = 6, BP = 7,
}
pub use self::Gpr::*;

impl From<usize> for Gpr {
    fn from(v: usize) -> Gpr {
        match v {
            0 => Gpr::AX,
            1 => Gpr::CX,
            2 => Gpr::DX,
            3 => Gpr::BX,
            4 => Gpr::SI,
            5 => Gpr::DI,
            6 => Gpr::SP,
            7 => Gpr::BP,
            _ => panic!("Undefined Gpr: {}", v),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Seg {
    ES = 0, CS = 1, SS = 2, DS = 3,
}
pub use self::Seg::*;

impl From<usize> for Seg {
    fn from(v: usize) -> Seg {
        match v {
            0 => Seg::ES,
            1 => Seg::CS,
            2 => Seg::SS,
            3 => Seg::DS,
            _ => panic!("Undefined Seg: {}", v),
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut r = 0;
        set_high(&mut r, 0x80);
        set_low(&mut r, 0x80);
        let tmp = low(r) + 6;
        set_low(&mut r, tmp);
        assert_eq!(r, 0x8086);
    }
}

