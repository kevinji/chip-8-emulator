use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules! include_roms {
    ( $( $x:expr ),* $(,)? ) => {
        [
            $(
                (
                    $x.to_owned(),
                    include_bytes!(concat!("../../roms/", $x, ".rom")).to_vec(),
                ),
            )*
        ]
    };
}

lazy_static! {
    pub static ref ROMS: HashMap<String, Vec<u8>> = include_roms!(
        "15PUZZLE", "BLINKY", "BLITZ", "BRIX", "CONNECT4", "GUESS", "HIDDEN", "IBM", "INVADERS",
        "KALEID", "MAZE", "MERLIN", "MISSILE", "PONG", "PONG2", "PUZZLE", "SYZYGY", "TANK",
        "TETRIS", "TICTAC", "UFO", "VBRIX", "VERS", "WIPEOFF",
    )
    .into_iter()
    .collect();
}
