use gl::types::{GLfloat, GLsizeiptr, GLuint, GLvoid};
use tobj::Model;
use crate::camera::Camera;
use crate::object::RatObject;

pub struct RatRenderer {
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,

    index_buf: GLuint,
    pos_buf: GLuint,
    norm_buf: GLuint,
    uv_buf: GLuint,
}

impl RatRenderer {
    pub fn new() -> Self {
        RatRenderer {
            vao: 0,
            vbo: 0,
            ebo: 0,

            index_buf: 0,
            pos_buf: 0,
            norm_buf: 0,
            uv_buf: 0,
        }
    }

    pub fn clear(&self) {
        unsafe {
            // Clear the screen with depth buffer
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn bind_buffers(&mut self, model: &Model) -> i32 {
        let mesh = &model.mesh;

        unsafe {
            gl::GenBuffers(1, &mut self.index_buf);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buf);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                mesh.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut self.pos_buf);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.pos_buf);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.positions.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                mesh.positions.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut self.norm_buf);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.norm_buf);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.normals.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                mesh.normals.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut self.uv_buf);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buf);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.texcoords.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                mesh.texcoords.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buf);

            // Position
            gl::BindBuffer(gl::ARRAY_BUFFER, self.pos_buf);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0); // Vertex position

            // Normal
            gl::BindBuffer(gl::ARRAY_BUFFER, self.norm_buf);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1); // Normal

            // UV
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buf);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(2); // UV

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            gl::DeleteBuffers(1, &self.index_buf);
            gl::DeleteBuffers(1, &self.pos_buf);
            gl::DeleteBuffers(1, &self.norm_buf);
            gl::DeleteBuffers(1, &self.uv_buf);
        }

        mesh.indices.len() as i32
    }

    pub fn unbind_buffers(&self) {
        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            gl::DeleteVertexArrays(1, &self.vao);
            // gl::DeleteVertexArrays(1, &self.vbo);
            // gl::DeleteVertexArrays(1, &self.ebo);
        }
    }

    pub fn render_model(&mut self, object: &RatObject, camera: &Camera) {
        let model = &object.model;
        for m in &model.models {
            let total_indices = self.bind_buffers(m);

            model.bind(object, camera);

            unsafe {
                gl::BindVertexArray(self.vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    total_indices,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }

            self.unbind_buffers();
        }
    }
}