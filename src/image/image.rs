use image::{ImageBuffer, Rgb};

pub fn save_image(buffer: &[u8], width: usize, height: usize, filename: &str) {
    let img = ImageBuffer::<Rgb<u8>, _>::from_raw(width as u32, height as u32, buffer.to_vec())
        .expect("Failed to create image buffer");
    img.save(filename).expect("Failed to save image");
}