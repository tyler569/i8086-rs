
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

