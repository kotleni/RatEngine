use crate::camera::Camera;
use crate::material::Material;
use crate::object::RatObject;

pub struct ObjModel {
    pub models: Vec<tobj::Model>,
    pub material: Material,
}

impl ObjModel {
    pub fn bind(&self, object: &RatObject, camera: &Camera) {
        self.material.shader.bind(object, camera);
        self.material.bind();
    }
}