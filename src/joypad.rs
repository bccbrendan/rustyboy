use std::{rc::Rc, cell::RefCell};
use super::interrupts::Interrupts;
use super::memory::Memory;

pub struct Joypad {
    pub interrupts: Rc<RefCell<Interrupts>>,
}

impl Joypad {
    pub fn init(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        Self {
            interrupts: interrupts,
        }
    }
}

impl Memory for Joypad {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
           _ => panic!("unimplemented address read on Joypad {:#04x}", addr)
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            _ => panic!("unimplemented address write on Joypad {:#04x}", addr)
        }
    }
}
 