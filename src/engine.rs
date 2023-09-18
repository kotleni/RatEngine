use std::time::Instant;
use egui::{Frame, FullOutput, Style, Widget};
use nalgebra::Vector3;
use sdl2::event;
use sdl2::keyboard::Keycode;
use crate::assets_manager::AssetsManager;
use crate::camera::Camera;
use crate::object::RatObject;
use crate::window::RatWindow;

pub struct Engine {
    pub window: RatWindow,
    pub camera: Camera,

    pub objects: Vec<RatObject>,

    pub log_output: String,
    pub is_running: bool
}

impl Engine {
    pub fn load() -> Self {
        let mut window = RatWindow::new(1400, 800);

        let mut camera = Camera::new();
        let object_position = Vector3::new(0.0, 3.0, 0.0);
        camera.look_at(&object_position, &Vector3::new(0.0, 1.0, 0.0));

        Engine {
            window,
            camera,

            objects: Vec::new(),

            log_output: String::new(),
            is_running: false,
        }
    }

    pub fn run(&mut self) {
        self.is_running = true;
        self.window.set_mouse_locked(true);

        let rat1 = self.load_object("rat");
        let rat2 = self.load_object("rat");
        rat2.position.z = 4.0;

        let start_time = Instant::now();

        let mut is_show_console = false;

        let mut command_line = "";

        let mut event_pump = self.window.sdl.event_pump().unwrap();
        while self.is_running {
            // Process events
            for event in event_pump.poll_iter() {
                match event {
                    event::Event::Quit {..} => self.is_running = false,
                    event::Event::KeyDown { keycode, .. } => {
                        match keycode.unwrap() {
                            Keycode::F2 => {
                                is_show_console = !is_show_console;
                            }
                            Keycode::F3 => {
                                self.window.set_mouse_locked(!self.window.is_mouse_locked);
                            }
                            _ => {},
                        }
                    },
                    _ => {},
                }

                self.window.egui_state.process_input(&self.window.sdl_window, event, &mut self.window.egui_painter);
            }

            if self.window.is_mouse_locked {
                self.camera.process_input(&event_pump.keyboard_state(), &event_pump.relative_mouse_state());
            }
            self.window.renderer.clear();

            unsafe { gl::Enable(gl::DEPTH_TEST); }
            for object in &self.objects {
                self.window.renderer.render_model(&object, &self.camera);
            }
            unsafe { gl::Disable(gl::DEPTH_TEST); }

            self.window.egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
            self.window.egui_ctx.begin_frame(self.window.egui_state.input.take());

            if is_show_console {
                egui::Window::new("Console")
                    .show(&self.window.egui_ctx, |ui| {
                        ui.label(&engine().log_output);

                        // horizontal layout
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut command_line)
                                    .hint_text("Enter command")
                            );
                            if ui.button("Send").clicked() {
                                engine().log(format!("> {}", command_line).as_str());
                            }
                        });
                    });
            }

            let FullOutput {
                platform_output,
                repaint_after,
                textures_delta,
                shapes,
            } = self.window.egui_ctx.end_frame();
            self.window.egui_state.process_output(&self.window.sdl_window, &platform_output);

            let paint_jobs = self.window.egui_ctx.tessellate(shapes);
            self.window.egui_painter.paint_jobs(None, textures_delta, paint_jobs);

            self.window.sdl_window.gl_swap_window();
        }
    }

    pub fn load_object(&mut self, name: &str) -> &mut RatObject {
        let mut object = AssetsManager::load_object(name);
        self.objects.push(object);
        self.objects.last_mut().unwrap()
    }

    pub fn log(&mut self, msg: &str) {
        let text = format!("{}\n", msg);

        println!("{}", text);
        self.log_output.push_str(text.as_str());
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