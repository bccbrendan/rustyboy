use imgui::Ui;

use crate::cpu::OP_MNEMONICS;
use crate::cpu::OP_CB_MNEMONICS;
use crate::memory::Memory;

use super::main_board::MainBoard;
use super::execution_modes::ExecutionMode;

pub struct Gui {
    pub lcd_scale: u8,
    pub execution_mode: ExecutionMode,
    pub disassembly_start_address: u16,
    pub disassembly_end_address: u16,
    pub disassembly_lines_to_print: u16,
}

impl Default for Gui {
    fn default() -> Self {
        Gui {
            lcd_scale: 1,
            execution_mode: ExecutionMode::Stopped,
            disassembly_start_address: 0x100,
            disassembly_end_address: 0x100 + 16,
            disassembly_lines_to_print: 16,
        }
    }
}

impl Gui {

    pub fn show(&mut self, ui: &mut Ui, main_board: &mut MainBoard) -> ExecutionMode {
        self.execution_mode = match self.execution_mode {
            ExecutionMode::CpuOperation => ExecutionMode::Stopped,
            ExecutionMode::Frame => ExecutionMode::Stopped,
            _ => self.execution_mode,
        };
        ui.window("Rustyboy")
            .size([0.0, 0.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.child_window("Cartridge and Audio")
                    .size([200.0, 400.0])
                    .build(|| {
                        ui.child_window("Cartridge")
                            .size([0.0, 100.0])
                            .build(|| {
                                ui.text("title: 123456789AB");
                                ui.text("type: MCB1?");
                                ui.text("CCB: None");
                                ui.button("load ROM");
                        });
                    ui.separator();
                    ui.child_window("Controls")
                        .size([200.0, 100.0])
                        .build(|| {
                            if ui.button("go")
                            {
                                self.execution_mode = ExecutionMode::Running;
                            }
                            ui.same_line();
                            if ui.button("stop")
                            {
                                self.execution_mode = ExecutionMode::Stopped;
                            }
                            if ui.button("step")
                            {
                                self.execution_mode = ExecutionMode::CpuOperation;
                            }
                            if ui.button("step frame")
                            {
                                self.execution_mode = ExecutionMode::Frame;
                            }
                    });
                    ui.separator();
                    ui.child_window("APU")
                        .size([0.0, 0.0])
                        .build(|| {
                            ui.button("mute");
                            ui.same_line();
                            ui.button("unmute");
                            ui.text("Enabled: no");
                            ui.separator();
                            ui.text("wave ram:");
                            ui.separator();
                            ui.text("channel 1 (off): ");
                            ui.text("channel 2 (off): ");
                            ui.text("channel 3 (off): ");
                            ui.text("channel 4 (off): ");
                    });
                });
        ui.same_line();
        ui.child_window("LCD and memory")
            .size([500.00, 100.0])
            .build(|| {
                ui.child_window("LCD")
                    .build(|| {
                        ui.text("LCD");
                        ui.slider("scale", 1, 4, &mut self.lcd_scale);
                        // let lcd_image_size = imgui::ImVec2(160 * state.lcd_scale, 144 * state.lcd_scale);
                        let window_size = Ui::window_size(ui);
                        let draw_list = ui.get_window_draw_list();
                        // TODO select button
                    });

                ui.child_window("Memory")
                    .build(|| {
                        ui.separator();
                        ui.text("Memory table tbd");
                        // TODO select button
                    });

            });
        ui.same_line();
        ui.child_window("CPU")
            .build(|| {
                ui.child_window("Registers")
                    .size([200.0, 200.0])
                    .build(|| {
                        ui.text(format!("a     {:02X}", main_board.cpu.a));
                        ui.text(format!("b     {:02X}", main_board.cpu.b));
                        ui.text(format!("c     {:02X}", main_board.cpu.c));
                        ui.text(format!("d     {:02X}", main_board.cpu.d));
                        ui.text(format!("e     {:02X}", main_board.cpu.e));
                        ui.text(format!("hl    {:02X}{:02X}", main_board.cpu.h, main_board.cpu.l));
                        ui.text(format!("pc    {:04X}", main_board.cpu.pc));
                        ui.text(format!("sp    {:04X}", main_board.cpu.sp));
                        ui.text(format!("flags {}", main_board.cpu.flags_as_str()));
                    });
                ui.separator();
                ui.child_window("Disassembly")
                    .size([200.0, 300.0])
                    .build(|| {
                        ui.text(self.get_disassembly_text(&main_board));
                    });
                ui.child_window("Interrupts")
                    .size([200.0, 200.0])
                    .build(|| {
                        ui.text("cycles 12");
                        ui.separator();
                        ui.text("Interrupts: off (enabled / flag)");
                        ui.text("VBLANK: yes / yes");
                        ui.text("LCDSTAT: yes / yes");
                        ui.text("TIMER: yes / yes");
                        ui.text("SERIAL: yes / yes");
                        ui.text("JOYPAD: yes / yes");
                        ui.separator();
                        ui.text("timer: stopped");
                        ui.text("tac: 00: tma: 00");
                        ui.text("tima: 00: div: 0A");
                    });
            });
            ui.child_window("Graphics")
                .size([200.0, 200.0])
                .build(|| {
                    ui.text("background:  on");
                    ui.text("    tileset: on");
                    ui.text("    tilemap: on");
                    ui.text("    scroll: (42, 24)");
                })
        });
        return self.execution_mode
    }

    fn set_disassembly_window_pc(&mut self, main_board: &MainBoard, current_pc: u16) {
        if current_pc < self.disassembly_start_address || current_pc > self.disassembly_end_address {
            self.disassembly_start_address = current_pc;
            self.disassembly_end_address = get_last_address_in_disassembly_text(main_board, current_pc, self.disassembly_lines_to_print);
        }
    }

    fn get_disassembly_text(&mut self, main_board: &MainBoard) -> String {
        let current_pc = main_board.cpu.pc.wrapping_sub(1);
        self.set_disassembly_window_pc(main_board, current_pc);
        let mut result = "Disassembly\n".to_string();
        let mut i = 0;
        while i < self.disassembly_lines_to_print {
            let current_address = self.disassembly_start_address.wrapping_add(1 + i);
            result.push(if current_address == main_board.cpu.pc { '>' } else { ' ' });
            let (operation, size) = get_disassembled_operation(main_board, current_address);
            result.push_str(&operation);
            i = i.wrapping_add(size);
        }
        return result;
    }

 

}

fn get_disassembled_operation(main_board: &MainBoard, pc: u16) -> (String, u16) {
    let opcode = main_board.mmu.borrow().read8(pc);
    if opcode != 0xCB {
        (format!("[{:04X}]   {:02X} | {}\n", pc, opcode, OP_MNEMONICS[opcode as usize]), 1)
    } else {
        let cb_opcode = main_board.mmu.borrow().read8(pc.wrapping_add(1));
        (format!("[{:04X}] {:02X}{:02X} | {}\n", pc, opcode, cb_opcode, OP_CB_MNEMONICS[cb_opcode as usize]), 2)
    }
}

fn get_last_address_in_disassembly_text(main_board: &MainBoard, starting_address: u16, addresses_to_print: u16) -> u16 {
    let mut i = 0;
    let mut current_address = starting_address;
    while i < addresses_to_print {
        let opcode = main_board.mmu.borrow().read8(starting_address.wrapping_add(i));
        current_address = current_address + if opcode == 0xcb { 2 } else { 1 };
        i = i + 1;
    }
    current_address
}

        