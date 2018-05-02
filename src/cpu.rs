use opcode::Opcode;

pub struct Cpu {
    pub memory: [u8; 4096],

    pub regs: [u8; 16],
    pub i_reg: u16, // Index register
    pub pc: u16, // Program counter

    pub stack: [u8; 16],
    pub sp: u8, // Stack pointer

    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        let cpu = Cpu {
            memory: [0; 4096],

            regs: [0; 16],
            i_reg: 0,
            pc: 0x200,

            stack: [0; 16],
            sp: 0,

            delay_timer: 0,
            sound_timer: 0,
        };

        // TODO: Load fontset into memory.

        cpu
    }

    pub fn load_program(&mut self, program: &[u8]) {
        assert!(self.memory.len() >= 0x200 + program.len(), "Program does not fit in memory.");

        // Fill memory from 0x200.
        self.memory[0x200..0x200+program.len()].copy_from_slice(program)
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.decode_and_execute_opcode(opcode);
        self.update_timers();
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
