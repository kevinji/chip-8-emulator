use crate::{keypad::Keypad, opcode::Opcode, view::View};
use std::sync::{Arc, Condvar, Mutex};

const TOTAL_MEMORY_BYTES: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const PROGRAM_START_ADDRESS: u16 = 0x200;

#[derive(Debug)]
pub struct Cpu {
    pub memory: [u8; TOTAL_MEMORY_BYTES],

    pub regs: [u8; REGISTER_COUNT],
    /// Index register
    pub i_reg: u16,
    /// Program counter
    pub pc: u16,

    pub stack: [u16; STACK_SIZE],
    /// Stack pointer
    pub sp: u8,

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub view: View,
    pub keypad_and_keypress: Arc<(Mutex<Keypad>, Condvar)>,
}

const FONTSET: [u8; 80] = [
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

const _: () = assert!(FONTSET.len() <= PROGRAM_START_ADDRESS as usize);

impl Cpu {
    #[must_use]
    pub fn new(
        rom_buf: &[u8],
        view: View,
        keypad_and_keypress: Arc<(Mutex<Keypad>, Condvar)>,
    ) -> Self {
        let mut cpu = Self {
            memory: [0; TOTAL_MEMORY_BYTES],

            regs: [0; REGISTER_COUNT],
            i_reg: 0,
            pc: PROGRAM_START_ADDRESS,

            stack: [0; STACK_SIZE],
            sp: 0,

            delay_timer: 0,
            sound_timer: 0,

            view,
            keypad_and_keypress,
        };

        // Store font data before `PROGRAM_START_ADDRESS`.
        cpu.memory[..FONTSET.len()].copy_from_slice(&FONTSET);

        // Load the chosen ROM into memory.
        cpu.load_rom(rom_buf);

        cpu
    }

    /// # Panics
    /// Panics if the program does not fit into memory.
    pub fn load_rom(&mut self, program: &[u8]) {
        assert!(
            self.memory.len() >= PROGRAM_START_ADDRESS as usize + program.len(),
            "Program does not fit in memory.",
        );

        // Fill memory from `PROGRAM_START_ADDRESS`.
        self.memory[PROGRAM_START_ADDRESS as usize..PROGRAM_START_ADDRESS as usize + program.len()]
            .copy_from_slice(program);
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.decode_and_execute_opcode(opcode);

        // TODO: Ensure this runs at 60Hz
        self.update_timers();
    }

    fn fetch_opcode(&self) -> u16 {
        assert!(
            (self.pc as usize) < TOTAL_MEMORY_BYTES - 1,
            "pc is outside memory bounds!",
        );

        // Opcode is 2 bytes, big-endian.
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16)
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
