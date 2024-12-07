mod scene;
mod bvh;

use scene::load_scene_from_file;
use bvh::BVHNode;

fn main() {
    let scene = load_scene_from_file("scene.json").expect("Failed to load scene");

    let mut objects = scene.objects; // Take ownership of objects
    let bvh_root = BVHNode::new(&mut objects);

    println!("BVH constructed successfully!");
}
