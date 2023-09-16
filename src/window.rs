use std::ffi::CString;
use nalgebra::{Matrix4, Rotation3, Vector3};
use sdl2::event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use crate::camera::Camera;
use crate::model::ObjModel;
use crate::renderer::RatRenderer;
use crate::shader::RatShader;

pub struct RatWindow {
    pub sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub sdl_window: Window,
    pub renderer: RatRenderer,
}

impl RatWindow {
    pub fn new(w: u32, h: u32) -> Self {
        let sdl = sdl2::init().unwrap();
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
        let _gl_context = self.sdl_window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| self.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);

            // TODO: bug if reverse this - some polygons are not rendered
            // TODO: but if stay current, other polygons rendered uncorrectly
            // Cull back faces
            gl::CullFace(gl::FRONT);
            gl::FrontFace(gl::CCW);

            // Alpha blending
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        let mut camera = Camera::new();

        let model = ObjModel::from_file("teapot");
        let shader = RatShader::from_file("default");

        let x = 0.0;
        let y = 0.0;
        let z = 220.0;

        // Lock mouse to window
        let mut is_mouse_locked = true;
        self.sdl.mouse().set_relative_mouse_mode(is_mouse_locked);

        let mut translation = Matrix4::new_translation(&Vector3::new(x, y, z));
        let mut rotation = Rotation3::from_euler_angles(0.0, 0.0, 0.0).to_homogeneous();
        // let mut rotation = Matrix4::new_rotation( Vector3::new(0.0, 0.0, 0.0));
        let scaling = Matrix4::new_scaling(1.0);

        let object_position = Vector3::new(x, y, z);
        camera.look_at(&object_position, &Vector3::new(0.0, 1.0, 0.0));

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
            let shader_program = self.renderer.use_shader(&shader);
            unsafe {
                // model matrix
                let model_location = gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr());
                let model_matrix = translation * rotation * scaling;
                gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_matrix.as_slice().as_ptr());

                // view matrix
                let view_location = gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr());
                let view = camera.view_matrix;
                gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view.as_slice().as_ptr());

                // projection matrix
                let projection_location = gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());
                gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, camera.projection_matrix.as_slice().as_ptr());

                // light position
                let light_position_location = gl::GetUniformLocation(shader_program, CString::new("lightPos").unwrap().as_ptr());
                gl::Uniform3f(light_position_location, 9.0, 10.0, 9.0);

                // view position
                let light_color_location = gl::GetUniformLocation(shader_program, CString::new("viewPos").unwrap().as_ptr());
                let view_position = camera.position;
                gl::Uniform3f(light_color_location, view_position.x, view_position.y, view_position.z);

                // object color
                let object_color_location = gl::GetUniformLocation(shader_program, CString::new("objectColor").unwrap().as_ptr());
                gl::Uniform3f(object_color_location, 1.0, 0.5, 0.31);
            }

            self.renderer.render_model(&model);

            self.sdl_window.gl_swap_window();
        }
    }
}