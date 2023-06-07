use crate::{cpu::Cpu, keypad::KeyState};
use rand::random;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Opcode {
    /// 0nnn - Jump to a machine code routine at nnn. (ignored)
    SYS,
    /// 00E0 - Clear the display.
    CLS,
    /// 00EE - Return from a subroutine.
    RET,
    /// 1nnn - Jump to location nnn.
    JP { addr: u16 },
    /// 2nnn - Call subroutine at nnn.
    CALL { addr: u16 },
    /// 3xkk - Skip next instruction if Vx = kk.
    SE { vx: usize, byte: u8 },
    /// 4xkk - Skip next instruction if Vx != kk.
    SNE { vx: usize, byte: u8 },
    /// 5xy0 - Skip next instruction if Vx = Vy.
    SE_R { vx: usize, vy: usize },
    /// 6xkk - Set Vx = kk.
    LD { vx: usize, byte: u8 },
    /// 7xkk - Set Vx = Vx + kk.
    ADD { vx: usize, byte: u8 },
    /// 8xy0 - Set Vx = Vy.
    LD_R { vx: usize, vy: usize },
    /// 8xy1 - Set Vx = Vx OR Vy.
    OR_R { vx: usize, vy: usize },
    /// 8xy2 - Set Vx = Vx AND Vy.
    AND_R { vx: usize, vy: usize },
    /// 8xy3 - Set Vx = Vx XOR Vy.
    XOR_R { vx: usize, vy: usize },
    /// 8xy4 - Set Vx = Vx + Vy, set VF = carry.
    ADD_R { vx: usize, vy: usize },
    /// 8xy5 - Set Vx = Vx - Vy, set VF = NOT borrow.
    SUB_R { vx: usize, vy: usize },
    /// 8xy6 - Set Vx = Vx SHR 1.
    SHR { vx: usize },
    /// 8xy7 - Set Vx = Vy - Vx, set VF = NOT borrow.
    SUBN_R { vx: usize, vy: usize },
    /// 8xyE - Set Vx = Vx SHL 1.
    SHL { vx: usize },
    /// 9xy0 - Skip next instruction if Vx != Vy.
    SNE_R { vx: usize, vy: usize },
    /// Annn - Set I = nnn.
    LD_A { addr: u16 },
    /// Bnnn - Jump to location nnn + V0.
    JP_A { addr: u16 },
    /// Cxkk - Set Vx = random byte AND kk.
    RND { vx: usize, byte: u8 },
    /// Dxyn - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    DRW { vx: usize, vy: usize, n: u8 },
    /// Ex9E - Skip next instruction if key with the value of Vx is pressed.
    SKP { vx: usize },
    /// ExA1 - Skip next instruction if key with the value of Vx is not pressed.
    SKNP { vx: usize },
    /// Fx07 - Set Vx = delay timer value.
    LD_R_DT { vx: usize },
    /// Fx0A - Wait for a key press, store the value of the key in Vx.
    LD_R_K { vx: usize },
    /// Fx15 - Set delay timer = Vx.
    LD_DT_R { vx: usize },
    /// Fx18 - Set sound timer = Vx.
    LD_DT_S { vx: usize },
    /// Fx1E - Set I = I + Vx.
    ADD_I { vx: usize },
    /// Fx29 - Set I = location of sprite for digit Vx.
    LD_F { vx: usize },
    /// Fx33 - Store BCD representation of Vx in memory locations I, I+1, and I+2.
    LD_B { vx: usize },
    /// Fx55 - Store registers V0 through Vx in memory starting at location I.
    LD_I_R { vx: usize },
    /// Fx65 - Read registers V0 through Vx from memory starting at location I.
    LD_R_I { vx: usize },
}

