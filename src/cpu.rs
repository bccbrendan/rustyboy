use std::rc::Rc;
use std::cell::RefCell;
use super::memory::Memory;

pub struct Cpu {
    pub mmu: Rc<RefCell<dyn Memory>>,
}

impl Cpu {
    pub fn init(mmu: Rc<RefCell<dyn Memory>>) -> Cpu {
        Self {mmu}
    }
}

pub struct ThrottledCpu {
    pub cpu: Cpu,
}

impl ThrottledCpu {
    pub fn init(mmu: Rc<RefCell<dyn Memory>>) -> ThrottledCpu {
        Self {cpu: Cpu::init(mmu.clone())}
    }
}