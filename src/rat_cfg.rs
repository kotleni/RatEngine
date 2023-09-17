use std::collections::HashMap;
use nalgebra_glm::{Vec2, Vec3};

pub struct RatCfg {
    pub file_path: String,
    pub dict: HashMap<String, String>,
}

impl RatCfg {
    pub fn get_str(&self, key: &str) -> String {
        self.dict.get(key).unwrap().to_string()
    }

    pub fn get_i32(&self, key: &str) -> i32 {
        self.dict.get(key).unwrap().parse::<i32>().unwrap()
    }

    pub fn get_f32(&self, key: &str) -> f32 {
        self.dict.get(key).unwrap().parse::<f32>().unwrap()
    }

    pub fn get_bool(&self, key: &str) -> bool {
        self.dict.get(key).unwrap().parse::<bool>().unwrap()
    }

    pub fn get_vec2(&self, key: &str) -> Vec2 {
        let mut parts = self.dict.get(key).unwrap().split(',');
        let x = parts.next().unwrap().parse::<f32>().unwrap();
        let y = parts.next().unwrap().parse::<f32>().unwrap();
        Vec2::new(x, y)
    }

    pub fn get_vec3(&self, key: &str) -> Vec3 {
        let mut parts = self.dict.get(key).unwrap().split(',');
        let x = parts.next().unwrap().parse::<f32>().unwrap();
        let y = parts.next().unwrap().parse::<f32>().unwrap();
        let z = parts.next().unwrap().parse::<f32>().unwrap();
        Vec3::new(x, y, z)
    }
}