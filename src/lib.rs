pub mod cpu;
pub mod keypad;
pub mod opcode;
pub mod view;

use crate::{
    cpu::Cpu,
    keypad::{KeyPressListeners, Keypad},
    view::View,
};
use core::mem;
use std::sync::{Arc, Condvar, Mutex};
use wasm_bindgen::prelude::*;
use web_sys::console;

/// # Errors
/// Any program errors are returned as a top-level WASM error.
#[wasm_bindgen(start)]
pub fn entry() -> Result<(), JsValue> {
    main().map_err(|err| err.to_string())?;
    Ok(())
}

fn main() -> eyre::Result<()> {
    console::log_1(&"Starting up emulator...".into());

    // TODO: Enable loading the other roms.
    let rom_buf = include_bytes!("../roms/PONG.rom");

    console::log_1(&"Finished reading ROMs".into());
    let view = View::new()?;
    let keypad_and_keypress = Arc::new((Mutex::new(Keypad::new()), Condvar::new()));

    let key_press_listeners = KeyPressListeners::new(&keypad_and_keypress)?;
    // TODO: Handle removing listeners instead of leaking
    mem::forget(key_press_listeners);

    let mut cpu = Cpu::new(rom_buf, &view, Arc::clone(&keypad_and_keypress));
    cpu.cycle();
    Ok(())
}
