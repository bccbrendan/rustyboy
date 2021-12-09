use std::rc::Rc;
use std::cell::RefCell;
use super::memory::Memory;

// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
#[derive(Copy, Clone)]
pub enum Flag {
    // This bit is set if and only if the result of an operation is zero. Used by conditional jumps.
    Z = 1 << 7, 

    /* These flags are used by the DAA instruction only. 
    N indicates whether the previous instruction has been a subtraction, 
    and H indicates carry for the lower 4 bits of the result. 
    DAA also uses the C flag, which must indicate carry for the upper 4 bits. 
    After adding/subtracting two BCD numbers, 
    DAA is used to convert the result to BCD format. 
    BCD numbers range from $00 to $99 rather than $00 to $FF. 
    Because only two flags (C and H) exist to indicate carry-outs of BCD digits, 
    DAA is ineffective for 16-bit operations (which have 4 digits), 
    and use for INC/DEC operations (which do not affect C-flag) has limits.
    */
    N = 1 << 6,
    H = 1 << 5,
    /* Is set in these cases:
    When the result of an 8-bit addition is higher than $FF.
    When the result of a 16-bit addition is higher than $FFFF.
    When the result of a subtraction or comparison is lower than zero (like in Z80 and 80x86 CPUs, but unlike in 65XX and ARM CPUs).
    When a rotate/shift operation shifts out a “1” bit.
    Used by conditional jumps and instructions such as ADC, SBC, RL, RLA, etc.
    */
    C = 1 << 4,
}

pub struct Cpu {
    pub mmu: Rc<RefCell<dyn Memory>>,
    flags: u8,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,  // stack pointer
    pc: u16,  // program counter
}


impl Cpu {
    pub fn init(mmu: Rc<RefCell<dyn Memory>>) -> Cpu {
        // https://gbdev.io/pandocs/Power_Up_Sequence.html
        Self {
            mmu,
            flags: Flag::Z as u8,
            a: 0xFF,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4d,
            sp: 0xfffe,
            pc: 0x0100,
        }
    }

    fn set_flag(&mut self, f: Flag, set: u8) {
        match set {
            1 => self.flags |= f as u8,
            0 => self.flags &= !(f as u8),
            _ => panic!("Invalid arg to set_flag: {}", set),
        }
    }

    pub fn emulate_operation(&mut self) -> u32{
        let opcode = self.fetch();
        // https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
        let cycles = match opcode {
            // CPU Control Instructions
            0x00 => {rog::debugln!("[{:#04X}] NOP", self.pc - 1); 4},
            // Jump instructions
            0xC2 | 0xC3 | 0xCA | 0xD2 | 0xDA | 0xE9 | // jp 
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 |  // jr
            0xC4 | 0xCC | 0xCD | 0xD4 | 0xDC |  // call
            0xC0 | 0xC8 | 0xC9 | 0xD0 | 0xD8 | 0xD9  => self.emulate_jump_operation(opcode),
            // LD operations
            0x02 | 0x06 | 0x08 | 0x0A | 0x0E |
            0x12 | 0x16 | 0x18 | 0x1A | 0x1E |
            0x22 | 0x26 | 0x28 | 0x2A | 0x2E |
            0x32 | 0x36 | 0x38 | 0x3A | 0x3E |
            0x40 ..= 0x7F |
            0xE0 | 0xE2 | 0xEA |
            0xF0 | 0xF2 | 0xF8 | 0xF9 | 0xFA => self.emulate_load_operation(opcode),
            0x07 | 0x17 | 0x0F | 0x1F | 0xCB => self.emulate_8bit_rotation_or_shift(opcode),
            _ => panic!("Unrecognized opcode {:#02x} at addr {:#04x}", opcode, self.pc - 1),
        };
        // return cycles taken (in hardware clock cycles)
        cycles
    }

    fn fetch(&mut self) -> u8 {
        let b = self.mmu.borrow().read8(self.pc);
        self.pc += 1;
        b
    }

    fn fetch16(&mut self) -> u16 {
        let w = self.mmu.borrow().read16(self.pc);
        self.pc += 2;
        w
    }

    fn emulate_jump_operation(&mut self, opcode: u8) -> u32 {
        match opcode {
            0xC2 => panic!("unimplemented!"),
            0xC3 => { // jp
                let target = self.fetch16();
                rog::debugln!("[{:#02X}] JP {:#04X}", self.pc - 3, target);
                self.pc = target;
                16
            },
            0xCA | 0xD2 | 0xDA | 0xE9 | // jp 
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 |  // jr
            0xC4 | 0xCC => panic!("unimplemented opcode: {:#02x}", opcode),
            0xCD => { // call
                let target = self.fetch16();
                rog::debugln!("[{:#02X}] CALL {:#04X}", self.pc - 3, target);
                self.sp = self.sp - 2;
                self.mmu.borrow_mut().write16(self.sp, self.pc);
                self.pc = target;
                24
            },
            0xD4 | 0xDC | 0xC0 | 0xC8 | 0xC9 | 0xD0 | 0xD8 | 0xD9  => panic!("unimplemented opcode: {:#02x}", opcode),
            _ => panic!("unexpected opcode: {:#02x}", opcode),
        }
    }


