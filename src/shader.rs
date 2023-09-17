use gl::types::{GLchar, GLint, GLuint};

pub struct RatShader {
    pub vert_id: GLuint,
    pub frag_id: GLuint,
}

impl RatShader {
    pub fn compile_shader(src: &str, shader_type: GLuint) -> GLuint {
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
}