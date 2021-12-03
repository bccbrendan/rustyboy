pub trait Memory {
    fn read8(&self, _addr: u16) -> u8;
    fn write8(&mut self, _addr: u16, _data: u8);
    fn read16(&self, addr: u16) -> u16 {
        u16::from(self.read8(addr)) | (u16::from(self.read8(addr + 1)) << 8)
    }
    fn write16(&mut self, addr: u16, data: u16) {
        self.write8(addr, (data & 0x00FF) as u8);
        self.write8(addr + 1, (data >> 8) as u8);
    }
}