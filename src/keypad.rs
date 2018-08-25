const KEY_COUNT: usize = 16;
static KEY_CODES: &[&str] = &[
    "KeyX", "Digit1", "Digit2", "Digit3", "KeyQ", // 0 - 4
    "KeyE", "KeyA", "KeyS", "KeyD", "KeyW", // 5 - 9
    "KeyZ", "KeyC", "Digit4", "KeyR", "KeyF", "KeyV", // A - F
];

pub struct Keypad {
    pub key_states: [&'static str; KEY_COUNT],
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            key_states: ["Release"; KEY_COUNT],
        }
    }

    pub fn update_key_state(&mut self, key: &str, state: &'static str) {
        if let Some(i) = KEY_CODES.iter().position(|&saved_key| saved_key == key) {
            self.key_states[i] = state;
        }
    }
}
