use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Light {
    pub r#type: String, // `type` is a reserved word, so escape it
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
}
