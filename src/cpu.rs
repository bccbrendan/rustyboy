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
            0x00 => {4}, // NOP
            // Jump instructions
            0xC3 => { // jp
                let target = self.fetch16();
                self.pc = target;
                16
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
}