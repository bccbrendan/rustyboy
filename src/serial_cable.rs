use std::{rc::Rc, cell::RefCell};
use super::interrupts::Interrupts;

pub struct SerialCable {
    interrupts: Rc<RefCell<Interrupts>>,
}