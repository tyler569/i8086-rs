
use std::num::Wrapping;
use std::mem::transmute;

/// A representation of an 8086 register
///
/// Note that this currently relies on some probably unstable 
/// definately unsafe behavior, as it creates pointers into an integer,
/// and will likely not work on a big-endian platform.
///
/// All internal values are Wrapping<u16> or Wrapping<u8> to model
/// the 8086's wrapping behavior.
///
/// # Example
///
/// ```
/// let mut c = Register::new(0);
/// *c.high() += 1;
/// *c.low() += 1;
///
/// assert_eq!(*c.val(), Wrapping(257));
/// ```
#[derive(Debug)]
pub struct Register(Wrapping<u16>);

impl Register {
    pub fn new(value: u16) -> Register {
        Register(Wrapping(value))
    }
    pub fn val(&mut self) -> &mut Wrapping<u16> {
        unsafe { transmute(self) }
    }
    pub fn low(&mut self) -> &mut Wrapping<u8> {
        unsafe { transmute(self) }
    }
    pub fn high(&mut self) -> &mut Wrapping<u8> {
        let c = unsafe { transmute::<_, *mut Wrapping<u8>>(self).offset(1) };
        unsafe { transmute(c) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut c = Register(Wrapping(0x1010));
        *c.high() = Wrapping(0xA0);
        *c.low() += Wrapping(0xEF);
        *c.low() += Wrapping(0x01);
        assert_eq!(*c.val(), Wrapping(0xA000));
    }
}

