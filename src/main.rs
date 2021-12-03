use rustyboy::main_board::MainBoard;

fn main() {
    println!("Hello, world!");
    rog::reg("rustyboy");
    let mut romfile = String::from("");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("a toy gameboy emulator");
        ap.refer(&mut romfile).add_argument("rom", argparse::Store, "Rom filename");
        ap.parse_args_or_exit();
    }
    let mut main_board = MainBoard::init(&romfile[..]).unwrap();
    println!("Loaded rom type: {} title: lol unknown", main_board.mmu.borrow().cartridge.get_type());
}
