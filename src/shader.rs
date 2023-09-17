use std::ffi::CString;
use gl::types::{GLchar, GLint, GLuint};
use nalgebra::Matrix4;
use crate::camera::Camera;
use crate::model::ObjModel;
use crate::prefab::Prefab;

pub struct RatShader {
    pub vert_id: GLuint,
    pub frag_id: GLuint,
    pub shader_program: GLuint,
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

    pub fn use_shader(&mut self) {
        unsafe {
            self.shader_program = gl::CreateProgram();
            gl::AttachShader(self.shader_program, self.vert_id);
            gl::AttachShader(self.shader_program, self.frag_id);
            gl::LinkProgram(self.shader_program);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(self.shader_program, gl::LINK_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(self.shader_program, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::with_capacity(len as usize);
                info_log.set_len((len as usize) - 1); // subtract 1 to skip the null terminator
                gl::GetProgramInfoLog(
                    self.shader_program,
                    len,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "Shader program linking failed:\n{}",
                    String::from_utf8_lossy(&info_log)
                );
            }
            gl::DeleteShader(self.vert_id);
            gl::DeleteShader(self.frag_id);
        }
    }

    pub fn bind(&self, prefab: &Prefab, camera: &Camera) {
        unsafe {
            // model matrix
            let model_location = gl::GetUniformLocation(self.shader_program, CString::new("model").unwrap().as_ptr());
            let translation = Matrix4::new_translation(&prefab.position);
            let rotation = Matrix4::from(prefab.rotation);
            let scale = Matrix4::new_scaling(1.0); // TODO: Not implemented yet

            let model_matrix = translation * rotation * scale;
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_matrix.as_slice().as_ptr());

            // view matrix
            let view_location = gl::GetUniformLocation(self.shader_program, CString::new("view").unwrap().as_ptr());
            let view = camera.view_matrix;
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view.as_slice().as_ptr());

            // projection matrix
            let projection_location = gl::GetUniformLocation(self.shader_program, CString::new("projection").unwrap().as_ptr());
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, camera.projection_matrix.as_slice().as_ptr());

            // light position
            let light_position_location = gl::GetUniformLocation(self.shader_program, CString::new("lightPos").unwrap().as_ptr());
            gl::Uniform3f(light_position_location, 9.0, 10.0, 9.0);

            // view position
            let light_color_location = gl::GetUniformLocation(self.shader_program, CString::new("viewPos").unwrap().as_ptr());
            let view_position = camera.position;
            gl::Uniform3f(light_color_location, view_position.x, view_position.y, view_position.z);

            // object color
            let object_color_location = gl::GetUniformLocation(self.shader_program, CString::new("objectColor").unwrap().as_ptr());
            gl::Uniform3f(object_color_location, 1.0, 0.5, 0.31);

            gl::UseProgram(self.shader_program);
        }
    }
}