use gloo_events::EventListener;
use gloo_utils::window;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

/// The CHIP-8 used a hexadecimal keyboard with the following layout:
///
/// 1 2 3 C
/// 4 5 6 D
/// 7 8 9 E
/// A 0 B F
///
/// We remap these keys to the following layout:
///
/// 1 2 3 4
/// Q W E R
/// A S D F
/// Z X C V
const KEY_CODES: &[&str] = &[
    "KeyX", "Digit1", "Digit2", "Digit3", "KeyQ", // 0 - 4
    "KeyW", "KeyE", "KeyA", "KeyS", "KeyD", // 5 - 9
    "KeyZ", "KeyC", "Digit4", "KeyR", "KeyF", "KeyV", // A - F
];

lazy_static! {
    static ref KEY_CODE_INDICES: HashMap<String, usize> = KEY_CODES
        .iter()
        .enumerate()
        .map(|(i, key)| ((*key).to_owned(), i))
        .collect();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyState {
    Down,
    Up,
}

impl Default for KeyState {
    fn default() -> Self {
        Self::Down
    }
}

#[derive(Debug)]
pub enum LastKeypressState {
    NotWaiting,
    Waiting,
    Found(usize),
}

impl Default for LastKeypressState {
    fn default() -> Self {
        Self::NotWaiting
    }
}

#[derive(Debug, Default)]
pub struct Keypad {
    pub key_states: [KeyState; KEY_CODES.len()],
    pub last_keypress: LastKeypressState,
}

impl Keypad {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_key_state(&mut self, i: usize, state: KeyState) {
        self.key_states[i] = state;

        // `keyup` should only fire if previous state was down
        if let LastKeypressState::Waiting = self.last_keypress {
            if let KeyState::Up = state {
                self.last_keypress = LastKeypressState::Found(i);
            }
        }
    }

    pub fn try_take_last_keypress(&mut self) -> Option<usize> {
        if let LastKeypressState::Found(key) = self.last_keypress {
            self.last_keypress = LastKeypressState::NotWaiting;
            Some(key)
        } else {
            self.last_keypress = LastKeypressState::Waiting;
            None
        }
    }
}

fn on_keypress(keystate: KeyState, keypad: &Arc<Mutex<Keypad>>) -> impl Fn(&Event) {
    let keypad = Arc::clone(keypad);
    move |event: &Event| {
        let event = event.dyn_ref::<KeyboardEvent>().unwrap();
        let code = event.code();
        if let Some(&key_index) = KEY_CODE_INDICES.get(&code) {
            keypad.lock().unwrap().update_key_state(key_index, keystate);
        }
    }
}

#[derive(Debug)]
pub struct KeyPressListeners {
    pub on_keydown: EventListener,
    pub on_keyup: EventListener,
}

impl KeyPressListeners {
    pub fn new(keypad: &Arc<Mutex<Keypad>>) -> Self {
        let window = window();

        let on_keydown =
            EventListener::new(&window, "keydown", on_keypress(KeyState::Down, keypad));

        let on_keyup = EventListener::new(&window, "keyup", on_keypress(KeyState::Up, keypad));

        Self {
            on_keydown,
            on_keyup,
        }
    }
}
