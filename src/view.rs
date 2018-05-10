use failure::{self, Error};
use piston_window::*;

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
    pub window: PistonWindow,
    pub state: GameState,
}

impl View {
    pub fn new() -> Result<Self, Error> {
        let window = WindowSettings::new("CHIP-8 Emulator",
                                         (COLS * SCALE, ROWS * SCALE))
            .resizable(false)
            .exit_on_esc(true)
            .build::<PistonWindow>()
            .map_err(failure::err_msg)?
            .max_fps(60)
            .ups(60);

        Ok(View {
            window: window,
            state: GameState::Idle,
        })
    }

    pub fn clear<G>(&self, g: &mut G)
        where G: Graphics
    {
        clear([0., 0., 0., 1.], // black
              g);
    }

    pub fn draw_sprite<G>(&self, c: context::Context, g: &mut G, y: u32, x: u32,
                          sprite: &Vec<u8>)
        where G: Graphics
    {
        for (row_i, row) in sprite.iter().enumerate() {
            let mut row_copy = row;
            for col_i in 0..8 {
                let color = (row_copy & 1) as f32;
                self.draw_pixel(c, g, y + row_i as u32, x + 7 - col_i as u32,
                                color);
            }
        }
    }

    fn draw_pixel<G>(&self, c: context::Context, g: &mut G, y: u32, x: u32,
                     color: f32)
        where G: Graphics
    {
        rectangle([color, color, color, 1.],
                  [y.into(), x.into(), SCALE.into(), SCALE.into()],
                  c.transform, g);
    }
}
