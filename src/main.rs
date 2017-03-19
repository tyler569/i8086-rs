
#[macro_use]
extern crate bitflags;

mod cpu;
mod flags;
mod ram;
mod register;

use cpu::CPU;

fn main() {
    let mut c = CPU::new();
    c.ram.write_b(0, 0x40);
    c.ram.write_b(1, 0x41);
    c.execute_op();
    c.execute_op();
    println!("{:?}", c);
}

