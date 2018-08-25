use failure::{self, Error};

const COLS: u32 = 64;
const ROWS: u32 = 32;
const SCALE: u32 = 10;

#[derive(PartialEq)]
pub enum GameState {
    Idle,
    ClearScreen,
    DrawSprite { sprite: Vec<u8>, y: u32, x: u32 },
}

pub struct View {
    pub state: GameState,
}

impl View {
    pub fn new() -> Result<Self, Error> {
        /*
        let window = WindowSettings::new("CHIP-8 Emulator",
                                         (COLS * SCALE, ROWS * SCALE))
            .resizable(false)
            .exit_on_esc(true)
            .build::<PistonWindow>()
            .map_err(failure::err_msg)?
            .max_fps(60)
            .ups(60);
        */

        Ok(View {
            state: GameState::Idle,
        })
    }
}
