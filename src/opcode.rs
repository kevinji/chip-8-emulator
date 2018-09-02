use rand::random;

use cpu::Cpu;
use keypad::{self, KeyState, KEYPAD};
use view;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Opcode {
    SYS, // 0nnn - Jump to a machine code routine at nnn. (ignored)
    CLS, // 00E0 - Clear the display.
    RET, // 00EE - Return from a subroutine.
    JP { addr: u16 }, // 1nnn - Jump to location nnn.
    CALL { addr: u16 }, // 2nnn - Call subroutine at nnn.
    SE { vx: usize, byte: u8 }, // 3xkk - Skip next instruction if Vx = kk.
    SNE { vx: usize, byte: u8 }, // 4xkk - Skip next instruction if Vx != kk.
    SE_R { vx: usize, vy: usize }, // 5xy0 - Skip next instruction if Vx = Vy.
    LD { vx: usize, byte: u8 }, // 6xkk - Set Vx = kk.
    ADD { vx: usize, byte: u8 }, // 7xkk - Set Vx = Vx + kk.
    LD_R { vx: usize, vy: usize }, // 8xy0 - Set Vx = Vy.
    OR_R { vx: usize, vy: usize }, // 8xy1 - Set Vx = Vx OR Vy.
    AND_R { vx: usize, vy: usize }, // 8xy2 - Set Vx = Vx AND Vy.
    XOR_R { vx: usize, vy: usize }, // 8xy3 - Set Vx = Vx XOR Vy.
    ADD_R { vx: usize, vy: usize }, // 8xy4 - Set Vx = Vx + Vy, set VF = carry.
    SUB_R { vx: usize, vy: usize }, // 8xy5 - Set Vx = Vx - Vy, set VF = NOT borrow.
    SHR { vx: usize }, // 8xy6 - Set Vx = Vx SHR 1.
    SUBN_R { vx: usize, vy: usize }, // 8xy7 - Set Vx = Vy - Vx, set VF = NOT borrow.
    SHL { vx: usize }, // 8xyE - Set Vx = Vx SHL 1.
    SNE_R { vx: usize, vy: usize }, // 9xy0 - Skip next instruction if Vx != Vy.
    LD_A { addr: u16 }, // Annn - Set I = nnn.
    JP_A { addr: u16 }, // Bnnn - Jump to location nnn + V0.
    RND { vx: usize, byte: u8 }, // Cxkk - Set Vx = random byte AND kk.
    DRW { vx: usize, vy: usize, n: u8 }, // Dxyn - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    SKP { vx: usize }, // Ex9E - Skip next instruction if key with the value of Vx is pressed.
    SKNP { vx: usize }, // ExA1 - Skip next instruction if key with the value of Vx is not pressed.
    LD_R_DT { vx: usize }, // Fx07 - Set Vx = delay timer value.
    LD_R_K { vx: usize }, // Fx0A - Wait for a key press, store the value of the key in Vx.
    LD_DT_R { vx: usize }, // Fx15 - Set delay timer = Vx.
    LD_DT_S { vx: usize }, // Fx18 - Set sound timer = Vx.
    ADD_I { vx: usize }, // Fx1E - Set I = I + Vx.
    LD_F { vx: usize }, // Fx29 - Set I = location of sprite for digit Vx.
    LD_B { vx: usize }, // Fx33 - Store BCD representation of Vx in memory locations I, I+1, and I+2.
    LD_I_R { vx: usize }, // Fx55 - Store registers V0 through Vx in memory starting at location I.
    LD_R_I { vx: usize }, // Fx65 - Read registers V0 through Vx from memory starting at location I.
}

