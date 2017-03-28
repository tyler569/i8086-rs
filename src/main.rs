
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

mod cpu;
mod flags;
mod ram;
mod register;

use cpu::CPU;
use ram::RAM;

fn main() {
    let mut c = CPU::new();
    let mut r = Box::new(RAM::new(65536));

    let program = &[01, 06, 01, 00, 40, 50, 60];
    println!("Program: {:?}", program);
    r.load(0, program);
    
    for _ in 0..program.len() {
        c.execute_op(&mut r);
    }
    
    println!("{:?}", c);
    println!("Ram[0xFFF0..0xFFFF]: {:?}", r.dump(0xFFF0, 16));
}