    fn emulate_load_operation(&mut self, opcode: u8) -> u32 {
        match opcode {
            0x02 | 0x06 | 0x08 | 0x0A | 0x0E |
            0x12 | 0x16 | 0x18 | 0x1A | 0x1E |
            0x22 | 0x26 | 0x28 | 0x2A | 0x2E |
            0x32 | 0x36 | 0x38 | 0x3A | 0x3E |
            0x40 ..= 0x7F |
            0xE2 | 0xEA |
            0xF2 | 0xF8 | 0xF9 | 0xFA => panic!("LD Not yet implemented: {:#02X}", opcode),
            0xE0 => {
                let n = self.fetch();
                rog::debugln!("[{:#04X}] LD (FF00 + {:#02X}), A", self.pc - 2, n);
                self.mmu.borrow_mut().write8(0xFF00 + n as u16, self.a);
                12
            }
            0xF0 => {
                let n = self.fetch();
                rog::debugln!("[{:#04X}] LD A, 0xFF00+{:#02X}", self.pc - 2, n);
                self.a = self.mmu.borrow().read8(0xFF00 + n as u16);
                12
            }
            _ => panic!("Unsupported LD opcode: {:#02X}", opcode),
        }
    }

    fn emulate_8bit_rotation_or_shift(&mut self, opcode: u8) -> u32 {
        match opcode {
            // RLCA RLA RRCA RRA 
            0x07 => { 
                self.a = self.op_rlc(self.a, false);
                4
            },
            0x17 => {
                self.a = self.op_rl(self.a, false);
                4
            },
            0x0F => {
                self.a = self.op_rrc(self.a, false);
                4
            },
            0x1F => {
                self.a = self.op_rr(self.a, false);
                4
            },
            0xCB => {
                let cb_opcode = self.fetch();
                let operand = match cb_opcode & 0x7 {
                    0x0 => self.b,
                    0x1 => self.c,
                    0x2 => self.d,
                    0x3 => self.e,
                    0x4 => self.h,
                    0x5 => self.l,
                    0x6 => {
                        let addr = (self.h as u16) << 8 | self.l as u16;
                        self.mmu.borrow().read8(addr)
                    },
                    0x7 => self.a,
                    _ => panic!("invalid cb_opcode? {:#02X}", cb_opcode),
                };
                let result = match cb_opcode {
                    // https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
                    0x00 ..= 0x07 => self.op_rlc(operand, true),
                    0x08 ..= 0x0F => self.op_rrc(operand, true),
                    0x10 ..= 0x17 => self.op_rl(operand, true),
                    0x18 ..= 0x1F => self.op_rr(operand, true),
                    0x20 ..= 0x27 => self.op_sla(operand),
                    0x28 ..= 0x2F => self.op_sra(operand),
                    0x30 ..= 0x37 => self.op_swap(operand),
                    0x38 ..= 0x3F => self.op_srl(operand),
                    0x40 ..= 0x7F => self.op_bit(operand, (cb_opcode - 0x40) >> 3),
                    0x80 ..= 0xBF => self.op_res(operand, (cb_opcode - 0x80) >> 3),
                    0xC0 ..= 0xFF => self.op_set(operand, (cb_opcode - 0xC0) >> 3),
                };
                match cb_opcode & 0x7 {
                    0x0 => self.b = result,
                    0x1 => self.c = result,
                    0x2 => self.d = result,
                    0x3 => self.e = result,
                    0x4 => self.h = result,
                    0x5 => self.l = result,
                    0x6 => {
                        let addr = (self.h as u16) << 8 | self.l as u16;
                        self.mmu.borrow_mut().write8(addr, result)
                    },
                    0x7 => self.a = result,
                    _ => panic!("invalid cb_opcode? {:#02X}", cb_opcode),

                }
                let cpu_cycles = if cb_opcode & 0x7 == 0x6 { 16 } else { 8 };
                cpu_cycles
            },
            _ => panic!("Unsupported shift opcode: {:#02X}", opcode),
        }
    }

    // preform a rotate-logical through carry with the given register value. bit 7 becomes the new carry bit.
    fn op_rlc(&mut self, operand: u8, is_cb_prefixed: bool) -> u8 {
        rog::debugln!("[{:#04X}] RLC {:#02X}", self.pc - if is_cb_prefixed {2} else {1}, operand);
        let carry_bit = operand >> 7;
        self.set_flag(Flag::C, carry_bit);
        let new_val = operand << 1 | carry_bit;
        self.set_flag(Flag::Z,
            match is_cb_prefixed {
                true => match new_val { 0 => 1, _ => 0, },
                false => 0,
            });
        rog::debugln!("    new_val: {:#02X}", new_val);
        new_val
    }

