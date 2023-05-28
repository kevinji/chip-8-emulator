use core::mem;
use eyre::eyre;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, KeyboardEvent};

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
) -> impl Fn(KeyboardEvent) {
    let keypad_and_keypress = Arc::clone(keypad_and_keypress);
    move |event: KeyboardEvent| {
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

pub fn set_up_key_press_listeners(
    keypad_and_keypress: &Arc<(Mutex<Keypad>, Condvar)>,
) -> eyre::Result<()> {
    let window = window().ok_or_else(|| eyre!("window does not exist"))?;

    let on_keydown = <Closure<dyn Fn(_)>>::new(on_keypress(KeyState::Down, keypad_and_keypress));
    window
        .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
        .map_err(|_| eyre!("Failed to create keydown event listener"))?;
    mem::forget(on_keydown);

    let on_keyup = <Closure<dyn Fn(_)>>::new(on_keypress(KeyState::Up, keypad_and_keypress));
    window
        .add_event_listener_with_callback("keyup", on_keyup.as_ref().unchecked_ref())
        .map_err(|_| eyre!("Failed to create keyup event listener"))?;
    mem::forget(on_keyup);

    Ok(())
}
