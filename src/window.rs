use sdl2::video::{GLProfile, Window};
use crate::renderer::RatRenderer;
use egui_sdl2_gl::ShaderVersion;
use egui_sdl2_gl::DpiScaling;
use egui_sdl2_gl::painter::Painter;
use egui_sdl2_gl as egui_backend;
use gl::types::{GLchar, GLenum, GLsizei, GLuint};

pub struct RatWindow {
    pub sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub sdl_window: Window,
    pub renderer: RatRenderer,
    pub egui_painter: Painter,
    pub egui_state: egui_backend::EguiStateHandler,
    pub egui_ctx: egui::Context,
    pub is_mouse_locked: bool,

    _gl_context: sdl2::video::GLContext,
}

extern "system" fn gl_debug_callback(source: GLenum,
                                     gltype: GLenum,
                                     id: GLuint,
                                     severity: GLenum,
                                     _length: GLsizei,
                                     message: *const GLchar,
                                     _user_param: *mut std::ffi::c_void) {
    let message = unsafe { std::ffi::CStr::from_ptr(message).to_str().unwrap() };
    if severity != gl::DEBUG_SEVERITY_NOTIFICATION {
        let formatted = format!("OpenGL debug message: {:?} {:?} {:?} {:?} {:?}", source, gltype, id, severity, message);
        println!("{}", formatted);
        // MARK: removed: engine().log(formatted.as_str());
        // MARK: gl logs too much messages at this time
    }
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

        let (egui_painter, egui_state) = egui_backend::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Custom(1.0));
        let egui_ctx = egui::Context::default();

        // Configure OpenGL
        unsafe {
            //gl::Enable(gl::DEPTH_TEST); // moved to render pipeline
            gl::Enable(gl::MULTISAMPLE);
            gl::Enable(gl::DEBUG_CALLBACK_FUNCTION);

            gl::DebugMessageCallback(Some(gl_debug_callback), std::ptr::null());

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
            egui_painter,
            egui_state,
            egui_ctx,
            is_mouse_locked: false,

            _gl_context
        }
    }

    pub fn set_mouse_locked(&mut self, is_locked: bool) {
        self.is_mouse_locked = is_locked;
        self.sdl.mouse().set_relative_mouse_mode(is_locked);
    }
}