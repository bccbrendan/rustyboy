use sdl2::event::Event;
mod frontend;
mod gui;
use rustyboy::main_board::MainBoard;


fn main() {
    // set up the emulated hardware
    rog::reg("rustyboy");
    rog::reg("rustyboy::cpu");
    let mut romfile = String::from("");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("a toy gameboy emulator");
        ap.refer(&mut romfile).add_argument("rom", argparse::Store, "Rom filename");
        ap.parse_args_or_exit();
    }
    let mut main_board = MainBoard::init(&romfile[..]).unwrap();
    println!("Loaded rom type: {} title: {}", main_board.mmu.borrow().cartridge.get_type(),
        main_board.mmu.borrow().cartridge.get_title());

    // set up the gui frontend
    let mut frontend = frontend::init();
    let mut gui = gui::Gui { lcd_scale: 2 };
 
    // run the game+gui loop
    'main: loop {
        for event in frontend.event_pump.poll_iter() {
            /* pass all events to imgui platform */
            frontend.platform.handle_event(&mut frontend.imgui, &event);

            if let Event::Quit { .. } = event {
                break 'main;
            }
        }
        /* call prepare_frame before calling imgui.new_frame() */
        frontend.show_gui(&mut gui);
    }
}

/*
fn main() {
    rog::reg("rustyboy");
    rog::reg("rustyboy::cpu");
    let mut romfile = String::from("");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("a toy gameboy emulator");
        ap.refer(&mut romfile).add_argument("rom", argparse::Store, "Rom filename");
        ap.parse_args_or_exit();
    }
    let mut main_board = MainBoard::init(&romfile[..]).unwrap();
    println!("Loaded rom type: {} title: {}", main_board.mmu.borrow().cartridge.get_type(),
        main_board.mmu.borrow().cartridge.get_title());

    'game_loop: loop {
        main_board.emulate_frame();
        match main_board.mmu.borrow_mut().gpu.get_updated_image() {
            None => {}
            Some(updated_image) => {},
        }
    }
}
*/
