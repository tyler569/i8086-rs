
enum RamAddressMode {
    BX_SI,
    BX_DI,
    BP_SI,
    BP_Di,
    SI,
    DI,
    BP,
    BX,
    Disp,
}

enum RegMem {
    Ram(RamAddressMode),
    Reg16(usize),
    Reg8(usize),
    SegReg(usize),
    OpExt(usize),
}

enum ModRMType {
    R8,
    R16,
    SReg,
    OpExt,
    Ram,
}
use ModRMType::*;

/*
 * Mod = 0,1,2,3
 */

impl CPU {
    fn modrm(r: ModRMType, rm: ModRMType, modrm: u8) -> RegMem {
        let ix = modrm >> 6;
        let r_val = modrm >> 3 & 0x7;
        let rm_val = modrm & 0x7;
        
        let register = match r {
            R8 => RegMem::Reg8(r_val),
            R16 => RegMem::Reg16(r_val),
            SReg => RegMem::SegReg(r_val),
            OpExt => RegMem::OpExt(r_val),
            _ => panic!("Impossible - register values covered");
        }

        let register_mem = match (ix, rm) {
            (0, Ram) => {
                match rm_val {
                    6 => RegMem::Ram(RamAddressMode::Disp),
                    _ => RegMem::Ram(rm_val as RamAddressMode),
                }
            }
            _ => panic!(),
        }

        (register, register_mem)
    }
}        

