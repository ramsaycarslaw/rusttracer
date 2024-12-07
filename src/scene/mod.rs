mod objects;
mod camera;
mod light;

pub use objects::{Object, Material};
pub use camera::Camera;
pub use light::Light;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

use std::fs::File;
use std::io::Read;
use serde_json::Result;

pub fn load_scene_from_file(file_path: &str) -> Result<Scene> {
    let mut file = File::open(file_path)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;
    serde_json::from_str(&json_string)
}
