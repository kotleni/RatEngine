pub struct ObjModel {
    pub models: Vec<tobj::Model>,
}

impl ObjModel {
    pub fn from_file(name: &str) -> Self {
        let path = format!("assets/models/{}.obj", name);

        let mut options = tobj::LoadOptions::default();
        options.triangulate = true;
        options.single_index = true;

        let (models, materials) = tobj::load_obj(
            path,
            &options
        ).expect("Failed to OBJ load file");

        ObjModel { models }
    }
}