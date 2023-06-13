use core::mem;
use gloo_events::EventListener;
use gloo_utils::window;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

const KEY_CODES: &[&str] = &[
    "KeyX", "Digit1", "Digit2", "Digit3", "KeyQ", // 0 - 4
    "KeyE", "KeyA", "KeyS", "KeyD", "KeyW", // 5 - 9
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

#[derive(Clone, Debug, Default)]
pub struct Keypad {
    pub key_states: [KeyState; KEY_CODES.len()],
    pub last_keypress: Option<usize>,
}

impl Keypad {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_key_state(&mut self, keypress: &Condvar, i: usize, state: KeyState) {
        let prev_state = mem::replace(&mut self.key_states[i], state);
        if prev_state == KeyState::Up && state == KeyState::Down {
            self.last_keypress = Some(i);
            keypress.notify_all();
        }
    }
}

fn on_keypress(
    keystate: KeyState,
    keypad_and_keypress: &Arc<(Mutex<Keypad>, Condvar)>,
) -> impl Fn(&Event) {
    let keypad_and_keypress = Arc::clone(keypad_and_keypress);
    move |event: &Event| {
        let event = event.dyn_ref::<KeyboardEvent>().unwrap();
        let code = event.code();
        if let Some(&key_index) = KEY_CODE_INDICES.get(&code) {
            let (keypad, keypress) = &*keypad_and_keypress;
            keypad
                .lock()
                .unwrap()
                .update_key_state(keypress, key_index, keystate);
        }
    }
}

#[derive(Debug)]
pub struct KeyPressListeners {
    pub on_keydown: EventListener,
    pub on_keyup: EventListener,
}

impl KeyPressListeners {
    pub fn new(keypad_and_keypress: &Arc<(Mutex<Keypad>, Condvar)>) -> Self {
        let window = window();

        let on_keydown = EventListener::new(
            &window,
            "keydown",
            on_keypress(KeyState::Down, keypad_and_keypress),
        );

        let on_keyup = EventListener::new(
            &window,
            "keyup",
            on_keypress(KeyState::Up, keypad_and_keypress),
        );

        Self {
            on_keydown,
            on_keyup,
        }
    }
}
