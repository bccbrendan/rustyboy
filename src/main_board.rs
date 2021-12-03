use std::{cell::RefCell, rc::Rc};
use super::cpu::ThrottledCpu;
use super::memory_management_unit::MemoryManagementUnit;

pub struct MainBoard {
    pub cpu: ThrottledCpu,
    pub mmu: Rc<RefCell<MemoryManagementUnit>>,
}

impl MainBoard {
    pub fn init(filepath: &str) -> std::io::Result<MainBoard> {
        let mmu = Rc::new(RefCell::new(MemoryManagementUnit::init(filepath)));
        let cpu = ThrottledCpu::init(mmu.clone());
        Ok(MainBoard {
            cpu,
            mmu,
        })
    }
}