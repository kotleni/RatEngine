use gl::types::{GLfloat, GLuint};

pub struct ObjModel {
    pub vertices: Vec<GLfloat>,
    pub indices: Vec<GLuint>,
    pub uv: Vec<GLfloat>,
}

impl ObjModel {
    pub fn from_file(name: &str) -> Self {
        let path = format!("assets/models/{}.obj", name);
        let (models, materials) = tobj::load_obj(
            path,
            &tobj::LoadOptions::default()
        ).expect("Failed to OBJ load file");

        let mut model = ObjModel {
            vertices: Vec::new(),
            indices: Vec::new(),
            uv: Vec::new(),
        };

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            // assert!(mesh.positions.len() % 3 == 0);
            // assert!(mesh.texcoords.len() % 2 == 0);
            // assert!(mesh.indices.len() % 3 == 0);

            if mesh.positions.len() % 3 != 0 {
                println!("Mesh positions length is not divisible by 3! SKIP!");
                continue;
            }
            if mesh.texcoords.len() % 2 != 0 {
                println!("Mesh texcoords length is not divisible by 2! SKIP!");
                continue;
            }
            if mesh.indices.len() % 3 != 0 {
                println!("Mesh indices length is not divisible by 3! SKIP!");
                continue;
            }

            println!("Model[{}] '{}': #vertices[{}], #indices[{}], #texcoords[{}]",
                     i, m.name, mesh.positions.len() / 3, mesh.indices.len() / 3, mesh.texcoords.len() / 2);

            // add vertices
            for v in 0..mesh.positions.len() / 3 {
                model.vertices.push(mesh.positions[3 * v]);
                model.vertices.push(mesh.positions[3 * v + 1]);
                model.vertices.push(mesh.positions[3 * v + 2]);
            }

            // add indices
            for i in 0..mesh.indices.len() / 3 {
                model.indices.push(mesh.indices[3 * i] as u32);
                model.indices.push(mesh.indices[3 * i + 1] as u32);
                model.indices.push(mesh.indices[3 * i + 2] as u32);
            }

            // add uv
            for uv in 0..mesh.texcoords.len() / 2 {
                model.uv.push(mesh.texcoords[2 * uv]);
                model.uv.push(mesh.texcoords[2 * uv + 1]);
            }
        }

        model
    }
}