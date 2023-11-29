use super::memory::Memory;
use super::apu::Apu;
use super::cartridge;
use super::cartridge::Cartridge;
use super::gpu::Gpu;
use super::interrupts::Interrupts;
use super::joypad::Joypad;
use super::serial_cable::SerialCable;
use super::timer::Timer;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MemoryManagementUnit {
    pub cartridge: Box<dyn Cartridge>,
    pub apu: Apu,
    pub gpu: Gpu,
    pub joypad: Joypad,
    pub serial_cable: SerialCable,
    pub timer: Timer,
    // interrupts: Rc<RefCell<Interrupts>>,
    // hdma,
    pub work_ram_c000: [u8; 4096],  //wram
    pub work_ram_d000: [u8; 4096],  //wram
    pub hram: [u8; 128],// hram,
    interrupt_enable: u8,
}

impl MemoryManagementUnit {
    pub fn init(filepath: &str) -> Self {
        let cartridge = cartridge::init(filepath);
        let interrupts = Rc::new(RefCell::new(Interrupts::init()));
        let mmu = Self {
            cartridge: cartridge,
            apu: Apu::init(),
            gpu: Gpu::init(interrupts.clone()),
            joypad: Joypad::init(interrupts.clone()),
            serial_cable: SerialCable::init(interrupts.clone()),
            timer: Timer::init(/*interrupts.clone()*/),
            // interrupts: interrupts.clone(),
            work_ram_c000: [0x0; 4096],
            work_ram_d000: [0x0; 4096],
            hram: [0x0; 128],
            interrupt_enable: 0x00,
        };
        mmu
    }

    pub fn run_cycles(&mut self, cpu_clock_cycles: u32) {
        // TODO run cycles on components, let them drive interrupts to each other.
        // This is done in small pieces from the main_board, so no need to break up cpu_clock_cycles
        self.gpu.run_cycles(cpu_clock_cycles);
    }
}

impl Memory for MemoryManagementUnit {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ..= 0x7FFF => self.cartridge.read8(addr),
            0xC000 ..= 0xCFFF => self.work_ram_c000[(addr - 0xC000) as usize],
            0xD000 ..= 0xDFFF => self.work_ram_d000[(addr - 0xD000) as usize],
            // Mirror of C000-DDFF
            0xE000 ..= 0xEFFF => self.work_ram_c000[(addr - 0xE000) as usize],
            0xF000 ..= 0xFDFF => self.work_ram_d000[(addr - 0xF000) as usize],
            0xFF00 => self.joypad.read8(addr),
            0xFF01 | 0xFF02 => self.serial_cable.read8(addr),
            0xFF04 ..= 0xFF07 => self.timer.read8(addr),
            0xFF10 ..= 0xFF26 => self.apu.read8(addr),
            0xFF30 ..= 0xFF3F => panic!("Waveform RAM not implemented!"),
            0xFF40 ..= 0xFF4B => self.gpu.read8(addr),
            // $FF4F       VRAM Bank Select
            // $FF50       Set to non-zero to disable boot ROM
            // $FF51 $FF55 VRAM DMA
            // $FF68 $FF69 Palettes
            // $FF70       WRAM Bank Select
            0xFF80 ..= 0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable,
            _ => panic!("unimplemented address read on MemoryManagementUnit {:#04x}", addr)
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7fff => self.cartridge.write8(addr, data),
            0xC000 ..= 0xCFFF => { self.work_ram_c000[(addr - 0xC000) as usize] = data },
            0xD000 ..= 0xDFFF => { self.work_ram_d000[(addr - 0xD000) as usize] = data },
            // Mirror of C000-DDFF
            0xE000 ..= 0xEFFF => { self.work_ram_c000[(addr - 0xE000) as usize] = data },
            0xF000 ..= 0xFDFF => { self.work_ram_d000[(addr - 0xF000) as usize] = data },
            0xFF00 => self.joypad.write8(addr, data),
            0xFF01 | 0xFF02 => self.serial_cable.write8(addr, data),
            0xFF04 ..= 0xFF07 => self.timer.write8(addr, data),
            0xFF10 ..= 0xFF26 => self.apu.write8(addr, data),
            0xFF30 ..= 0xFF3F => panic!("Waveform RAM not implemented!"),
            0xFF40 ..= 0xFF4B => self.gpu.write8(addr, data),
            // $FF4F       VRAM Bank Select
            // $FF50       Set to non-zero to disable boot ROM
            // $FF51 $FF55 VRAM DMA
            // $FF68 $FF69 Palettes
            // $FF70       WRAM Bank Select
            0xFF80 ..= 0xFFFE => self.hram[(addr - 0xFF80) as usize] = data,
            0xFFFF => self.interrupt_enable = data,
             _ => panic!("unimplemented address write on MemoryManagementUnit {:#04x}", addr),
        }
    }
 }
