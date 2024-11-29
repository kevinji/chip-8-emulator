use std::{collections::HashMap, sync::LazyLock};

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

pub static ROMS_BY_NAME: LazyLock<HashMap<String, Vec<u8>>> = LazyLock::new(|| {
    include_roms!(
        "15PUZZLE", "BLINKY", "BLITZ", "BRIX", "CONNECT4", "GUESS", "HIDDEN", "IBM", "INVADERS",
        "KALEID", "MAZE", "MERLIN", "MISSILE", "PONG", "PONG2", "PUZZLE", "SYZYGY", "TANK",
        "TETRIS", "TICTAC", "UFO", "VBRIX", "VERS", "WIPEOFF",
    )
    .into_iter()
    .collect()
});
