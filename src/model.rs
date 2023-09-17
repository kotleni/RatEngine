use crate::camera::Camera;
use crate::material::Material;
use crate::prefab::Prefab;

pub struct ObjModel {
    pub models: Vec<tobj::Model>,
    pub material: Material,
}

impl ObjModel {
    pub fn bind(&self, prefab: &Prefab, camera: &Camera) {
        self.material.shader.bind(prefab, camera);
        self.material.bind();
    }
}