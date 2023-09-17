use gl::types::{GLchar, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use crate::model::{ObjModel, Vertex};
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

    pub fn bind_buffers(&mut self, model: &ObjModel) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (model.vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
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

            // Check for OpenGL errors
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                panic!("OpenGL error: {:?}", error);
            }

            // Position attribute
            let stride = (8 * std::mem::size_of::<GLfloat>()) as GLsizei;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // Normal attribute
            let normal_offset = (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid;
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, normal_offset);
            gl::EnableVertexAttribArray(1);

            // TexCoord attribute
            let texcoord_offset = (6 * std::mem::size_of::<GLfloat>()) as *const GLvoid;
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, texcoord_offset);
            gl::EnableVertexAttribArray(2);

            // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn render_model(&mut self, model: &ObjModel) {
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