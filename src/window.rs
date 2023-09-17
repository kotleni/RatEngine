
use nalgebra::{Vector3};
use sdl2::event;
use sdl2::keyboard::Keycode;
use sdl2::video::{GLProfile, Window};
use crate::assets_manager::AssetsManager;
use crate::camera::Camera;
use crate::engine::engine;
use crate::renderer::RatRenderer;

pub struct RatWindow {
    pub sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub sdl_window: Window,
    pub renderer: RatRenderer,
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

        RatWindow {
            sdl,
            video_subsystem,
            sdl_window: window,
            renderer: RatRenderer::new(),
        }
    }

    pub fn run_loop(&mut self) {
        // Set OpenGL context attributes
        self.video_subsystem.gl_attr().set_context_version(3, 3);
        self.video_subsystem.gl_attr().set_context_profile(GLProfile::Core);
        self.video_subsystem.gl_attr().set_context_flags().debug().set();
        // self.video_subsystem.gl_attr().set_double_buffer(true);

        // Create OpenGL context using CreateContextAttribs
        let _gl_context = self.sdl_window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| self.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::MULTISAMPLE);
            //gl::Enable(gl::CULL_FACE);

            // Cull back faces
            // gl::CullFace(gl::FRONT);
            // gl::FrontFace(gl::CCW);

            // Alpha blending
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        let mut camera = Camera::new();

        let object = AssetsManager::load_object("rat");

        let x = 0.0;
        let y = 0.0;
        let z = 5.0;

        // Lock mouse to window
        let mut is_mouse_locked = true;
        self.sdl.mouse().set_relative_mouse_mode(is_mouse_locked);

        let object_position = Vector3::new(x, y, z);
        camera.look_at(&object_position, &Vector3::new(0.0, 1.0, 0.0));

        engine().log("Engine started");

        let mut event_pump = self.sdl.event_pump().unwrap();
        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    event::Event::Quit {..} => break 'main,
                    event::Event::KeyDown { keycode, .. } => {
                        match keycode.unwrap() {
                            Keycode::F3 => {
                                is_mouse_locked = !is_mouse_locked;
                                self.sdl.mouse().set_relative_mouse_mode(is_mouse_locked);
                            }
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }

            camera.process_input(&event_pump.keyboard_state(), &event_pump.relative_mouse_state());

            self.renderer.clear();
            self.renderer.render_model(&object, &camera);
            self.sdl_window.gl_swap_window();
        }
    }
}