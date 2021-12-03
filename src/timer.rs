use std::{rc::Rc, cell::RefCell};
use super::interrupts::Interrupts;

pub struct Timer {
    interrupts: Rc<RefCell<Interrupts>>,
}