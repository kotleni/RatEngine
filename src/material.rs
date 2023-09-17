use gl::types::GLuint;
use crate::shader::RatShader;

pub struct Material {
    pub name: String,
    pub shader: RatShader,
    pub texture_id: GLuint,
}

impl Material {
    pub fn bind(&self) {
        // TODO: self.shader.bind();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }
}