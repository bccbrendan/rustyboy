// https://gbdev.io/pandocs/The_Cartridge_Header.html
use std::{fs::File, io::Read};
use super::memory::Memory;


pub trait Cartridge: Memory {
    fn get_type(&self) -> String;
    fn get_title(&self) -> String {
        let start = 0x134;
        let maybe_cgb_flag = self.read8(0x143);
        let length = if maybe_cgb_flag == 0x80 || maybe_cgb_flag == 0xC0 { 11 } else { 16 };
        let mut title_string = String::new();
        for i in start .. start + length {
            match self.read8(i) {
                0x00 => break,
                byte => title_string.push(byte as char),
            }
        }
        title_string
    }
}


pub fn init(filepath: &str) -> Box<dyn Cartridge> {
    let mut f = File::open(filepath).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let cartridge_type = buffer[0x0147];
    match cartridge_type {
        0x00 => Box::new(NoMbc::init(buffer)),
        0x01 ..= 0x03 => Box::new(Mbc1::init(buffer)),
        _ => panic!("Unknown ROM Cartridge type: {:#02X}", cartridge_type),
    }
}

/* NoMbc */
struct NoMbc {
    rom: std::vec::Vec<u8>,
}

impl NoMbc {
    pub fn init(rom_bytes: std::vec::Vec<u8>) -> Self {
        NoMbc {rom: rom_bytes}
    }
}

impl Memory for NoMbc {
    fn read8(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
    
    fn write8(&mut self, _addr: u16, _data: u8) {
    }
}

impl Cartridge for NoMbc {
    fn get_type(&self) -> String {
        "NoMbc".to_string()
    }

}

enum BankMode {
    SimpleRomBanking,
    RamBankingOrAdvancedRomBanking,
}

/* Mbc1 - A memory bank controller - may have a battery, and may have ram
   https://gbdev.io/pandocs/MBC1.html */
struct Mbc1 {
    rom: std::vec::Vec<u8>,
    ram: std::vec::Vec<u8>,
    ram_enable: bool,
    rom_bank: u8,
    ram_bank: u8,
    banking_mode_select: BankMode,
}

impl Mbc1 {
    pub fn init(rom_bytes: std::vec::Vec<u8>) -> Self {
        let ram_size: usize = match rom_bytes[0x0149] {
            0x00 | 0x01 => 0,
            0x02 => 8192,
            0x03 => 8192 * 4,
            0x04 => 8192 * 16,
            0x05 => 8192 * 8,
            _ => panic!("Unrecognized ram_size code: {:#02X}", rom_bytes[0x0149]),
        };

        Mbc1 {
            rom: rom_bytes,
            ram: vec![0x0; ram_size],
            ram_enable: false,
            rom_bank: 0x1,
            ram_bank: 0x0,
            banking_mode_select: BankMode::SimpleRomBanking,
        }
    }
}

const ROM_BANK_SIZE: usize = 16_384; //16KiB
const RAM_BANK_SIZE: usize = 2048; //2KiB


impl Memory for Mbc1 {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            // ROM Bank X0
            0x0000 ..= 0x3FFF => self.rom[addr as usize],
            // Rom Bank 01-7F
            0x4000 ..= 0x7FFF => {
                assert!(self.rom_bank != 0);
                let physical_addr = self.rom_bank as usize * ROM_BANK_SIZE + addr as usize;
                self.rom[physical_addr]
            }
            // RAM Bank 00-03, if any
            0xA000 ..= 0xBFFF => {
                if self.ram_enable {
                    self.ram[addr as usize + self.ram_bank as usize * RAM_BANK_SIZE - 0xA000]
                } else {
                    0xFF
                }
            }
            _ => panic!("Unmapped memory in Mbc1: {:#04x}", addr),
        }
    }
    
    fn write8(&mut self, _addr: u16, _data: u8) {
    }
}

impl Cartridge for Mbc1 {
    fn get_type(&self) -> String {
        "Mbc1".to_string()
    }
}


/*  TODO:
    Mbc2,
    Mbc3,
    Mmm01,
    Mbc5,
    Mbc6,
    Mbc7,
*/