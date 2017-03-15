
#![macro_use]
extern crate bitflags;

mod cpu;
mod register;

use cpu::CPU;

fn main() {
    let mut x = CPU::new();
    x.ax.low = 0x80;
    x.ax.high = 0x80;
    x.ax += 6;
    assert_eq!(x.ax.get(), 0x8086);
    println!("{:x}", x.ax.get());
}

