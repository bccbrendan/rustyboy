use std::{rc::Rc, cell::RefCell};
use super::interrupts::Interrupts;

pub struct Gpu {
    pub interrupts: Rc<RefCell<Interrupts>>,
}