impl From<u16> for Opcode {
    fn from(opcode: u16) -> Self {
        match op0(opcode) {
            0x0 => match (op1(opcode), op2(opcode), op3(opcode)) {
                (0x0, 0xE, 0x0) => Opcode::CLS,
                (0x0, 0xE, 0xE) => Opcode::RET,
                _ => Opcode::SYS,
            },
            0x1 => Opcode::JP { addr: addr(opcode) },
            0x2 => Opcode::CALL { addr: addr(opcode) },
            0x3 => Opcode::SE { vx: vx(opcode), byte: byte(opcode) },
            0x4 => Opcode::SNE { vx: vx(opcode), byte: byte(opcode) },
            0x5 => Opcode::SE_R { vx: vx(opcode), vy: vy(opcode) },
            0x6 => Opcode::LD { vx: vx(opcode), byte: byte(opcode) },
            0x7 => Opcode::ADD { vx: vx(opcode), byte: byte(opcode) },
            0x8 => match op3(opcode) {
                0x0 => Opcode::LD_R { vx: vx(opcode), vy: vy(opcode) },
                0x1 => Opcode::OR_R { vx: vx(opcode), vy: vy(opcode) },
                0x2 => Opcode::AND_R { vx: vx(opcode), vy: vy(opcode) },
                0x3 => Opcode::XOR_R { vx: vx(opcode), vy: vy(opcode) },
                0x4 => Opcode::ADD_R { vx: vx(opcode), vy: vy(opcode) },
                0x5 => Opcode::SUB_R { vx: vx(opcode), vy: vy(opcode) },
                0x6 => Opcode::SHR { vx: vx(opcode) },
                0x7 => Opcode::SUBN_R { vx: vx(opcode), vy: vy(opcode) },
                0xE => Opcode::SHL { vx: vx(opcode) },
                _ => panic!("Opcode not implemented!"),
            },
            0x9 => Opcode::SNE_R { vx: vx(opcode), vy: vy(opcode) },
            0xA => Opcode::LD_A { addr: addr(opcode) },
            0xB => Opcode::JP_A { addr: addr(opcode) },
            0xC => Opcode::RND { vx: vx(opcode), byte: byte(opcode) },
            0xD => Opcode::DRW { vx: vx(opcode), vy: vy(opcode), n: op3(opcode) },
            0xE => match (op2(opcode), op3(opcode)) {
                (0x9, 0xE) => Opcode::SKP { vx: vx(opcode) },
                (0xA, 0x1) => Opcode::SKNP { vx: vx(opcode) },
                _ => panic!("Opcode not implemented!"),
            },
            0xF => match (op2(opcode), op3(opcode)) {
                (0x0, 0x7) => Opcode::LD_R_DT { vx: vx(opcode) },
                (0x0, 0xA) => Opcode::LD_R_K { vx: vx(opcode) },
                (0x1, 0x5) => Opcode::LD_DT_R { vx: vx(opcode) },
                (0x1, 0x8) => Opcode::LD_DT_S { vx: vx(opcode) },
                (0x1, 0xE) => Opcode::ADD_I { vx: vx(opcode) },
                (0x2, 0x9) => Opcode::LD_F { vx: vx(opcode) },
                (0x3, 0x3) => Opcode::LD_B { vx: vx(opcode) },
                (0x5, 0x5) => Opcode::LD_I_R { vx: vx(opcode) },
                (0x6, 0x5) => Opcode::LD_R_I { vx: vx(opcode) },
                _ => panic!("Opcode not implemented!"),
            },
            _ => panic!("Opcode not implemented!"),
        }
    }
}

#[inline]
fn op0(opcode: u16) -> u8 {
    ((opcode & 0xF000) >> 12) as u8
}

#[inline]
fn op1(opcode: u16) -> u8 {
    ((opcode & 0x0F00) >> 8) as u8
}

#[inline]
fn vx(opcode: u16) -> usize {
    op1(opcode) as usize
}

#[inline]
fn op2(opcode: u16) -> u8 {
    ((opcode & 0x00F0) >> 4) as u8
}

#[inline]
fn vy(opcode: u16) -> usize {
    op2(opcode) as usize
}

#[inline]
fn op3(opcode: u16) -> u8 {
    (opcode & 0x000F) as u8
}

#[inline]
fn byte(opcode: u16) -> u8 {
    (opcode & 0x00FF) as u8
}

#[inline]
fn addr(opcode: u16) -> u16 {
    opcode & 0x0FFF
}

