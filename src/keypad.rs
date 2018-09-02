use std::sync::{Condvar, Mutex};

use wasm_bindgen::prelude::*;

const KEY_COUNT: usize = 16;

/*
static KEY_CODES: &[&str] = &[
    "KeyX", "Digit1", "Digit2", "Digit3", "KeyQ", // 0 - 4
    "KeyE", "KeyA", "KeyS", "KeyD", "KeyW", // 5 - 9
    "KeyZ", "KeyC", "Digit4", "KeyR", "KeyF", "KeyV", // A - F
];
*/

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyState {
    Down,
    Up,
}

impl Default for KeyState {
    fn default() -> KeyState { KeyState::Down }
}

#[derive(Clone, Debug, Default)]
pub struct Keypad {
    pub key_states: [KeyState; KEY_COUNT],
}

impl Keypad {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update_key_state(&mut self, i: usize, state: KeyState) {
        self.key_states[i] = state;
    }
}

// Keep the Keypad public for wasm_bindgen.
lazy_static! {
    pub static ref KEYPAD: Mutex<Keypad> = Mutex::new(Keypad::new());
    static ref WAIT: (Mutex<Option<usize>>, Condvar) =
        (Mutex::new(None), Condvar::new());
}

pub fn wait_for_key_press() -> usize {
    let &(ref lock, ref cvar) = &*WAIT;
    let mut key = lock.lock().unwrap();
    while (*key).is_none() {
        key = cvar.wait(key).unwrap();
    }

    return key.unwrap();
}

fn alert_key_press(i: usize) {
    let &(ref lock, ref cvar) = &*WAIT;
    let mut key = lock.lock().unwrap();
    *key = Some(i);
    cvar.notify_one();
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn update_key_state(i: usize, pressed: bool) {
    let state = if pressed { KeyState::Down } else { KeyState::Up };
    KEYPAD.lock().unwrap().update_key_state(i, state);

    if state == KeyState::Down {
        alert_key_press(i);
    }
}
