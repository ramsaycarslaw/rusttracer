use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Camera {
    pub position: [f32; 3],
    pub look_at: [f32; 3],
    pub up: [f32; 3],
    pub fov: f32,
}
