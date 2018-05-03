extern crate rand;

mod cpu;
mod graphics;
mod opcode;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.cycle();
    println!("Hello, world!");
}
