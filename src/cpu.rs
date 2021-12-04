use std::rc::Rc;
use std::cell::RefCell;
use super::memory::Memory;

// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
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

    pub fn emulate_operation(&mut self) -> u32{
        let opcode = self.fetch();
        let cycles = match opcode {
            // CPU Control Instructions
            0x00 => {rog::debugln!("[{:#04X}] NOP", self.pc - 1); 4},
            // Jump instructions
            0xC2 | 0xC3 | 0xCA | 0xD2 | 0xDA | 0xE9 | // jp 
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 |  // jr
            0xC4 | 0xCC | 0xCD | 0xD4 | 0xDC |  // call
            0xC0 | 0xC8 | 0xC9 | 0xD0 | 0xD8 | 0xD9  => { // ret+reti
                self.emulate_jump_operation(opcode)
            },
            // LD operations
            0x02 | 0x06 | 0x08 | 0x0A | 0x0E |
            0x12 | 0x16 | 0x18 | 0x1A | 0x1E |
            0x22 | 0x26 | 0x28 | 0x2A | 0x2E |
            0x32 | 0x36 | 0x38 | 0x3A | 0x3E |
            0x40 ..= 0x7F |
            0xE0 | 0xE2 | 0xEA |
            0xF0 | 0xF2 | 0xF8 | 0xF9 | 0xFA => { 
                self.emulate_load_operation(opcode)
            },
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
            0xE0 | 0xE2 | 0xEA |
            0xF2 | 0xF8 | 0xF9 | 0xFA => panic!("Not yet implemented: {:#02X}", opcode),
            0xF0 => {
                let n = self.fetch();
                rog::debugln!("[{:#04X}] LD A, 0xFF00+{:#02X}", self.pc - 2, n);
                self.a = self.mmu.borrow().read8(0xFF00 + n as u16);
                12
            }
            _ => panic!("Unsupported LD opcode: {:#02X}", opcode),
        }
    }
}
