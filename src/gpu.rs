use std::{rc::Rc, cell::RefCell};
use super::interrupts::Interrupts;
use super::memory::Memory;

pub struct Gpu {
    pub interrupts: Rc<RefCell<Interrupts>>,
    obj_palette_0_data: u8,
    obj_palette_1_data: u8,
    window_pos_y: u8,
    window_pos_x: u8,
}

impl Gpu {
    pub fn init(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        Self {
            interrupts: interrupts,
            obj_palette_0_data: 0xFF,
            obj_palette_1_data: 0xFF,
             window_pos_y: 0,
            window_pos_x: 0,
        }
    }
}

impl Memory for Gpu {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0xFF48 => self.obj_palette_0_data,
            0xFF49 => self.obj_palette_1_data,
            0xFF4A => self.window_pos_y,
            0xFF4B => self.window_pos_x,
            _ => panic!("unimplemented address read on Gpu {:#04x}", addr)
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0xFF48 => self.obj_palette_0_data = data,
            0xFF49 => self.obj_palette_1_data = data,
            0xFF4A => self.window_pos_y = data,
            0xFF4B => self.window_pos_x = data,
            _ => panic!("unimplemented address write on Gpu {:#04x}", addr)

        }
    }
}
 