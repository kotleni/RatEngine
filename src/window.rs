use sdl2::video::{GLProfile, Window};
use crate::renderer::RatRenderer;

pub struct RatWindow {
    pub sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub sdl_window: Window,
    pub renderer: RatRenderer,
    pub is_mouse_locked: bool,

    _gl_context: sdl2::video::GLContext,
}

impl RatWindow {
    pub fn new(w: u32, h: u32) -> Self {
        let sdl = sdl2::init().unwrap();

        sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "8");
        sdl2::hint::set("SDL_HINT_RENDER_VSYNC", "1");

        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("Rat Engine", w, h)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        // Set OpenGL context attributes
        video_subsystem.gl_attr().set_context_version(3, 3);
        video_subsystem.gl_attr().set_context_profile(GLProfile::Core);
        video_subsystem.gl_attr().set_context_flags().debug().set();

        // Create OpenGL context using CreateContextAttribs
        let _gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        // Configure OpenGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::MULTISAMPLE);

            // Alpha blending
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        RatWindow {
            sdl,
            video_subsystem,
            sdl_window: window,
            renderer: RatRenderer::new(),
            is_mouse_locked: false,

            _gl_context
        }
    }

    pub fn set_mouse_locked(&mut self, is_locked: bool) {
        self.is_mouse_locked = is_locked;
        self.sdl.mouse().set_relative_mouse_mode(is_locked);
    }
}