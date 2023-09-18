use lazy_static::lazy_static;
use nalgebra::Vector3;
use sdl2::event;
use sdl2::keyboard::Keycode;
use crate::assets_manager::AssetsManager;
use crate::camera::Camera;
use crate::window::RatWindow;

pub struct Engine {
    pub window: RatWindow,
    pub camera: Camera,

    pub is_running: bool
}

impl Engine {
    pub fn load() -> Self {
        let mut window = RatWindow::new(800, 600);

        let mut camera = Camera::new();
        let object_position = Vector3::new(0.0, 3.0, 0.0);
        camera.look_at(&object_position, &Vector3::new(0.0, 1.0, 0.0));

        Engine {
            window,
            camera,

            is_running: false,
        }
    }

    pub fn run(&mut self) {
        self.is_running = true;
        self.window.set_mouse_locked(true);

        let object = AssetsManager::load_object("rat");

        let mut event_pump = self.window.sdl.event_pump().unwrap();
        while self.is_running {
            for event in event_pump.poll_iter() {
                match event {
                    event::Event::Quit {..} => self.is_running = false,
                    event::Event::KeyDown { keycode, .. } => {
                        match keycode.unwrap() {
                            Keycode::F3 => {
                                self.window.set_mouse_locked(!self.window.is_mouse_locked);
                            }
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }

            self.camera.process_input(&event_pump.keyboard_state(), &event_pump.relative_mouse_state());

            self.window.renderer.clear();
            self.window.renderer.render_model(&object, &self.camera);
            self.window.sdl_window.gl_swap_window();
        }
    }

    pub fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

// MARK: big brain singleton
// YOU CAN'T USE engine() BEFORE IT'S INITIALIZED!
static mut ENGINE: Option<Engine> = None;
pub fn engine() -> &'static mut Engine {
    unsafe {
        if ENGINE.is_none() {
            ENGINE = Some(Engine::load());
        }
        ENGINE.as_mut().unwrap()
    }
}