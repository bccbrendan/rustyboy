use super::memory::Memory;
pub struct Apu {

}

impl Apu {
    pub fn init() -> Self {
        Self {

        }
    }
}

impl Memory for Apu {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
           _ => panic!("unimplemented address read on Apu {:#04x}", addr)
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            _ => panic!("unimplemented address write on Apu {:#04x}, value: {:#02x}", addr, data)
        }
    }
}
 