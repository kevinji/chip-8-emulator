use failure::Error;
use piston_window::*;

use keypad::Keypad;
use view::{GameState, View};
use opcode::Opcode;

pub struct Cpu {
    pub memory: [u8; 4096],

    pub regs: [u8; 16],
    pub i_reg: u16, // Index register
    pub pc: u16, // Program counter

    pub stack: [u16; 16],
    pub sp: u8, // Stack pointer

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub keypad: Keypad,
    pub view: View,
}

static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Cpu {
    pub fn new(rom_buf: &Vec<u8>) -> Result<Self, Error> {
        let mut cpu = Cpu {
            memory: [0; 4096],

            regs: [0; 16],
            i_reg: 0,
            pc: 0x200,

            stack: [0; 16],
            sp: 0,

            delay_timer: 0,
            sound_timer: 0,

            keypad: Keypad::new(),
            view: View::new()?,
        };

        // Store font data before 0x200.
        cpu.memory[..FONTSET.len()].copy_from_slice(&FONTSET);

        // Load the chosen ROM into memory.
        cpu.load_rom(rom_buf);

        Ok(cpu)
    }

    pub fn load_rom(&mut self, program: &[u8]) {
        assert!(self.memory.len() >= 0x200 + program.len(), "Program does not fit in memory.");

        // Fill memory from 0x200.
        self.memory[0x200..0x200+program.len()].copy_from_slice(program);
    }

    pub fn cycle(&mut self) {
        while let Some(e) = self.view.window.next() {
            // println!("{:?}", e);
            match e {
                Event::Input(input) => {
                    if let Input::Button(button_args) = input {
                        if let Button::Keyboard(key) = button_args.button {
                            self.keypad.update_key_state(key, button_args.state);
                        }
                    }
                },
                Event::Loop(loop_e) => match loop_e {
                    Loop::Update(_update_args) => {
                        let opcode = self.fetch_opcode();
                        self.decode_and_execute_opcode(opcode);
                        self.update_timers();
                    },
                    Loop::Render(_render_args) => {
                        if self.view.state != GameState::Idle {
                            self.view.window.draw_2d(&e, |c, g| {
                            });
                            self.view.state = GameState::Idle;
                        }
                    },
                    _ => (),
                },
                Event::Custom(_, _) => (),
            }
        }
    }

    fn fetch_opcode(&self) -> u16 {
        assert!(self.pc < 4095, "pc is outside memory bounds!");

        // Opcode is 2 bytes, big-endian.
        (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[(self.pc + 1) as usize] as u16)
    }

    fn decode_and_execute_opcode(&mut self, opcode: u16) {
        self.push_pc();

        let opcode = Opcode::from(opcode);
        opcode.execute(self);
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn push_pc(&mut self) {
        self.pc += 2;
    }
}
