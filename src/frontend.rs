use imgui::{
    Context,
    FontId,
    FontSource
};
use sdl2::{
    EventPump,
    video::{GLProfile, Window}
};
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::{SdlPlatform};
use super::gui;


pub struct Frontend {
    pub window: Window,
    pub event_pump: EventPump,
    pub imgui: Context,
    pub platform: SdlPlatform,
    pub renderer: AutoRenderer,
    pub font: FontId,
}


// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

pub fn init() -> Frontend {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);
    /* create a new window, be sure to call opengl method on the builder when using glow! */
    let window = video_subsystem
        .window("Rustyboy", 1280, 720)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    let gl = glow_context(&window);

    let mut imgui = Context::create();
    // set up text font
    let consolas = imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/Consolas.ttf"),
        size_pixels: 13.0,
        config: None,
    }]);

    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
    Frontend {
        window: window,
        event_pump: sdl.event_pump().unwrap(),
        imgui: imgui,
        platform: platform,
        renderer: renderer,
        font: consolas,
    }
}

impl Frontend {
    pub fn show_gui(&mut self, gui: &mut gui::Gui) {

        self.platform.prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        let ui = self.imgui.new_frame();
        gui.show(ui);
        //ui.show_demo_window(&mut true);

        let draw_data = self.imgui.render();

        // unsafe { self.renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        self.renderer.render(draw_data).unwrap();

        self.window.gl_swap_window();
    }

}