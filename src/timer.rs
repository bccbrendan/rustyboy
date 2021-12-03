use std::{rc::Rc, cell::RefCell};
use super::interrupts::Interrupts;
use super::memory::Memory;

// https://gbdev.io/pandocs/Timer_and_Divider_Registers.html
pub struct Timer {
    interrupts: Rc<RefCell<Interrupts>>,
    div: u8,
}

impl Timer {
    pub fn init(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        Timer {
            interrupts: interrupts,
            div: 0
        }
    }
}

impl Memory for Timer {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.div,
           _ => panic!("unimplemented address read on Timer {:#04x}", addr)
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0xFF04 => self.div = data,
            _ => panic!("unimplemented address write on Timer {:#04x}", addr)
        }
    }
}
 