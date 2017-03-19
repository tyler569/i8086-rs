
use std::num::Wrapping;

pub fn low(r: Wrapping<u16>) -> Wrapping<u8> {
    let Wrapping(v) = r;
    Wrapping(v as u8)
}

pub fn high(r: Wrapping<u16>) -> Wrapping<u8> {
    let Wrapping(v) = r >> 8;
    Wrapping(v as u8)
}

pub fn set_low(r: Wrapping<u16>, n: Wrapping<u8>) -> Wrapping<u16> {
    let Wrapping(v) = n;
    (r & Wrapping(0xFF00)) | Wrapping(v as u16)
}

pub fn set_high(r: Wrapping<u16>, n: Wrapping<u8>) -> Wrapping<u16> {
    let Wrapping(v) = n;
    (r & Wrapping(0x00FF)) | Wrapping((v as u16) << 8)
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
        let r = Wrapping(0);
        let r = set_high(r, Wrapping(0x80));
        let r = set_low(r, Wrapping(0x80));
        let r = set_low(r, low(r) + Wrapping(6));
        assert_eq!(r, Wrapping(0x8086));
        
    }
}

