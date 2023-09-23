use std::fmt::format;
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
        // Prepare window
        let mut window = RatWindow::new(1400, 800);

        // Prepare camera
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

        engine().execute_commands_file("autoexec.cmds");

        let start_time = Instant::now();

        let mut is_show_console = false;

        let mut command_line = String::new();

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

                                self.window.set_mouse_locked(!is_show_console);
                            }
                            Keycode::F3 => {
                                self.window.set_mouse_locked(!self.window.is_mouse_locked);

                                if self.window.is_mouse_locked {
                                    is_show_console = false;
                                }
                            }
                            _ => { },
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
                                engine().execute_command(format!("{}", command_line).as_str());
                            }
                        });
                    });
            }

            if is_show_console {
                let FullOutput {
                    platform_output,
                    repaint_after,
                    textures_delta,
                    shapes,
                } = self.window.egui_ctx.end_frame();
                self.window.egui_state.process_output(&self.window.sdl_window, &platform_output);

                let paint_jobs = self.window.egui_ctx.tessellate(shapes);
                self.window.egui_painter.paint_jobs(None, textures_delta, paint_jobs);
            }

            self.window.sdl_window.gl_swap_window();
        }
    }

    // MARK: This method contains bad code
    // TODO: I need to refactor this method
    pub fn load_object(&mut self, _name: &str) -> &mut RatObject {
        let mut name = _name.to_string();

        // Make name unique
        let mut index = 0;
        for object in &self.objects {
            let object_name = &object.name;
            if object_name.starts_with(&name) {
                if object_name.len() == name.len() {
                    index = 1; // The name itself is present, so start with 1
                } else if let Some(suffix) = object_name[name.len()..].strip_prefix('_') {
                    if let Ok(suffix_index) = suffix.parse::<u32>() {
                        index = index.max(suffix_index + 1);
                    }
                }
            }
        }

        if index > 0 {
            name.push_str(&format!("_{}", index));
        }


        let mut object = AssetsManager::load_object(_name);
        object.name = name;
        self.objects.push(object);

        let object_inmemory = self.objects.last_mut().unwrap();
        engine().log(format!("Loaded object: {}", object_inmemory.name.to_string()).as_str());
        object_inmemory
    }

    pub fn execute_command(&mut self, command: &str) {
        engine().log(format!("> {}", command).as_str());

        let mut args = command.split_whitespace();
        let command_name = args.next().unwrap();

        match command_name {
            "obj" => {
                let operation = args.next().unwrap();

                match operation {
                    "load" => {
                        let object_name = args.next().unwrap();
                        self.load_object(object_name);
                    }
                    "list" => {
                        for object in &engine().objects {
                            self.log(format!("{} ({})", object.name, object.position).as_str());
                        }
                    }
                    "remove" => {
                        let object_name = args.next().unwrap();
                        let mut index = 0;
                        for object in &self.objects {
                            if object.name == object_name {
                                break;
                            }
                            index += 1;
                        }
                        self.objects.remove(index);
                    }
                    "moveto" => {
                        let object_name = args.next().unwrap();
                        let x = args.next().unwrap().parse::<f32>().unwrap();
                        let y = args.next().unwrap().parse::<f32>().unwrap();
                        let z = args.next().unwrap().parse::<f32>().unwrap();
                        let mut index = 0;
                        for object in &mut engine().objects {
                            if object.name == object_name {
                                object.position = Vector3::new(x, y, z);
                                engine().log(format!("Moved object {} to ({}, {}, {})", object.name, x, y, z).as_str());
                                break;
                            }
                            index += 1;
                        }
                    }
                    _ => {
                        self.log(format!("Unknown operation: {}", operation).as_str());
                    }
                }
            }
            "quit" => {
                self.is_running = false;
            }
            _ => {
                self.log(format!("Unknown command: {}", command_name).as_str());
            }
        }
    }

    pub fn execute_commands_file(&mut self, file_path: &str) {
        let path = format!("assets/{}", file_path);
        let file = std::fs::read_to_string(path).unwrap();
        for line in file.lines() {
            if line.starts_with(';') || line.is_empty() {
                continue;
            }
            self.execute_command(line);
        }
    }

    pub fn log(&mut self, msg: &str) {
        let text = format!("{}\n", msg);

        print!("{}", text);
        self.log_output.push_str(text.as_str());
    }
}

// MARK: Big brain singleton
// MARK: Maybe I should use lazy_static instead?
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