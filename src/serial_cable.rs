use std::{rc::Rc, cell::RefCell};
use super::interrupts::Interrupts;
use super::memory::Memory;

pub struct SerialCable {
    interrupts: Rc<RefCell<Interrupts>>,
}

impl SerialCable {
    pub fn init(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        SerialCable { interrupts }
    }
}


impl Memory for SerialCable {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
           _ => panic!("unimplemented address read on SerialCable {:#04x}", addr)
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            _ => panic!("unimplemented address write on SerialCable {:#04x}", addr)
        }
    }
}
 