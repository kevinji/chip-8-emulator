extern crate failure;
extern crate js_sys;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate wasm_bindgen;

mod cpu;
mod keypad;
mod opcode;
mod view;

use std::fs::File;
use std::io::Read;

use failure::Fallible;
use wasm_bindgen::prelude::*;

use cpu::Cpu;

const DEFAULT_ROM: &str = "PONG";

#[wasm_bindgen]
pub fn entry() {
    main().unwrap()
}

fn main() -> Fallible<()> {
    let mut rom_buf = Vec::new();
    read_rom(DEFAULT_ROM, &mut rom_buf)?;

    let mut cpu = Cpu::new(&rom_buf);
    cpu.cycle();
    Ok(())
}

fn read_rom(name: &str, buf: &mut Vec<u8>) -> Fallible<()> {
    let mut f = File::open(format!("roms/{}.rom", name))?;
    f.read_to_end(buf)?;
    Ok(())
}
