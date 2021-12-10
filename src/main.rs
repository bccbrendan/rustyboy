
use rustyboy::main_board::MainBoard;

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
