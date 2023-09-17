use nalgebra::{Rotation3, Vector3};
use tobj::Model;
use crate::material::Material;
use crate::model::ObjModel;

pub struct Prefab {
    pub name: String,
    pub model: ObjModel,
    pub material: Material,
    pub position: Vector3<f32>,
    pub rotation: Rotation3<f32>,
    pub scale: Vector3<f32>,
    pub weight: f32,
}