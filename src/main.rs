extern crate sdl2;
extern crate gl;

use std::ffi::CString;
use sdl2::video::Window;
use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLvoid};
use gl::types::GLuint;
use gl::types::GLint;
use gl::types::GLchar;
use na::{Matrix4, Vector3};
use nalgebra as na;
use nalgebra::{one, Rotation3, zero};

struct RatShader {
    vert_id: GLuint,
    frag_id: GLuint,
}

impl RatShader {
    fn load_shader(path: &str, shader_type: GLuint) -> GLuint {
        let src = std::fs::read_to_string(path).unwrap();

        let shader;
        unsafe {
            shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, &(src.as_ptr() as *const i8), &(src.len() as i32));
            gl::CompileShader(shader);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::with_capacity(len as usize);
                info_log.set_len((len as usize) - 1); // subtract 1 to skip the null terminator
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "Shader compilation failed:\n{}",
                    String::from_utf8_lossy(&info_log)
                );
            }
        }

        shader
    }

    fn from_file(path: &str) -> Self {
        let vert_id = RatShader::load_shader(&*(path.to_owned() + ".vert"), gl::VERTEX_SHADER);
        let frag_id = RatShader::load_shader(&*(path.to_owned() + ".frag    "), gl::FRAGMENT_SHADER);

        RatShader {
            vert_id,
            frag_id,
        }
    }
}

struct ObjModel {
    vertices: Vec<GLfloat>,
    indices: Vec<GLuint>,
    uv: Vec<GLfloat>,
}

impl ObjModel {
    fn from_file(path: &str) -> Self {

        let (models, materials) = tobj::load_obj(
                path,
                &tobj::LoadOptions::default()
            ).expect("Failed to OBJ load file");

        let mut model = ObjModel {
            vertices: Vec::new(),
            indices: Vec::new(),
            uv: Vec::new(),
        };

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            // model.vertices.extend(&mesh.positions);
            // model.uv.extend(&mesh.texcoords);
            // model.indices.extend(&mesh.indices);

            assert!(mesh.positions.len() % 3 == 0);
            assert!(mesh.texcoords.len() % 2 == 0);
            assert!(mesh.indices.len() % 3 == 0);

            println!("Model[{}] '{}': #vertices[{}], #indices[{}], #texcoords[{}]",
                i, m.name, mesh.positions.len() / 3, mesh.indices.len() / 3, mesh.texcoords.len() / 2);

            // add vertices
            for v in 0..mesh.positions.len() / 3 {
                model.vertices.push(mesh.positions[3 * v]);
                model.vertices.push(mesh.positions[3 * v + 1]);
                model.vertices.push(mesh.positions[3 * v + 2]);
            }

            // add indices
            for i in 0..mesh.indices.len() / 3 {
                model.indices.push(mesh.indices[3 * i] as u32);
                model.indices.push(mesh.indices[3 * i + 1] as u32);
                model.indices.push(mesh.indices[3 * i + 2] as u32);
            }

            // add uv
            for uv in 0..mesh.texcoords.len() / 2 {
                model.uv.push(mesh.texcoords[2 * uv]);
                model.uv.push(mesh.texcoords[2 * uv + 1]);
            }
        }

        model
    }
}

struct RatRenderer {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

struct RatWindow {
    sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    sdl_window: Window,
    renderer: RatRenderer,
}

impl RatRenderer {
    fn new() -> Self {
        RatRenderer {
            vao: 0,
            vbo: 0,
            ebo: 0,
        }
    }

    fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn use_shader(&self, shader: &RatShader) -> GLuint {
        let shader_program;
        unsafe {
            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, shader.vert_id);
            gl::AttachShader(shader_program, shader.frag_id);
            gl::LinkProgram(shader_program);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::with_capacity(len as usize);
                info_log.set_len((len as usize) - 1); // subtract 1 to skip the null terminator
                gl::GetProgramInfoLog(
                    shader_program,
                    len,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "Shader program linking failed:\n{}",
                    String::from_utf8_lossy(&info_log)
                );
            }
            gl::DeleteShader(shader.vert_id);
            gl::DeleteShader(shader.frag_id);
        }

        unsafe {
            gl::UseProgram(shader_program);
        }

        shader_program
    }

    fn bind_buffers(&mut self, model: &ObjModel) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (model.vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                model.vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (model.indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                model.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // Configure vertex attributes here
            let stride = 3 * std::mem::size_of::<GLfloat>() as GLsizei;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    fn render_model(&mut self, model: &ObjModel) {
        self.bind_buffers(model);

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                model.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }
}

impl RatWindow {
    fn new(w: u32, h: u32) -> Self {
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

    pub fn look_at(&self, eye: &Vector3<f32>, center: &Vector3<f32>, up: &Vector3<f32>) -> Matrix4<f32> {
        let f = (center - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(&f);

        Matrix4::new(
            s.x, u.x, -f.x, 0.0,
            s.y, u.y, -f.y, 0.0,
            s.z, u.z, -f.z, 0.0,
            -eye.dot(&s), -eye.dot(&u), eye.dot(&f), 1.0,
        )
    }

    fn run_loop(&mut self) {
        let _gl_context = self.sdl_window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| self.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        let model = ObjModel::from_file("assets/teapot.obj");
        let shader = RatShader::from_file("assets/default");

        let x = 0.0;
        let y = 0.0;
        let z = 220.0;

        let mut translation = Matrix4::new_translation(&Vector3::new(x, y, z));
        let mut rotation = Rotation3::from_euler_angles(0.0, 0.0, 0.0).to_homogeneous();
        // let mut rotation = Matrix4::new_rotation( Vector3::new(0.0, 0.0, 0.0));
        let scaling = Matrix4::new_scaling(1.0);

        // 3.14 / 2.0
        let projection = Matrix4::new_perspective(800.0 / 600.0, 45.0, 0.1, 1000.0);

        let view = self.look_at(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 0.0, 3.0),
            &Vector3::new(0.0, 1.0, 0.0),
        );

        let mut event_pump = self.sdl.event_pump().unwrap();
        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'main,
                    _ => {},
                }
            }

            self.renderer.clear();
            let shader_program = self.renderer.use_shader(&shader);
            unsafe {
                let model_location = gl::GetUniformLocation(shader_program, CString::new("model").unwrap().as_ptr());
                let model_matrix = translation * rotation * scaling;
                gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_matrix.as_slice().as_ptr());

                let view_location = gl::GetUniformLocation(shader_program, CString::new("view").unwrap().as_ptr());
                gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view.as_slice().as_ptr());

                let projection_location = gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());
                gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection.as_slice().as_ptr());
            }

            self.renderer.render_model(&model);

            rotation = rotation * Rotation3::from_euler_angles(0.0, 0.01, 0.0).to_homogeneous();

            self.sdl_window.gl_swap_window();
        }
    }
}

fn main() {
    let mut window = RatWindow::new(800, 600);
    window.run_loop();
}