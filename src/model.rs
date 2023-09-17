use gl::types::{GLfloat, GLuint};
use nalgebra_glm::Vec3;

pub struct Vertex {
    pub position: Vec<GLfloat>,
    pub normals: Vec<GLfloat>,
    pub tex_coords: Vec<GLfloat>,
}

pub struct ObjModel {
    //pub vertices: Vec<GLfloat>,
    pub indices: Vec<GLuint>,
    pub vertices: Vec<Vertex>,
    //pub uv: Vec<GLfloat>,
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
                let position = [
                    mesh.positions[v],
                    mesh.positions[v + 1],
                    mesh.positions[v + 2],
                ];

                let normals = if !mesh.normals.is_empty() {
                    [
                        mesh.normals[v],
                        mesh.normals[v + 1],
                        mesh.normals[v + 2],
                    ]
                } else {
                    [0.0, 0.0, 0.0]
                };

                let tex_coords = if !mesh.texcoords.is_empty() {
                    [
                        mesh.texcoords[v / 3 * 2],
                        mesh.texcoords[v / 3 * 2 + 1],
                    ]
                } else {
                    [0.0, 0.0]
                };

                let vertex = Vertex {
                    position: position.to_vec(),
                    normals: normals.to_vec(),
                    tex_coords: tex_coords.to_vec(),
                };

                model.vertices.push(vertex);
            }

            // add indices
            for i in 0..mesh.indices.len() / 3 {
                model.indices.push(mesh.indices[3 * i] as u32);
                model.indices.push(mesh.indices[3 * i + 1] as u32);
                model.indices.push(mesh.indices[3 * i + 2] as u32);
            }
        }

        model
    }
}