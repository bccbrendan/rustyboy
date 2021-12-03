use super::apu::Apu;
use super::cartridge::Cartridge;
use super::gpu::Gpu;
use super::interrupts::Interrupts;
use super::joypad::Joypad;
use super::serial_cable::SerialCable;
use super::timer::Timer;

pub struct MemoryManagementUnit {
    interrupts: Interrupts,
    pub cartridge: Cartridge,
    pub apu: Apu,
    pub gpu: Gpu,
    pub joypad: Joypad,
    pub serial_cable: SerialCable,
    pub timer: Timer,
    // hdma,
    // hram,
    // wram,
}