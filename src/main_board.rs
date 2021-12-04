use std::time::{Instant, Duration};
use std::{cell::RefCell, rc::Rc};
use super::cpu::Cpu;
use super::memory_management_unit::MemoryManagementUnit;

pub const VSYNC_FREQ: f64 = 59.73;
pub const CPU_FREQUENCY: u32 = 4_194_304;
pub const CPU_CLOCKS_PER_FRAME: u32 = (CPU_FREQUENCY as f64 / VSYNC_FREQ) as u32;

pub struct MainBoard {
    pub cpu: Cpu,
    pub mmu: Rc<RefCell<MemoryManagementUnit>>,
}

impl MainBoard {
    pub fn init(filepath: &str) -> std::io::Result<MainBoard> {
        let mmu = Rc::new(RefCell::new(MemoryManagementUnit::init(filepath)));
        let cpu = Cpu::init(mmu.clone());
        Ok(MainBoard {
            cpu,
            mmu,
        })
    }

    pub fn emulate_frame(&mut self) {
        let mut emulated_cycles = 0;
        let time_before = Instant::now();
        const target_time: Duration = Duration::from_millis((1000.0_f64 / VSYNC_FREQ) as u64);
        while emulated_cycles < CPU_CLOCKS_PER_FRAME {
            let cycles = self.cpu.emulate_operation();
            self.mmu.borrow_mut().run_cycles(cycles);
            emulated_cycles += cycles;
        }
        let sleep_millis = target_time.checked_sub(Instant::now() - time_before);
        match sleep_millis {
            None => {}, // running below target fps
            Some(sleep_millis) => { 
                ::std::thread::sleep(sleep_millis);
            }
        }
    }
}