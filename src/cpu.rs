use std::rc::Rc;
use std::cell::RefCell;
use super::memory::Memory;

const OP_MNEMONICS: [&str; 256] = [
    "NOP", "LD BC,d16", "LD (BC),A", "INC BC", "INC B", "DEC B", "LD B,d8", "RLCA", "LD (a16),SP", "ADD HL,BC", "LD A,(BC)", "DEC BC", "INC C", "DEC C", "LD C,d8", "RRCA",
    "STOP 0", "LD DE,d16", "LD (DE),A", "INC DE", "INC D", "DEC D", "LD D,d8", "RLA", "JR r8", "ADD HL,DE", "LD A,(DE)", "DEC DE", "INC E", "DEC E", "LD E,d8", "RRA",
    "JR NZ,r8", "LD HL,d16", "LD (HL+),A", "INC HL", "INC H", "DEC H", "LD H,d8", "DAA", "JR Z,r8", "ADD HL,HL", "LD A,(HL+)", "DEC HL", "INC L", "DEC L", "LD L,d8", "CPL",
    "JR NC,r8", "LD SP,d16", "LD (HL-),A", "INC SP", "INC (HL)", "DEC (HL)", "LD (HL),d8", "SCF", "JR C,r8", "ADD HL,SP", "LD A,(HL-)", "DEC SP", "INC A", "DEC A", "LD A,d8", "CCF",
    "LD B,B", "LD B,C", "LD B,D", "LD B,E", "LD B,H", "LD B,L", "LD B,(HL)", "LD B,A", "LD C,B", "LD C,C", "LD C,D", "LD C,E", "LD C,H", "LD C,L", "LD C,(HL)", "LD C,A",
    "LD D,B", "LD D,C", "LD D,D", "LD D,E", "LD D,H", "LD D,L", "LD D,(HL)", "LD D,A", "LD E,B", "LD E,C", "LD E,D", "LD E,E", "LD E,H", "LD E,L", "LD E,(HL)", "LD E,A",
    "LD H,B", "LD H,C", "LD H,D", "LD H,E", "LD H,H", "LD H,L", "LD H,(HL)", "LD H,A", "LD L,B", "LD L,C", "LD L,D", "LD L,E", "LD L,H", "LD L,L", "LD L,(HL)", "LD L,A",
    "LD (HL),B", "LD (HL),C", "LD (HL),D", "LD (HL),E", "LD (HL),H", "LD (HL),L", "HALT", "LD (HL),A", "LD A,B", "LD A,C", "LD A,D", "LD A,E", "LD A,H", "LD A,L", "LD A,(HL)", "LD A,A",
    "ADD A,B", "ADD A,C", "ADD A,D", "ADD A,E", "ADD A,H", "ADD A,L", "ADD A,(HL)", "ADD A,A", "ADC A,B", "ADC A,C", "ADC A,D", "ADC A,E", "ADC A,H", "ADC A,L", "ADC A,(HL)", "ADC A,A",
    "SUB B", "SUB C", "SUB D", "SUB E", "SUB H", "SUB L", "SUB (HL)", "SUB A", "SBC A,B", "SBC A,C", "SBC A,D", "SBC A,E", "SBC A,H", "SBC A,L", "SBC A,(HL)", "SBC A,A",
    "AND B", "AND C", "AND D", "AND E", "AND H", "AND L", "AND (HL)", "AND A", "XOR B", "XOR C", "XOR D", "XOR E", "XOR H", "XOR L", "XOR (HL)", "XOR A",
    "OR B", "OR C", "OR D", "OR E", "OR H", "OR L", "OR (HL)", "OR A", "CP B", "CP C", "CP D", "CP E", "CP H", "CP L", "CP (HL)", "CP A",
    "RET NZ", "POP BC", "JP NZ,a16", "JP a16", "CALL NZ,a16", "PUSH BC", "ADD A,d8", "RST 00H", "RET Z", "RET", "JP Z,a16", "PREFIX CB", "CALL Z,a16", "CALL a16", "ADC A,d8", "RST 08H",
    "RET NC", "POP DE", "JP NC,a16", "UNKNOWN", "CALL NC,a16", "PUSH DE", "SUB d8", "RST 10H", "RET C", "RETI", "JP C,a16", " UNKNOWN", "CALL C,a16", "UNKNOWN", "SBC A,d8", "RST 18H",
    "LDH (a8),A", "POP HL", "LD (C),A", "UNKNOWN", "UNKNOWN", "PUSH HL", "AND d8", "RST 20H", "ADD SP,r8", "JP (HL)", "LD (a16),A", "UNKNOWN", "UNKNOWN", "UNKNOWN", "XOR d8", "RST 28H",
    "LDH A,(a8)", "POP AF", "LD A,(C)", "DI", "UNKNOWN", "PUSH AF", "OR d8", "RST 30H", "LD HL,SP+r8", "LD SP,HL", "LD A,(a16)", "EI", "UNKNOWN", "UNKNOWN", "CP d8", "RST 38H",
];

