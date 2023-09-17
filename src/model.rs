use crate::material::Material;

pub struct ObjModel {
    pub models: Vec<tobj::Model>,
    pub material: Material,
}

impl ObjModel {
    pub fn from_file(obj_name: &str, mat_name: &str) -> Self {
        let path = format!("assets/models/{}.obj", obj_name);
        let material = Material::from_file(mat_name);

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