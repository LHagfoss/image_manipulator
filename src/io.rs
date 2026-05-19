use image::DynamicImage;
use std::fs;

/// loads image from path
pub fn load(path: &str) -> DynamicImage {
    image::open(path).expect("Failed to open image")
}

/// saves image to a new path (automatically converts format based on extension)
pub fn save(img: &DynamicImage, path: &str) {
    img.save(path).expect("Failed to save image");
}

/// writes raw bytes directly to a file
pub fn write_bytes(path: &str, bytes: &[u8]) {
    fs::write(path, bytes).expect("Failed to write file");
}
