extern crate failure;
extern crate js_sys;
extern crate web_sys;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate wasm_bindgen;

pub mod cpu;
pub mod keypad;
pub mod opcode;
pub mod view;

use failure::Fallible;
use wasm_bindgen::prelude::*;
use web_sys::*;

use cpu::Cpu;
use view::View;

#[wasm_bindgen(start)]
pub fn entry() {
    console::log_1(&"Hello world!".into());
    match main() {
        Ok(()) => (),
        Err(error) => {
            let error = format!("{}", error);
            console::error_1(&error.into())
        },
    }
}

pub fn main() -> Fallible<()> {
    // TODO: Enable loading the other roms.
    let rom_buf = include_bytes!("../roms/PONG.rom");

    console::log_1(&"Finished reading ROMs".into());
    let view = View::new()?;
    let mut cpu = Cpu::new(rom_buf, &view);
    cpu.cycle();
    Ok(())
}
