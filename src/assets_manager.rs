use std::ffi::CString;
use gl::types::GLuint;
use stb_image::stb_image::bindgen::{stbi_image_free, stbi_load, stbi_set_flip_vertically_on_load};
use crate::material::Material;
use crate::model::ObjModel;
use crate::shader::RatShader;

pub struct AssetsManager {

}

impl AssetsManager {
    pub fn load_shader(name: &str) -> RatShader {
        let vert_path = format!("assets/shaders/{}.vert", name);
        let frag_path = format!("assets/shaders/{}.frag", name);

        let frag_src = std::fs::read_to_string(vert_path).unwrap();
        let vert_src = std::fs::read_to_string(frag_path).unwrap();

        let vert_id = RatShader::compile_shader(&frag_src, gl::VERTEX_SHADER);
        let frag_id = RatShader::compile_shader(&vert_src, gl::FRAGMENT_SHADER);

        RatShader {
            vert_id,
            frag_id,
        }
    }

    pub fn load_material(name: &str) -> Material {
        let path = format!("assets/textures/{}.png", name);
        let cpath = CString::new(path).unwrap();

        let mut texture_id: GLuint = 0;

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::FILL as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::FILL as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let mut width: i32 = 0;
            let mut height: i32 = 0;
            let mut nr_channels: i32 = 0;

            // Enable flipping texture vertically
            // OpenGL expects the 0.0 coordinate on the y-axis to be on the bottom side of the image
            stbi_set_flip_vertically_on_load(i32::from(true));

            // Load texture data from file
            let data = stbi_load(cpath.as_ptr(), &mut width, &mut height, &mut nr_channels, 0);
            if data.is_null() {
                let err = stb_image::stb_image::bindgen::stbi_failure_reason();
                let err = std::ffi::CStr::from_ptr(err).to_str().unwrap();
                panic!("Failed to load texture: {}", err);
            }

            // println!("Loaded texture: {} ({}x{})", name, width, height);

            // Determine texture format
            let format = match nr_channels {
                1 => gl::RED,
                2 => gl::RG,
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => panic!("Invalid number of channels: {}", nr_channels),
            };

            gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, width, height, 0, format, gl::UNSIGNED_BYTE, data as *const std::os::raw::c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            stbi_image_free(data as *mut std::os::raw::c_void);
        }

        Material { texture_id }
    }

    pub fn load_model(obj_name: &str, mat_name: &str) -> ObjModel {
        let path = format!("assets/models/{}.obj", obj_name);
        let material = AssetsManager::load_material(mat_name);

        let mut options = tobj::LoadOptions::default();
        options.triangulate = true;
        options.single_index = true;

        let (models, _) = tobj::load_obj(
            path,
            &options
        ).expect("Failed to OBJ load file");

        ObjModel { models, material }
    }
}