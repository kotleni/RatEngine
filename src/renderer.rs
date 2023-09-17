use gl::types::{GLchar, GLfloat, GLint, GLsizeiptr, GLuint, GLvoid};
use tobj::Model;
use crate::model::{ObjModel};
use crate::shader::RatShader;

pub struct RatRenderer {
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
}

impl RatRenderer {
    pub fn new() -> Self {
        RatRenderer {
            vao: 0,
            vbo: 0,
            ebo: 0,
        }
    }

    pub fn clear(&self) {
        unsafe {
            // Clear the screen with depth buffer
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn use_shader(&self, shader: &RatShader) -> GLuint {
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

    pub fn bind_buffers(&mut self, model: &Model) -> i32 {
        let mesh = &model.mesh;

        unsafe {
            let mut index_buf: GLuint = 0;
            let mut pos_buf: GLuint = 0;
            let mut norm_buf: GLuint = 0;
            let mut uv_buf: GLuint = 0;

            gl::GenBuffers(1, &mut index_buf);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buf);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                mesh.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut pos_buf);
            gl::BindBuffer(gl::ARRAY_BUFFER, pos_buf);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.positions.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                mesh.positions.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut norm_buf);
            gl::BindBuffer(gl::ARRAY_BUFFER, norm_buf);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.normals.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                mesh.normals.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut uv_buf);
            gl::BindBuffer(gl::ARRAY_BUFFER, uv_buf);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.texcoords.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                mesh.texcoords.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buf);

            // Position
            gl::BindBuffer(gl::ARRAY_BUFFER, pos_buf);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0); // Vertex position

            // Normal
            gl::BindBuffer(gl::ARRAY_BUFFER, norm_buf);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1); // Normal

            // UV
            gl::BindBuffer(gl::ARRAY_BUFFER, uv_buf);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(2); // UV

            gl::BindVertexArray(0);
        }

        mesh.indices.len() as i32
    }

    pub fn render_model(&mut self, model: &ObjModel) {
        for m in &model.models {
            let total_indices = self.bind_buffers(m);

            model.material.bind();

            unsafe {
                gl::BindVertexArray(self.vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    total_indices,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                gl::BindVertexArray(0);
            }
        }
    }
}