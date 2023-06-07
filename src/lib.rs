pub mod cpu;
pub mod keypad;
pub mod opcode;
pub mod view;

use crate::{
    cpu::Cpu,
    keypad::{KeyPressListeners, Keypad},
    view::{set_up_render_loop, View},
};
use gloo_console::log;
use std::sync::{Arc, Condvar, Mutex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn entry() {
    log!("Starting up emulator...");

    // TODO: Enable loading the other roms.
    let rom_buf = include_bytes!("../roms/PONG.rom");

    log!("Finished reading ROMs");
    let view = View::new();
    let keypad_and_keypress = Arc::new((Mutex::new(Keypad::new()), Condvar::new()));

    let key_press_listeners = KeyPressListeners::new(&keypad_and_keypress);

    // TODO: Handle removing listeners instead of leaking
    key_press_listeners.on_keydown.forget();
    key_press_listeners.on_keyup.forget();

    let mut cpu = Cpu::new(rom_buf, view, Arc::clone(&keypad_and_keypress));
    log!("Created CPU");

    set_up_render_loop(move || {
        for _ in 0..8 {
            cpu.cycle();
        }
    });
    log!("Set up render loop");
}
