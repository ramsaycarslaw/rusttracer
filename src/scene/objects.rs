use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Material {
    pub color: [f32; 3],
    pub reflection: f32,
    pub refraction: f32,
    pub shininess: u32,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Object {
    Sphere {
        position: [f32; 3],
        radius: f32,
        material: Material,
    },
    Plane {
        position: [f32; 3],
        normal: [f32; 3],
        material: Material,
    },
}
