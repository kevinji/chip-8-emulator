mod cpu;
mod keypad;
mod opcode;
mod roms;
mod view;

use crate::{
    cpu::Cpu,
    keypad::{KeyPressListeners, Keypad},
    roms::ROMS,
    view::{AnimationFrame, View},
};
use gloo_console::log;
use gloo_events::EventListener;
use gloo_utils::document;
use std::{
    panic,
    sync::{Arc, Condvar, Mutex},
};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, HtmlSelectElement};

fn start_game(keypad_and_keypress: Arc<(Mutex<Keypad>, Condvar)>) -> AnimationFrame {
    let select_game = document()
        .get_element_by_id("select-game")
        .unwrap_throw()
        .dyn_into::<HtmlSelectElement>()
        .unwrap_throw();
    let rom_name = select_game.value();

    let rom_buf = ROMS.get(&rom_name).unwrap_throw();
    log!("Finished reading ROMs");

    let view = View::new();

    let mut cpu = Cpu::new(rom_buf, view, keypad_and_keypress);
    log!("Created CPU");

    let animation_frame = view::set_up_render_loop(move || {
        for _ in 0..8 {
            cpu.cycle();
        }

        // Timers should update at 60Hz
        cpu.update_timers();
    });
    log!("Set up render loop");

    animation_frame
}

#[wasm_bindgen(start)]
pub fn entry() {
    log!("Setting up emulator...");

    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let keypad_and_keypress = Arc::new((Mutex::new(Keypad::new()), Condvar::new()));
    let key_press_listeners = KeyPressListeners::new(&keypad_and_keypress);

    // TODO: Handle removing listeners instead of leaking
    key_press_listeners.on_keydown.forget();
    key_press_listeners.on_keyup.forget();

    let btn_play = document()
        .get_element_by_id("btn-play")
        .unwrap_throw()
        .dyn_into::<HtmlButtonElement>()
        .unwrap_throw();

    let mut curr_animation_frame = None;

    let btn_play_on_click = EventListener::new(&btn_play, "click", move |_| {
        curr_animation_frame.replace(start_game(Arc::clone(&keypad_and_keypress)));
    });

    // TODO: Handle removing listener instead of leaking
    btn_play_on_click.forget();
}
