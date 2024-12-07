mod scene;
mod bvh;
mod image;

use scene::load_scene_from_file;
use bvh::{render};
use image::save_image;

fn main() {
    let scene = load_scene_from_file("scene.json").expect("Failed to load scene");

    let width = 800;
    let height = 600;
    let buffer = render(&scene, width, height);

    save_image(&buffer, width, height, "output.png");
    println!("Image saved to output.png");
}