impl From<u16> for Opcode {
    fn from(opcode: u16) -> Self {
        match op0(opcode) {
            0x0 => match (op1(opcode), op2(opcode), op3(opcode)) {
                (0x0, 0xE, 0x0) => Self::CLS,
                (0x0, 0xE, 0xE) => Self::RET,
                _ => Self::SYS,
            },
            0x1 => Self::JP { addr: addr(opcode) },
            0x2 => Self::CALL { addr: addr(opcode) },
            0x3 => Self::SE {
                vx: vx(opcode),
                byte: byte(opcode),
            },
            0x4 => Self::SNE {
                vx: vx(opcode),
                byte: byte(opcode),
            },
            0x5 => Self::SE_R {
                vx: vx(opcode),
                vy: vy(opcode),
            },
            0x6 => Self::LD {
                vx: vx(opcode),
                byte: byte(opcode),
            },
            0x7 => Self::ADD {
                vx: vx(opcode),
                byte: byte(opcode),
            },
            0x8 => match op3(opcode) {
                0x0 => Self::LD_R {
                    vx: vx(opcode),
                    vy: vy(opcode),
                },
                0x1 => Self::OR_R {
                    vx: vx(opcode),
                    vy: vy(opcode),
                },
                0x2 => Self::AND_R {
                    vx: vx(opcode),
                    vy: vy(opcode),
                },
                0x3 => Self::XOR_R {
                    vx: vx(opcode),
                    vy: vy(opcode),
                },
                0x4 => Self::ADD_R {
                    vx: vx(opcode),
                    vy: vy(opcode),
                },
                0x5 => Self::SUB_R {
                    vx: vx(opcode),
                    vy: vy(opcode),
                },
                0x6 => Self::SHR { vx: vx(opcode) },
                0x7 => Self::SUBN_R {
                    vx: vx(opcode),
                    vy: vy(opcode),
                },
                0xE => Self::SHL { vx: vx(opcode) },
                _ => unimplemented!("Opcode not implemented!"),
            },
            0x9 => Self::SNE_R {
                vx: vx(opcode),
                vy: vy(opcode),
            },
            0xA => Self::LD_A { addr: addr(opcode) },
            0xB => Self::JP_A { addr: addr(opcode) },
            0xC => Self::RND {
                vx: vx(opcode),
                byte: byte(opcode),
            },
            0xD => Self::DRW {
                vx: vx(opcode),
                vy: vy(opcode),
                n: op3(opcode),
            },
            0xE => match (op2(opcode), op3(opcode)) {
                (0x9, 0xE) => Self::SKP { vx: vx(opcode) },
                (0xA, 0x1) => Self::SKNP { vx: vx(opcode) },
                _ => unimplemented!("Opcode not implemented!"),
            },
            0xF => match (op2(opcode), op3(opcode)) {
                (0x0, 0x7) => Self::LD_R_DT { vx: vx(opcode) },
                (0x0, 0xA) => Self::LD_R_K { vx: vx(opcode) },
                (0x1, 0x5) => Self::LD_DT_R { vx: vx(opcode) },
                (0x1, 0x8) => Self::LD_DT_S { vx: vx(opcode) },
                (0x1, 0xE) => Self::ADD_I { vx: vx(opcode) },
                (0x2, 0x9) => Self::LD_F { vx: vx(opcode) },
                (0x3, 0x3) => Self::LD_B { vx: vx(opcode) },
                (0x5, 0x5) => Self::LD_I_R { vx: vx(opcode) },
                (0x6, 0x5) => Self::LD_R_I { vx: vx(opcode) },
                _ => unimplemented!("Opcode not implemented!"),
            },
            _ => unimplemented!("Opcode not implemented!"),
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
            Self::SYS => (),
            Self::CLS => {
                cpu.view.clear();
            }
            Self::RET => {
                cpu.sp -= 1;
                cpu.pc = cpu.stack[cpu.sp as usize];
            }
            Self::JP { addr } => {
                cpu.pc = addr;
            }
            Self::CALL { addr } => {
                cpu.stack[cpu.sp as usize] = cpu.pc;
                cpu.sp += 1;
                cpu.pc = addr;
            }
            Self::SE { vx, byte } => {
                if cpu.regs[vx] == byte {
                    cpu.push_pc();
                }
            }
            Self::SNE { vx, byte } => {
                if cpu.regs[vx] != byte {
                    cpu.push_pc();
                }
            }
            Self::SE_R { vx, vy } => {
                if cpu.regs[vx] == cpu.regs[vy] {
                    cpu.push_pc();
                }
            }
            Self::LD { vx, byte } => {
                cpu.regs[vx] = byte;
            }
            Self::ADD { vx, byte } => {
                cpu.regs[vx] = cpu.regs[vx].wrapping_add(byte);
            }
            Self::LD_R { vx, vy } => {
                cpu.regs[vx] = cpu.regs[vy];
            }
            Self::OR_R { vx, vy } => {
                cpu.regs[vx] |= cpu.regs[vy];
            }
            Self::AND_R { vx, vy } => {
                cpu.regs[vx] &= cpu.regs[vy];
            }
            Self::XOR_R { vx, vy } => {
                cpu.regs[vx] ^= cpu.regs[vy];
            }
            Self::ADD_R { vx, vy } => {
                let (sum, carry) = cpu.regs[vx].overflowing_add(cpu.regs[vy]);
                cpu.regs[vx] = sum;
                cpu.regs[0xF] = carry.into();
            }
            Self::SUB_R { vx, vy } => {
                let (diff, borrow) = cpu.regs[vx].overflowing_sub(cpu.regs[vy]);
                cpu.regs[vx] = diff;
                cpu.regs[0xF] = (!borrow).into();
            }
            Self::SHR { vx } => {
                cpu.regs[0xF] = cpu.regs[vx] & 0x1;
                cpu.regs[vx] >>= 1;
            }
            Self::SUBN_R { vx, vy } => {
                let (diff, borrow) = cpu.regs[vy].overflowing_sub(cpu.regs[vx]);
                cpu.regs[vx] = diff;
                cpu.regs[0xF] = (!borrow).into();
            }
            Self::SHL { vx } => {
                cpu.regs[0xF] = cpu.regs[vx] >> 7;
                cpu.regs[vx] <<= 1;
            }
            Self::SNE_R { vx, vy } => {
                if cpu.regs[vx] != cpu.regs[vy] {
                    cpu.push_pc();
                }
            }
            Self::LD_A { addr } => {
                cpu.i_reg = addr;
            }
            Self::JP_A { addr } => {
                cpu.pc = u16::from(cpu.regs[0x0]) + addr;
            }
            Self::RND { vx, byte } => {
                cpu.regs[vx] = random::<u8>() & byte;
            }
            Self::DRW { vx, vy, n } => {
                // TODO: Set VF for collision
                for y in 0..n {
                    let byte = cpu.memory[cpu.i_reg as usize + y as usize];
                    for x in 0..8 {
                        let bit = (byte >> (7 - x)) & 1;
                        cpu.view
                            .draw_pixel(cpu.regs[vx] + x, cpu.regs[vy] + y, bit == 1);
                    }
                }
            }
            Self::SKP { vx } => {
                let (keypad, _) = &*cpu.keypad_and_keypress;
                let key_states = keypad.lock().unwrap().key_states;
                if key_states[cpu.regs[vx] as usize] == KeyState::Down {
                    cpu.push_pc();
                }
            }
            Self::SKNP { vx } => {
                let (keypad, _) = &*cpu.keypad_and_keypress;
                let key_states = keypad.lock().unwrap().key_states;
                if key_states[cpu.regs[vx] as usize] == KeyState::Up {
                    cpu.push_pc();
                }
            }
            Self::LD_R_DT { vx } => {
                cpu.regs[vx] = cpu.delay_timer;
            }
            Self::LD_R_K { vx } => {
                let (keypad, keypress) = &*cpu.keypad_and_keypress;
                let keypad = keypad.lock().unwrap();
                let last_keypress = keypress.wait(keypad).unwrap().last_keypress.unwrap();
                cpu.regs[vx] = last_keypress as u8;
            }
            Self::LD_DT_R { vx } => {
                cpu.delay_timer = cpu.regs[vx];
            }
            Self::LD_DT_S { vx } => {
                cpu.sound_timer = cpu.regs[vx];
            }
            Self::ADD_I { vx } => {
                cpu.i_reg = cpu.i_reg.wrapping_add(vx as u16);
            }
            Self::LD_F { vx } => {
                cpu.i_reg = vx as u16 * 5;
            }
            Self::LD_B { vx } => {
                let mut vx_val = cpu.regs[vx];
                for i in (0..=2).rev() {
                    cpu.memory[cpu.i_reg as usize + i] = vx_val % 10;
                    vx_val /= 10;
                }
            }
            Self::LD_I_R { vx } => {
                for vi in 0..=vx {
                    cpu.memory[cpu.i_reg as usize + vi] = cpu.regs[vi];
                }
            }
            Self::LD_R_I { vx } => {
                for vi in 0..=vx {
                    cpu.regs[vi] = cpu.memory[cpu.i_reg as usize + vi];
                }
            }
        }
    }
}
