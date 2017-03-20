
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

mod cpu;
mod flags;
mod ram;
mod register;

use cpu::CPU;

fn main() {
    let mut c = CPU::new();
    let program = &[0x40, 0x40, 0x40, 0x50, 0x50, 0x50, 0x40, 0x59, 0x07, 0x96];
    c.ram.load(0, program);
    for _ in 0..program.len() {
        c.execute_op();
    }
    println!("{:?}", c);
    println!("Ram[0xFFF0..0xFFFF]: {:?}", c.ram.dump(0xFFF0, 16));
}

