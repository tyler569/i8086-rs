
use flags::Flags;
use register::Register;

 struct CPU {
     ax: Register,
     cx: Register,
     dx: Register,
     bx: Register,
     si: u16,
     di: u16,
     sp: u16,
     bp: u16,
     flags: Flags,
}

impl CPU {
     fn new() -> CPU {
        CPU {
            ax: Register::new(0), cx: Register::new(0),
            dx: Register::new(0), bx: Register::new(0),
            si: 0, di: 0, sp: 0, bp: 0,
            flags: 0
        }
    }
}

