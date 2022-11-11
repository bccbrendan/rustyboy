use imgui::Ui;

pub struct Gui {
    pub lcd_scale: u8,
}

impl Default for Gui {
    fn default() -> Self {
        Gui {
            lcd_scale: 1,
        }
    }
}

impl Gui {

    pub fn show(&mut self, ui: &mut Ui) {
        ui.window("Rustyboy")
            .size([0.0, 0.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("dummy    0xdead");
                /*ui.child_window("Cartridge and Audio")
                    .size([200.00, 100.0])
                    .build(|| {
                        ui.child_window("Cartridge")
                            .build(|| {
                                ui.text("title: 123456789AB");
                                ui.text("Type: MCB1?");
                                ui.text("CCB: None");
                                ui.button("Load ROM");
                        });
                    ui.separator();
                    ui.child_window("Controls")
                        .build(|| {
                            ui.button("Go");
                            ui.same_line();
                            ui.button("Stop");
                            ui.button("step");
                            ui.button("step frame");
                    });
                    ui.separator();
                    ui.child_window("APU")
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
                */
        //ui.same_line();
        /*ui.child_window("LCD and memory")
            .size([500.00, 100.0])
            .build(|| {
                ui.child_window("LCD")
                    .build(|| {
                        ui.text("LCD");
                        ui.slider("scale", 1, 4, &mut self.lcd_scale);
                        // let lcd_image_size = imgui::ImVec2(160 * state.lcd_scale, 144 * state.lcd_scale);
                        let window_size = Ui::window_size(ui);
                        // imgui::image();
                        // TODO select button
                    });

                ui.child_window("Memory")
                    .build(|| {
                        ui.separator();
                        ui.text("Memory table tbd");
                        // TODO select button
                    });

            });*/
        //ui.same_line();
        /*ui.child_window("CPU")
            .build(|| {
                ui.child_window("Registers")
                    .size([200.0, 200.0])
                    .build(|| {
                        ui.text("a    00");
                        ui.text("b    00");
                        ui.text("c    00");
                        ui.text("d    00");
                        ui.text("e    00");
                        ui.text("hl 1200");
                        ui.text("pc 1200");
                        ui.text("sp 2300");
                        ui.text("flags _ _ _ _");
                    });
                    */
         //       ui.separator();
                /*
                ui.child_window("Disassembly")
                    .size([200.0, 300.0])
                    .build(|| {
                        ui.text("Disassembly");
                        ui.text(">4242: LD A, A");
                        ui.text(" 4243: AND A, A");
                        ui.text(" 4244: XOR A, A");
                    });
                ui.child_window("Interrupts")
                    .size([0.0, 1.0])
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
                    */
        //    });
          /*  ui.child_window("Graphics")
                .build(|| {
                    ui.text("background:  on");
                    ui.text("    tileset: on");
                    ui.text("    tilemap: on");
                    ui.text("    scroll: (42, 24)");
                }) */
        });
    }

}