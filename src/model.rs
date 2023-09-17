use crate::material::Material;

pub struct ObjModel {
    pub models: Vec<tobj::Model>,
    pub material: Material,
}