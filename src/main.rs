extern crate failure;
extern crate piston_window;
extern crate rand;

mod cpu;
mod graphics;
mod keypad;
mod opcode;

use failure::Error;

use cpu::Cpu;

fn main() {
    ::std::process::exit(match app() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            1
        }
    })
}

fn app() -> Result<(), Error> {
    let mut cpu = Cpu::new()?;
    cpu.cycle();
    Ok(())
}
