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
use std::{cell::RefCell, panic, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, HtmlSelectElement};

const CYCLES_PER_FRAME: u8 = 10;

fn start_game(keypad: Rc<RefCell<Keypad>>) -> AnimationFrame {
    let select_game = document()
        .get_element_by_id("select-game")
        .unwrap_throw()
        .dyn_into::<HtmlSelectElement>()
        .unwrap_throw();
    let rom_name = select_game.value();
    let rom_buf = ROMS.get(&rom_name).unwrap_throw();

    let view = View::new();

    let mut cpu = Cpu::new(rom_buf, view, keypad);
    log!("Created CPU");

    let animation_frame = view::set_up_render_loop(move || {
        for _ in 0..CYCLES_PER_FRAME {
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

    let keypad = Rc::new(RefCell::new(Keypad::new()));
    let key_press_listeners = KeyPressListeners::new(&keypad);

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
        curr_animation_frame.replace(start_game(Rc::clone(&keypad)));
    });

    // TODO: Handle removing listener instead of leaking
    btn_play_on_click.forget();
}
