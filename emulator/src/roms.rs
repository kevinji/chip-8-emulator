use std::{collections::HashMap, sync::OnceLock};

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

// TODO: Replace this with `LazyLock` once rust-lang/rust#109736 is stabilized.
pub fn roms_by_name() -> &'static HashMap<String, Vec<u8>> {
    static LOCK: OnceLock<HashMap<String, Vec<u8>>> = OnceLock::new();
    LOCK.get_or_init(|| {
        include_roms!(
            "15PUZZLE", "BLINKY", "BLITZ", "BRIX", "CONNECT4", "GUESS", "HIDDEN", "IBM",
            "INVADERS", "KALEID", "MAZE", "MERLIN", "MISSILE", "PONG", "PONG2", "PUZZLE", "SYZYGY",
            "TANK", "TETRIS", "TICTAC", "UFO", "VBRIX", "VERS", "WIPEOFF",
        )
        .into_iter()
        .collect()
    })
}
