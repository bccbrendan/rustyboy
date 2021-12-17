use std::{rc::Rc, cell::RefCell};
use super::interrupts::Interrupts;
use super::memory::Memory;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;
pub const DOTS_PER_HLINE: usize = 456;
pub const SCANLINES: usize = 154;
pub const DOTS_PER_VBLANK: usize = 10 * DOTS_PER_HLINE;
pub const DOTS_PER_FRAME: usize = 70224;
pub const DOTS_BEFORE_VBLANK: usize = DOTS_PER_FRAME - DOTS_PER_VBLANK;

enum Mode {
    HorizontalBlank,
    VerticalBlank,
    OamScan,
    DrawingPixels,
}

fn will_enter_vblank(current_dot: usize, cycles_to_run: usize) -> bool {
    return current_dot < DOTS_BEFORE_VBLANK && ((current_dot + cycles_to_run) >= DOTS_BEFORE_VBLANK);
}


pub struct Gpu {
    pub interrupts: Rc<RefCell<Interrupts>>,
    // https://gbdev.io/pandocs/Scrolling.html
    current_dot: usize,
    mode: Mode,
    lcd_control: u8,
    lcd_status: u8,
    scroll_y: u8,
    scroll_x: u8,
    lcd_y_coordinate: u8,
    ly_compare: u8,
    obj_palette_0_data: u8,
    obj_palette_1_data: u8,
    window_pos_y: u8,
    window_pos_x: u8,
}

impl Gpu {
    pub fn init(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        Self {
            interrupts: interrupts,
            current_dot: 0,
            mode: Mode::OamScan,
            lcd_control: 0x00, // TODO https://gbdev.io/pandocs/LCDC.html
            lcd_status: 0x00, // TODO https://gbdev.io/pandocs/STAT.html#ff41---stat-lcd-status-rw
            scroll_y: 0x00,
            scroll_x: 0x00,
            lcd_y_coordinate: 0x00,
            ly_compare: 0x00,
            obj_palette_0_data: 0xFF,
            obj_palette_1_data: 0xFF,
            window_pos_y: 0,
            window_pos_x: 0,
        }
    }

    pub fn run_cycles(&mut self, cpu_clock_cycles: u32) {
        // mainly used when to determine vblank periods
        // TODO - dots take twice as many cycles if the cpu is running double speed
        if will_enter_vblank(self.current_dot, cpu_clock_cycles as usize) {
            // TODO interrupts.vblank = true; or something
        }
        self.current_dot = (self.current_dot + cpu_clock_cycles as usize) % DOTS_PER_FRAME;
        self.lcd_y_coordinate = (self.current_dot / DOTS_PER_HLINE) as u8;
        self.mode = if self.current_dot >= DOTS_BEFORE_VBLANK {
            Mode::VerticalBlank
        } else {
            // TODO - some actions lengthen mode 3 (drawing pixels) https://gbdev.io/pandocs/pixel_fifo.html
            match self.current_dot % DOTS_PER_HLINE {
                0 ..= 79 => Mode::OamScan,
                80 ..= 251 => Mode::DrawingPixels,
                _ => Mode::HorizontalBlank,
            }
        }
        // 4_194_304 cpu_clock_cycles / second
    }
 

    pub fn get_updated_image(&mut self) -> std::option::Option<[[u8; WIDTH]; HEIGHT]> {
        None
        // TODO
    }

    pub fn is_in_vblank(&self) -> bool {
        return 144 <= self.lcd_y_coordinate && self.lcd_y_coordinate <= 153;
    }
}

impl Memory for Gpu {
    // TODO - some memories are inaccessible in certian modes: https://gbdev.io/pandocs/pixel_fifo.html#pixel-fifo
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.lcd_control,
            0xFF41 => self.lcd_status,
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.lcd_y_coordinate,
            0xFF45 => self.ly_compare,
            0xFF48 => self.obj_palette_0_data,
            0xFF49 => self.obj_palette_1_data,
            0xFF4A => self.window_pos_y,
            0xFF4B => self.window_pos_x,
            _ => panic!("unimplemented address read on Gpu {:#04x}", addr)
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0xFF40 => self.lcd_control = data,
            0xFF41 => self.lcd_status = (self.lcd_status & 0x7) | data & 0xF8,
            0xFF42 => self.scroll_y = data,
            0xFF43 => self.scroll_x = data,
            0xFF44 => self.lcd_y_coordinate = 0x00,
            0xFF45 => self.ly_compare = data,
            0xFF48 => self.obj_palette_0_data = data,
            0xFF49 => self.obj_palette_1_data = data,
            0xFF4A => self.window_pos_y = data,
            0xFF4B => self.window_pos_x = data,
            _ => panic!("unimplemented address write on Gpu {:#04x}", addr)

        }
    }
}
 