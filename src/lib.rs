pub mod cpu;
pub mod keypad;
pub mod opcode;
pub mod view;

use failure::Fallible;
use wasm_bindgen::prelude::*;
use web_sys::console;

use cpu::Cpu;
use view::View;

#[wasm_bindgen(start)]
pub fn entry() -> Result<(), JsValue> {
    console::log_1(&"Hello world!".into());
    main().map_err(|error| error.to_string().into())
}

fn main() -> Fallible<()> {
    // TODO: Enable loading the other roms.
    let rom_buf = include_bytes!("../roms/PONG.rom");

    console::log_1(&"Finished reading ROMs".into());
    let view = View::new()?;
    let mut cpu = Cpu::new(rom_buf, &view);
    cpu.cycle();
    Ok(())
}
