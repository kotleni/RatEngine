use gl::types::GLuint;
use crate::shader::RatShader;

pub struct Material {
    pub name: String,
    pub shader: RatShader,
    pub texture_id: GLuint,
}

impl Material {
    pub fn bind(&self) {
        if self.texture_id != 0 {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            }
        }
    }
}