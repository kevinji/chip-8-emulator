use piston_window::*;

const KEY_COUNT: usize = 16;
static KEY_CODES: [Key; KEY_COUNT] = [
    Key::X, Key::D1, Key::D2, Key::D3, Key::Q, // 0 - 4
    Key::E, Key::A, Key::S, Key::D, Key::W, // 5 - 9
    Key::Z, Key::C, Key::D4, Key::R, Key::F, Key::V, // A - F
];

pub struct Keypad {
    pub key_states: [ButtonState; KEY_COUNT],
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            key_states: [ButtonState::Release; KEY_COUNT],
        }
    }

    pub fn update_key_state(&mut self, key: Key, state: ButtonState) {
        if let Some(i) = KEY_CODES.iter().position(|&saved_key| saved_key == key) {
            self.key_states[i] = state;
        }
    }
}
