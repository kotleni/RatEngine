use gl::types::GLuint;

pub struct Material {
    pub texture_id: GLuint,
}

impl Material {
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }
}