mod objects;
mod camera;
mod light;

pub use objects::Object;
pub use camera::Camera;
pub use light::Light;

use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};

#[derive(Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

pub fn load_scene_from_file(file_path: &str) -> Result<Scene, serde_json::Error> {
    let mut file = File::open(file_path).map_err(serde_json::Error::io)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).map_err(serde_json::Error::io)?;
    serde_json::from_str(&json_string)
}
