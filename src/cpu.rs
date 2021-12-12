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

    fn is_set(&self, f: Flag) -> bool {
        self.flags & f as u8 != 0
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
            0x00 => {rog::debugln!("[{:#06X}] NOP", self.pc - 1); 4},
            // Jump instructions
            0xC2 | 0xC3 | 0xCA | 0xD2 | 0xDA | 0xE9 | // jp 
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 |  // jr
            0xC4 | 0xCC | 0xCD | 0xD4 | 0xDC |  // call
            0xC0 | 0xC8 | 0xC9 | 0xD0 | 0xD8 | 0xD9  => self.emulate_jump_operation(opcode),
            // LD operations
            0x02 | 0x06 | 0x0A | 0x0E |
            0x12 | 0x16 | 0x1A | 0x1E |
            0x22 | 0x26 | 0x2A | 0x2E |
            0x32 | 0x36 | 0x3A | 0x3E |
            0x40 ..= 0x7F |
            0xE0 | 0xE2 | 0xEA |
            0xF0 | 0xF2 | 0xFA => self.emulate_8bit_load_operation(opcode),
            // 16-bit ld/store/move ops
            0x01 | 0x11 | 0x21 | 0x31 | 0x08 |
            0xC1 | 0xD1 | 0xE1 | 0xF1 | 
            0xC5 | 0xD5 | 0xE5 | 0xF5 | 
            0xF8 | 0xF9 => self.emulate_16bit_load_operation(opcode), 
            // 8-bit Arithmethic/Logic instructions
            0x04 | 0x05 | 0x0C | 0x0D | 0x14 | 0x15 | 0x1C | 0x1D |
            0x24 | 0x25 | 0x27 | 0x2C | 0x2D | 0x2F |
            0x34 | 0x35 | 0x37 | 0x3C | 0x3D | 0x3F |
            0x80 ..= 0x8F |
            0x90 ..= 0x9F |
            0xA0 ..= 0xAF |
            0xB0 ..= 0xBF |
            0xC6 | 0xD6 | 0xE6 | 0xF6 | 0xCE | 0xDE | 0xEE | 0xFE => self.emulate_8bit_arithmetic_or_logic(opcode),
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

    // based on https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html, this is true for many opcodes
    fn fetch_reg_operand(&self, opcode: u8) -> u8 {
        let operand = match opcode & 0x7 {
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
            _ => panic!("impossible"),
        };
        operand
    }

    // based on https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html, this is true for many opcodes
    fn store_result_in_register(&mut self, opcode: u8, result: u8) {
        match opcode & 0x7 {
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
            _ => panic!("impossible!"),
        };
    }

    fn push(&mut self, data: u8) {
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.borrow_mut().write8(self.sp, data);
    }

    fn pop(&mut self) -> u8 {
        let data = self.mmu.borrow().read8(self.sp);
        self.sp = self.sp.wrapping_add(1);
        data
    }

    fn emulate_jump_operation(&mut self, opcode: u8) -> u32 {
        match opcode {
            0xC2 => panic!("unimplemented!"),
            0xC3 => { // jp
                let target = self.fetch16();
                rog::debugln!("[{:#04X}] JP {:#04X}", self.pc - 3, target);
                self.pc = target;
                16
            },
            0xCA | 0xD2 | 0xDA | 0xE9 | // jp 
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 => self.op_jr(opcode),
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

    // jump conditionally
    fn op_jr(&mut self, opcode: u8) -> u32 {
        let target = self.pc + u16::from(self.fetch());
        rog::debugln!("[{:#04X}] JR {:#04X}", self.pc - 2, target);
        let taken = opcode == 0x18 ||
            (opcode == 0x20 && !self.is_set(Flag::Z)) ||
            (opcode == 0x28 && self.is_set(Flag::Z)) ||
            (opcode == 0x30 && !self.is_set(Flag::C)) ||
            (opcode == 0x38 && self.is_set(Flag::C));
        if taken {
            self.pc = target;
            12
        } else {
            8
        }
    }


    fn emulate_8bit_load_operation(&mut self, opcode: u8) -> u32 {
        match opcode {
            0x02 | 0x06 | 0x08 | 0x0A | 0x0E |
            0x12 | 0x16 | 0x18 | 0x1A | 0x1E |
            0x22 | 0x26 | 0x28 | 0x2A | 0x2E |
            0x32 | 0x36 | 0x38 | 0x3A | 0x3E |
            0x40 ..= 0x7F |
            0xE2 | 0xEA |
            0xF2 | 0xF8 | 0xFA => panic!("LD Not yet implemented: {:#02X}", opcode),
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

    fn emulate_16bit_load_operation(&mut self, opcode: u8) -> u32 {
        let cpu_cycles = match opcode {
            0x01 => {
                rog::debugln!("[{:#04X}] LD BC, d16", self.pc - 1);
                let d16 = self.fetch16();
                self.b  = (d16 >> 8) as u8;
                self.c  = (d16 & 0xFF) as u8;
                12
            },
            0x11 => {
                rog::debugln!("[{:#04X}] LD DE, d16", self.pc - 1);
                let d16 = self.fetch16();
                self.d  = (d16 >> 8) as u8;
                self.e  = (d16 & 0xFF) as u8;
                12
            },
            0x21 => {
                rog::debugln!("[{:#04X}] LD HL, d16", self.pc - 1);
                let d16 = self.fetch16();
                self.h  = (d16 >> 8) as u8;
                self.l  = (d16 & 0xFF) as u8;
                12
            },
            0x31 => {
                rog::debugln!("[{:#04X}] LD SP, d16", self.pc - 1);
                self.sp = self.fetch16();
                12
            },
            0x08 => {
                rog::debugln!("[{:#04X}] LD (a16),sp", self.pc - 1);
                let a16 = self.fetch16();
                self.mmu.borrow_mut().write16(a16, self.sp);
                20
            },
            0xC1 => {
                rog::debugln!("[{:#04X}] POP BC", self.pc - 1);
                self.c = self.pop();
                self.b = self.pop();
                12
            },
            0xD1 => {
                rog::debugln!("[{:#04X}] POP DE", self.pc - 1);
                self.e = self.pop();
                self.d = self.pop();
                12
            },
            0xE1 => {
                rog::debugln!("[{:#04X}] POP HL", self.pc - 1);
                self.l = self.pop();
                self.h = self.pop();
                12
            },
            0xF1 => {
                rog::debugln!("[{:#04X}] POP AF", self.pc - 1);
                self.flags = self.pop();
                self.a = self.pop();
                12
            },
            0xC5 => {
                rog::debugln!("[{:#04}] PUSH BC", self.pc - 1);
                self.push(self.b);
                self.push(self.c);
                16
            },
            0xD5 => {
                rog::debugln!("[{:#04}] PUSH DE", self.pc - 1);
                self.push(self.d);
                self.push(self.e);
                16
            },
            0xE5 => {
                rog::debugln!("[{:#04}] PUSH HL", self.pc - 1);
                self.push(self.h);
                self.push(self.l);
                16
            },
            0xF5 => {
                rog::debugln!("[{:#04}] PUSH AF", self.pc - 1);
                self.push(self.a);
                self.push(self.flags);
                16
            },
            0xF8 => {
                rog::debugln!("[{:#04X}] LD HL,SP+r8", self.pc - 1);
                let addend = i16::from(self.fetch() as i8) as u16;
                // carry flag set if overflow from bit 7
                let c_flag = (self.sp & 0x00FF) + (addend & 0x00FF) > 0x00FF;
                // half-carry flag set if overflow from bit 3
                let h_flag = (self.sp & 0x000F) + (addend & 0x000F) > 0x000F;
                let sum = self.sp.wrapping_add(addend);
                self.h = (sum >> 8) as u8;
                self.l = (sum & 0xFF) as u8;
                self.set_flag(Flag::Z, 0);
                self.set_flag(Flag::N, 0);
                self.set_flag(Flag::H, if h_flag { 1 } else { 0 });
                self.set_flag(Flag::C, if c_flag { 1 } else { 0 });
                12
            },
            0xF9 => {
                rog::debugln!("[{:#04X}] LD SP,HL", self.pc - 1);
                let hl = (self.h as u16) << 8 | self.l as u16;
                self.sp = hl;
                8
            },
            _ => panic!("not implemented: {:#02X}", opcode),
        };
        cpu_cycles
    }

    fn emulate_8bit_arithmetic_or_logic(&mut self, opcode: u8) -> u32 {
        // 8-bit Arithmethic/Logic instructions
        match opcode {
            0x04 | 0x05 | 0x0C | 0x0D |
            0x14 | 0x15 | 0x1C | 0x1D |
            0x24 | 0x25 | 0x2C | 0x2D |
            0x34 | 0x35 | 0x3C | 0x3D => self.op_inc_or_dec(opcode),
            0x2F => {  // CPL
                rog::debugln!("[{:#04X}] CPL", self.pc - 1);
                self.a = !self.a;
                self.set_flag(Flag::N, 1);
                self.set_flag(Flag::H, 1);
            },
            0x27 | 0x37 | 0x3F |
            0x80 ..= 0x8F |
            0x90 ..= 0x9F |
            0xA0 ..= 0xAF => panic!("not implemented {:#02X}", opcode),
            0xB0 ..= 0xBF | 0xFE => self.op_cp(opcode),
            0xC6 | 0xD6 | 0xE6 | 0xF6 | 0xCE | 0xDE | 0xEE => panic!("not implemented!"),
            _ => panic!("impossible opcode for 8bit math/logic: {:#02X}", opcode),
        }
        let cpu_cycles = if opcode == 0x34 || opcode == 0x35 {
            12
        } else if opcode & 0x07 == 0x06 {
            8  // those that dereference HL
        } else {
            4
        };
        cpu_cycles
    }

    fn op_inc_or_dec(&mut self, opcode: u8) {
        let (operand, operand_name) = match opcode {
            0x04 | 0x05 => (self.b, "B"),
            0x0C | 0x0D => (self.c, "C"),
            0x14 | 0x15 => (self.d, "D"),
            0x1C | 0x1D => (self.e, "E"),
            0x24 | 0x25 => (self.h, "H"),
            0x2C | 0x2D => (self.l, "L"),
            0x34 | 0x35 => (self.mmu.borrow().read8((self.h as u16) << 8 | self.l as u16), "(HL)"),
            0x3C | 0x3D => (self.a, "A"),
            _ => panic!("impossible opcode: {:#02X}!", opcode),
        };
        let (result, operation_name) = if opcode & 0x01 == 0x00 {
            // INC
            self.set_flag(Flag::H, if operand & 0x07 == 0x07 { 1 } else { 0 });
            self.set_flag(Flag::N, 0);
            (operand.wrapping_add(1), "INC")
        } else {
            // "DEC "
            self.set_flag(Flag::H, if operand & 0x07 == 0x00 { 1 } else { 0 });
            self.set_flag(Flag::N, 1);
            (operand.wrapping_sub(1), "DEC")
        };
        self.set_flag(Flag::Z, if result == 0x00 { 1 } else { 0 });
        match opcode {
            0x04 | 0x05 => self.b = result,
            0x0C | 0x0D => self.c = result,
            0x14 | 0x15 => self.d = result,
            0x1C | 0x1D => self.e = result,
            0x24 | 0x25 => self.h = result,
            0x2C | 0x2D => self.l = result,
            0x34 | 0x35 => self.mmu.borrow_mut().write8((self.h as u16) << 8 | self.l as u16, result),
            0x3C | 0x3D => self.a = result,
            _ => panic!("impossible opcode: {:#02X}!", opcode),
        }
        rog::debugln!("[{:#04X}] {} {}", self.pc - 1, operation_name, operand_name);
    }


    // compare the operand vs A, but don't store a result
    fn op_cp(&mut self, opcode: u8)  {
        rog::debugln!("[{:#04X}] CP (opcode: {:#02X})", self.pc - 1, opcode);
        let operand = match opcode {
            0xB8 ..= 0xBF => self.fetch_reg_operand(opcode),
            0xFE => self.fetch(),
            _ => panic!("invalid opcode for op_cp: {:#02X}", opcode),
        };
        let result = self.a.wrapping_sub(operand);
        self.set_flag(Flag::C, if self.a < operand { 1 } else { 0 });
        self.set_flag(Flag::N, 1);
        self.set_flag(Flag::H, if (self.a & 0x0F) < (operand & 0x0F) { 1 } else { 0 });
        self.set_flag(Flag::Z, if result == 0 { 1 } else { 0 });
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
                let operand = self.fetch_reg_operand(cb_opcode);
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
                self.store_result_in_register(cb_opcode, result);
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
