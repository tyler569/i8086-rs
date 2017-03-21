
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
    let program = include_bytes!("../asm/test_func");
    println!("Program: {:?}", program);
    c.ram.load(0, program);
    for _ in 0..program.len() {
        c.execute_op();
    }
    println!("{:?}", c);
    println!("Ram[0xFFF0..0xFFFF]: {:?}", c.ram.dump(0xFFF0, 16));
}

