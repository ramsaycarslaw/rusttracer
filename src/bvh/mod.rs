mod aabb;
pub use aabb::AABB;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
}

impl Ray {
    pub fn new(origin: [f32; 3], direction: [f32; 3]) -> Self {
        let norm = (direction[0].powi(2) + direction[1].powi(2) + direction[2].powi(2)).sqrt();
        Self {
            origin,
            direction: [direction[0] / norm, direction[1] / norm, direction[2] / norm],
        }
    }
}

pub fn render(scene: &crate::scene::Scene, width: usize, height: usize) -> Vec<u8> {
    let mut buffer = vec![0; width * height * 3];

    let camera = &scene.camera;
    let fov_scale = (camera.fov.to_radians() / 2.0).tan();

    for y in 0..height {
        for x in 0..width {
            let ndc_x = (x as f32 + 0.5) / width as f32 * 2.0 - 1.0;
            let ndc_y = 1.0 - (y as f32 + 0.5) / height as f32 * 2.0;

            let ray_dir = [
                ndc_x * fov_scale * width as f32 / height as f32,
                ndc_y * fov_scale,
                -1.0,
            ];
            let ray = Ray::new(camera.position, ray_dir);

            let intersects = scene.objects.iter().any(|obj| obj.intersects(&ray));

            let offset = (y * width + x) * 3;
            if intersects {
                buffer[offset + 2] = 255; // Blue
            }
        }
    }

    buffer
}

use image::{ImageBuffer, Rgb};

pub fn save_image(buffer: &[u8], width: usize, height: usize, filename: &str) {
    let img = ImageBuffer::<Rgb<u8>, _>::from_raw(width as u32, height as u32, buffer.to_vec())
        .expect("Failed to create image buffer");
    img.save(filename).expect("Failed to save image");
}
