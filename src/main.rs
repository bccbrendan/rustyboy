mod gui;
use rustyboy::main_board::MainBoard;

use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::{
    event::Event,
    video::{GLProfile, Window},
};

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}


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

    /* initialize SDL and its video subsystem */
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    /* hint SDL to initialize an OpenGL 3.3 core profile context */
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    /* create a new window, be sure to call opengl method on the builder when using glow! */
    let window = video_subsystem
        .window("Hello imgui-rs!", 1280, 720)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    /* create a new OpenGL context and make it current */
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    /* enable vsync to cap framerate */
    window.subsystem().gl_set_swap_interval(1).unwrap();

    /* create new glow and imgui contexts */
    let gl = glow_context(&window);

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    /* setup platform and renderer, and fonts to imgui */
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut gui = gui::Gui { lcd_scale: 2 };

    'main: loop {
        for event in event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            platform.handle_event(&mut imgui, &event);

            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        platform.prepare_frame(&mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();
        gui.show(ui);

        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();

        // main_board.emulate_frame();
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