    // Rotate register right.
    // [0] -> [7 -> 0] -> C
    fn op_rrc(&mut self, operand: u8, is_cb_prefixed: bool) -> u8 {
        rog::debugln!("[{:#04X}] RRC {:#02X}", self.pc - if is_cb_prefixed {2} else {1}, operand);
        let carry_bit = operand & 0x01;
        let new_val = (carry_bit << 7) | (operand >> 1);
        self.set_flag(Flag::C, carry_bit);
        self.set_flag(Flag::Z,
            match is_cb_prefixed {
                true => match new_val { 0 => 1, _ => 0, },
                false => 0,
            });
        rog::debugln!("    new_val: {:#02X}", new_val);
        new_val
    }

    // C <- [7 <- 0] <- C
    fn op_rl(&mut self, operand: u8, is_cb_prefixed: bool) -> u8 {
        rog::debugln!("[{:#04X}] RL {:#02X}", self.pc - if is_cb_prefixed {2} else {1}, operand);
        let c = if self.flags & Flag::C as u8 != 0 { 0x1 } else { 0x0 };
        let new_val = (operand << 1) | c;
        self.set_flag(Flag::C, operand >> 7);
        self.set_flag(Flag::Z,
            match is_cb_prefixed {
                true => match new_val { 0 => 1, _ => 0, },
                false => 0,
            });
        rog::debugln!("    new_val: {:#02X}", new_val);
        new_val
    }

    // rotate register through carry
    // C -> [7 -> 0] -> C
    fn op_rr(&mut self, operand: u8, is_cb_prefixed: bool) -> u8 {
        rog::debugln!("[{:#04X}] RR {:#02X}", self.pc - if is_cb_prefixed {2} else {1}, operand);
        let c = if self.flags & Flag::C as u8 != 0 { 0x1 } else { 0x0 };
        let new_val = (c << 7) | (operand >> 1) ;
        self.set_flag(Flag::C, operand & 0x01);
        self.set_flag(Flag::Z,
            match is_cb_prefixed {
                true => match new_val { 0 => 1, _ => 0, },
                false => 0,
            });
        rog::debugln!("    new_val: {:#02X}", new_val);
        new_val
    }

    // shift left arithmetically
    fn op_sla(&mut self, operand: u8) -> u8 {
        let result = operand << 1;
        self.set_flag(Flag::Z, if result == 0 { 1 } else { 0 });
        self.set_flag(Flag::N, 0);
        self.set_flag(Flag::H, 0);
        self.set_flag(Flag::C, operand >> 7);
        result
    }

    // shift right arithmetically (preserve bit 7)
    // [7] -> [7-> 0] -> C
    fn op_sra(&mut self, operand: u8) -> u8 {
        let result = (operand & 0x80) | operand >> 1;
        self.set_flag(Flag::Z, if result == 0 { 1 } else { 0 });
        self.set_flag(Flag::N, 0);
        self.set_flag(Flag::H, 0);
        self.set_flag(Flag::C, operand & 0x01);
        result
    }

    // shfit right logically (bit 7 becomes 0)
    fn op_srl(&mut self, operand: u8) -> u8 {
        let result = operand >> 1;
        self.set_flag(Flag::Z, if result == 0 { 1 } else { 0 });
        self.set_flag(Flag::N, 0);
        self.set_flag(Flag::H, 0);
        self.set_flag(Flag::C, operand & 0x01);
        result
    }

    // swap the upper 4 bits with the lower 4
    fn op_swap(&mut self, operand: u8) -> u8 {
        let result = (operand & 0x0F << 4) | (operand >> 4);
        self.set_flag(Flag::Z, if result == 0 { 1 } else { 0 });
        self.set_flag(Flag::N, 0);
        self.set_flag(Flag::H, 0);
        self.set_flag(Flag::C, 0);
        result
    }

    // test bit 'index' in operand - set Z if bit not set. 
    fn op_bit(&mut self, operand: u8, index: u8) -> u8 {
        self.set_flag(Flag::Z, if (operand & (1 << index)) == 0 { 1 } else { 0 });
        self.set_flag(Flag::N, 0);
        self.set_flag(Flag::H, 1);
        operand
    }

    // clear bit
    fn op_res(&mut self, operand: u8, index: u8) -> u8 {
        let result = operand & !(1 << index);
        result
    }

    // set bit
    fn op_set(&mut self, operand: u8, index: u8) -> u8 {
        let result = operand | 1 << index;
        result
    }



}
