use std::{rc::Rc, cell::RefCell};
use super::interrupts::Interrupts;

pub struct Joypad {
    pub interrupts: Rc<RefCell<Interrupts>>,
}