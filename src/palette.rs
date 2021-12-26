pub enum PaletteDataColor {
    White,
    LightGray,
    DarkGray,
    Black,
}

pub struct PaletteData {
    raw: u8,
    pub index_0_color: PaletteDataColor,
    pub index_1_color: PaletteDataColor,
    pub index_2_color: PaletteDataColor,
    pub index_3_color: PaletteDataColor,
}

fn bits_to_palette_data_color(bits: u8) -> PaletteDataColor {
    match bits {
        0 => PaletteDataColor::White,
        1 => PaletteDataColor::LightGray,
        2 => PaletteDataColor::DarkGray,
        3 => PaletteDataColor::Black,
        _ => panic!("Invalid 2bit code for palette data color: {:#02X}", bits),
    }
}

impl PaletteData {
    pub fn init(control_register: u8) -> Self {
        let index_0_color = bits_to_palette_data_color(control_register & 0x3);
        let index_1_color = bits_to_palette_data_color((control_register >> 2) & 0x3);
        let index_2_color = bits_to_palette_data_color((control_register >> 4) & 0x3);
        let index_3_color = bits_to_palette_data_color((control_register >> 6) & 0x3);
        Self {
            raw: control_register,
            index_0_color: index_0_color,
            index_1_color: index_1_color,
            index_2_color: index_2_color,
            index_3_color: index_3_color,
        }
    }

    pub fn read(&self) -> u8 {
        self.raw
    }
}