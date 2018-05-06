use failure::{self, Error};
use piston_window::*;

pub struct Graphics {
    pub window: PistonWindow,
}

impl Graphics {
    pub fn new() -> Result<Self, Error> {
        let window = WindowSettings::new("CHIP-8 Emulator", (640, 320))
            .exit_on_esc(true)
            .build()
            .map_err(failure::err_msg)?;

        Ok(Graphics {
            window: window,
        })
    }
}
