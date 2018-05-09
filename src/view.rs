use failure::{self, Error};
use piston_window::*;

const COLS: u32 = 64;
const ROWS: u32 = 32;
const SCALE: u32 = 10;

pub struct View {
    pub window: PistonWindow,
    pub should_draw: bool,
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
            should_draw: false,
        })
    }

    pub fn clear<G>(&self, g: &mut G)
        where G: Graphics
    {
        clear([0., 0., 0., 1.], // black
              g);
    }

    pub fn draw_pixel<G>(&self, c: context::Context, g: &mut G)
        where G: Graphics
    {
        rectangle([1., 1., 1., 1.], // white
                  [0., 0., SCALE.into(), SCALE.into()],
                  c.transform, g);
    }
}