const OP_CB_MNEMONICS: [&str; 256] = [
    "RLC B", "RLC C", "RLC D", "RLC E", "RLC H", "RLC L", "RLC (HL)", "RLC A", "RRC B", "RRC C", "RRC D", "RRC E", "RRC H", "RRC L", "RRC (HL)", "RRC A",
    "RL B", "RL C", "RL D", "RL E", "RL H", "RL L", "RL (HL)", "RL A", "RR B", "RR C", "RR D", "RR E", "RR H", "RR L", "RR (HL)", "RR A",
    "SLA B", "SLA C", "SLA D", "SLA E", "SLA H", "SLA L", "SLA (HL)", "SLA A", "SRA B", "SRA C", "SRA D", "SRA E", "SRA H", "SRA L", "SRA (HL)", "SRA A",
    "SWAP B", "SWAP C", "SWAP D", "SWAP E", "SWAP H", "SWAP L", "SWAP (HL)", "SWAP A", "SRL B", "SRL C", "SRL D", "SRL E", "SRL H", "SRL L", "SRL (HL)", "SRL A",
    "BIT 0,B", "BIT 0,C", "BIT 0,D", "BIT 0,E", "BIT 0,H", "BIT 0,L", "BIT 0,(HL)", "BIT 0,A", "BIT 1,B", "BIT 1,C", "BIT 1,D", "BIT 1,E", "BIT 1,H", "BIT 1,L", "BIT 1,(HL)", "BIT 1,A",
    "BIT 2,B", "BIT 2,C", "BIT 2,D", "BIT 2,E", "BIT 2,H", "BIT 2,L", "BIT 2,(HL)", "BIT 2,A", "BIT 3,B", "BIT 3,C", "BIT 3,D", "BIT 3,E", "BIT 3,H", "BIT 3,L", "BIT 3,(HL)", "BIT 3,A",
    "BIT 4,B", "BIT 4,C", "BIT 4,D", "BIT 4,E", "BIT 4,H", "BIT 4,L", "BIT 4,(HL)", "BIT 4,A", "BIT 5,B", "BIT 5,C", "BIT 5,D", "BIT 5,E", "BIT 5,H", "BIT 5,L", "BIT 5,(HL)", "BIT 5,A",
    "BIT 6,B", "BIT 6,C", "BIT 6,D", "BIT 6,E", "BIT 6,H", "BIT 6,L", "BIT 6,(HL)", "BIT 6,A", "BIT 7,B", "BIT 7,C", "BIT 7,D", "BIT 7,E", "BIT 7,H", "BIT 7,L", "BIT 7,(HL)", "BIT 7,A",
    "RES 0,B", "RES 0,C", "RES 0,D", "RES 0,E", "RES 0,H", "RES 0,L", "RES 0,(HL)", "RES 0,A", "RES 1,B", "RES 1,C", "RES 1,D", "RES 1,E", "RES 1,H", "RES 1,L", "RES 1,(HL)", "RES 1,A",
    "RES 2,B", "RES 2,C", "RES 2,D", "RES 2,E", "RES 2,H", "RES 2,L", "RES 2,(HL)", "RES 2,A", "RES 3,B", "RES 3,C", "RES 3,D", "RES 3,E", "RES 3,H", "RES 3,L", "RES 3,(HL)", "RES 3,A",
    "RES 4,B", "RES 4,C", "RES 4,D", "RES 4,E", "RES 4,H", "RES 4,L", "RES 4,(HL)", "RES 4,A", "RES 5,B", "RES 5,C", "RES 5,D", "RES 5,E", "RES 5,H", "RES 5,L", "RES 5,(HL)", "RES 5,A",
    "RES 6,B", "RES 6,C", "RES 6,D", "RES 6,E", "RES 6,H", "RES 6,L", "RES 6,(HL)", "RES 6,A", "RES 7,B", "RES 7,C", "RES 7,D", "RES 7,E", "RES 7,H", "RES 7,L", "RES 7,(HL)", "RES 7,A",
    "SET 0,B", "SET 0,C", "SET 0,D", "SET 0,E", "SET 0,H", "SET 0,L", "SET 0,(HL)", "SET 0,A", "SET 1,B", "SET 1,C", "SET 1,D", "SET 1,E", "SET 1,H", "SET 1,L", "SET 1,(HL)", "SET 1,A",
    "SET 2,B", "SET 2,C", "SET 2,D", "SET 2,E", "SET 2,H", "SET 2,L", "SET 2,(HL)", "SET 2,A", "SET 3,B", "SET 3,C", "SET 3,D", "SET 3,E", "SET 3,H", "SET 3,L", "SET 3,(HL)", "SET 3,A",
    "SET 4,B", "SET 4,C", "SET 4,D", "SET 4,E", "SET 4,H", "SET 4,L", "SET 4,(HL)", "SET 4,A", "SET 5,B", "SET 5,C", "SET 5,D", "SET 5,E", "SET 5,H", "SET 5,L", "SET 5,(HL)", "SET 5,A",
    "SET 6,B", "SET 6,C", "SET 6,D", "SET 6,E", "SET 6,H", "SET 6,L", "SET 6,(HL)", "SET 6,A", "SET 7,B", "SET 7,C", "SET 7,D", "SET 7,E", "SET 7,H", "SET 7,L", "SET 7,(HL)", "SET 7,A",
];

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
        if opcode != 0xCB {
            rog::debugln!("[{:#04X}] {:#04X} | {}", self.pc.wrapping_sub(1), opcode, OP_MNEMONICS[opcode as usize]);
        }
        // https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
        let cycles = match opcode {
            // CPU Control Instructions
            0x00 => 4, // NOP
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
                self.pc = target;
                16
            },
            0xCA | 0xD2 | 0xDA | 0xE9 | // jp 
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 => self.op_jr(opcode),
            0xC4 | 0xCC => panic!("unimplemented opcode: {:#02x}", opcode),
            0xCD => { // call
                let target = self.fetch16();
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
                self.mmu.borrow_mut().write8(0xFF00 + n as u16, self.a);
                12
            }
            0xF0 => {
                let n = self.fetch();
                self.a = self.mmu.borrow().read8(0xFF00 + n as u16);
                12
            }
            _ => panic!("Unsupported LD opcode: {:#02X}", opcode),
        }
    }

    fn emulate_16bit_load_operation(&mut self, opcode: u8) -> u32 {
        let cpu_cycles = match opcode {
            0x01 => {
                let d16 = self.fetch16();
                self.b  = (d16 >> 8) as u8;
                self.c  = (d16 & 0xFF) as u8;
                12
            },
            0x11 => {
                let d16 = self.fetch16();
                self.d  = (d16 >> 8) as u8;
                self.e  = (d16 & 0xFF) as u8;
                12
            },
            0x21 => {
                let d16 = self.fetch16();
                self.h  = (d16 >> 8) as u8;
                self.l  = (d16 & 0xFF) as u8;
                12
            },
            0x31 => {
                self.sp = self.fetch16();
                12
            },
            0x08 => {
                let a16 = self.fetch16();
                self.mmu.borrow_mut().write16(a16, self.sp);
                20
            },
            0xC1 => {
                self.c = self.pop();
                self.b = self.pop();
                12
            },
            0xD1 => {
                self.e = self.pop();
                self.d = self.pop();
                12
            },
            0xE1 => {
                self.l = self.pop();
                self.h = self.pop();
                12
            },
            0xF1 => {
                self.flags = self.pop();
                self.a = self.pop();
                12
            },
            0xC5 => {
                self.push(self.b);
                self.push(self.c);
                16
            },
            0xD5 => {
                self.push(self.d);
                self.push(self.e);
                16
            },
            0xE5 => {
                self.push(self.h);
                self.push(self.l);
                16
            },
            0xF5 => {
                self.push(self.a);
                self.push(self.flags);
                16
            },
            0xF8 => {
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
        let operand = match opcode {
            0x04 | 0x05 => self.b,
            0x0C | 0x0D => self.c,
            0x14 | 0x15 => self.d,
            0x1C | 0x1D => self.e,
            0x24 | 0x25 => self.h,
            0x2C | 0x2D => self.l,
            0x34 | 0x35 => self.mmu.borrow().read8((self.h as u16) << 8 | self.l as u16),
            0x3C | 0x3D => self.a,
            _ => panic!("impossible opcode: {:#02X}!", opcode),
        };
        let result = if opcode & 0x01 == 0x00 {
            // INC
            self.set_flag(Flag::H, if operand & 0x07 == 0x07 { 1 } else { 0 });
            self.set_flag(Flag::N, 0);
            operand.wrapping_add(1)
        } else {
            // "DEC "
            self.set_flag(Flag::H, if operand & 0x07 == 0x00 { 1 } else { 0 });
            self.set_flag(Flag::N, 1);
            operand.wrapping_sub(1)
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
    }


    // compare the operand vs A, but don't store a result
    fn op_cp(&mut self, opcode: u8)  {
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
                rog::debugln!("[{:#06X}] {:#04X}{:02X} | {}", self.pc.wrapping_sub(2), opcode, cb_opcode, OP_CB_MNEMONICS[cb_opcode as usize]);
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
        let carry_bit = operand >> 7;
        self.set_flag(Flag::C, carry_bit);
        let new_val = operand << 1 | carry_bit;
        self.set_flag(Flag::Z,
            match is_cb_prefixed {
                true => match new_val { 0 => 1, _ => 0, },
                false => 0,
            });
        new_val
    }

    // Rotate register right.
    // [0] -> [7 -> 0] -> C
    fn op_rrc(&mut self, operand: u8, is_cb_prefixed: bool) -> u8 {
        let carry_bit = operand & 0x01;
        let new_val = (carry_bit << 7) | (operand >> 1);
        self.set_flag(Flag::C, carry_bit);
        self.set_flag(Flag::Z,
            match is_cb_prefixed {
                true => match new_val { 0 => 1, _ => 0, },
                false => 0,
            });
        new_val
    }

    // C <- [7 <- 0] <- C
    fn op_rl(&mut self, operand: u8, is_cb_prefixed: bool) -> u8 {
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