impl Opcode {
    pub fn execute(&self, cpu: &mut Cpu) {
        match *self {
            Opcode::SYS => (),
            Opcode::CLS => {
                view::clear();
            },
            Opcode::RET => {
                cpu.sp -= 1;
                cpu.pc = cpu.stack[cpu.sp as usize];
            },
            Opcode::JP { addr } => {
                cpu.pc = addr;
            },
            Opcode::CALL { addr } => {
                cpu.stack[cpu.sp as usize] = cpu.pc;
                cpu.sp += 1;
                cpu.pc = addr;
            },
            Opcode::SE { vx, byte } => {
                if cpu.regs[vx] == byte {
                    cpu.push_pc();
                }
            },
            Opcode::SNE { vx, byte } => {
                if cpu.regs[vx] != byte {
                    cpu.push_pc();
                }
            },
            Opcode::SE_R { vx, vy } => {
                if cpu.regs[vx] == cpu.regs[vy] {
                    cpu.push_pc();
                }
            },
            Opcode::LD { vx, byte } => {
                cpu.regs[vx] = byte;
            },
            Opcode::ADD { vx, byte } => {
                cpu.regs[vx] = cpu.regs[vx].wrapping_add(byte);
            },
            Opcode::LD_R { vx, vy } => {
                cpu.regs[vx] = cpu.regs[vy];
            },
            Opcode::OR_R { vx, vy } => {
                cpu.regs[vx] |= cpu.regs[vy];
            },
            Opcode::AND_R { vx, vy } => {
                cpu.regs[vx] &= cpu.regs[vy];
            },
            Opcode::XOR_R { vx, vy } => {
                cpu.regs[vx] ^= cpu.regs[vy];
            },
            Opcode::ADD_R { vx, vy } => {
                let (sum, carry) = cpu.regs[vx].overflowing_add(cpu.regs[vy]);
                cpu.regs[vx] = sum;
                cpu.regs[0xF] = carry as u8;
            },
            Opcode::SUB_R { vx, vy } => {
                let (diff, borrow) = cpu.regs[vx].overflowing_sub(cpu.regs[vy]);
                cpu.regs[vx] = diff;
                cpu.regs[0xF] = !borrow as u8;
            },
            Opcode::SHR { vx } => {
                cpu.regs[0xF] = cpu.regs[vx] & 0x1;
                cpu.regs[vx] >>= 1;
            },
            Opcode::SUBN_R { vx, vy } => {
                let (diff, borrow) = cpu.regs[vy].overflowing_sub(cpu.regs[vx]);
                cpu.regs[vx] = diff;
                cpu.regs[0xF] = !borrow as u8;
            },
            Opcode::SHL { vx } => {
                cpu.regs[0xF] = cpu.regs[vx] >> 7;
                cpu.regs[vx] <<= 1;
            },
            Opcode::SNE_R { vx, vy } => {
                if cpu.regs[vx] != cpu.regs[vy] {
                    cpu.push_pc();
                }
            },
            Opcode::LD_A { addr } => {
                cpu.i_reg = addr;
            },
            Opcode::JP_A { addr } => {
                cpu.pc = (cpu.regs[0x0] as u16) + addr;
            },
            Opcode::RND { vx, byte } => {
                cpu.regs[vx] = random::<u8>() & byte;
            },
            Opcode::DRW { vx: _, vy: _, n: _ } => (), // Dxyn - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            Opcode::SKP { vx } => {
                let key_states = KEYPAD.lock().unwrap().key_states;
                if key_states[cpu.regs[vx] as usize] == KeyState::Down {
                    cpu.push_pc();
                }
            },
            Opcode::SKNP { vx } => {
                let key_states = KEYPAD.lock().unwrap().key_states;
                if key_states[cpu.regs[vx] as usize] == KeyState::Up {
                    cpu.push_pc();
                }
            },
            Opcode::LD_R_DT { vx } => {
                cpu.regs[vx] = cpu.delay_timer;
            },
            Opcode::LD_R_K { vx } => {
                cpu.regs[vx] = keypad::wait_for_key_press() as u8;
            },
            Opcode::LD_DT_R { vx } => {
                cpu.delay_timer = cpu.regs[vx];
            },
            Opcode::LD_DT_S { vx } => {
                cpu.sound_timer = cpu.regs[vx];
            },
            Opcode::ADD_I { vx } => {
                cpu.i_reg = cpu.i_reg.wrapping_add(vx as u16);
            },
            Opcode::LD_F { vx } => {
                cpu.i_reg = vx as u16 * 5;
            },
            Opcode::LD_B { vx } => {
                let mut vx_val = cpu.regs[vx];
                for i in 2..=0 {
                    cpu.memory[cpu.i_reg as usize + i] = vx_val % 10;
                    vx_val /= 10;
                }
            },
            Opcode::LD_I_R { vx } => {
                for vi in 0..=vx {
                    cpu.memory[cpu.i_reg as usize + vi] = cpu.regs[vi];
                }
            },
            Opcode::LD_R_I { vx } => {
                for vi in 0..=vx {
                    cpu.regs[vi] = cpu.memory[cpu.i_reg as usize + vi];
                }
            },
        }
    }
}
