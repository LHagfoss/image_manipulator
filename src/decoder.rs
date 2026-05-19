use image::{DynamicImage, GenericImageView};

/// loads image from path
pub fn load(path: &str) -> DynamicImage {
    image::open(path).expect("Failed to open image")
}

/// converts image to buffer for display
/// uses RGBA pixel data to create a buffer of 32-bit unsigned integers
pub fn convert(img: &DynamicImage) -> (u32, u32, Vec<u32>) {
    let (width, height) = img.dimensions();

    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];

    for (x, y, rgba) in img.pixels() {
        let r = rgba[0] as u32;
        let g = rgba[1] as u32;
        let b = rgba[2] as u32;

        let rgb = (r << 16) | (g << 8) | b;
        buffer[(y * width + x) as usize] = rgb;
    }

    (width, height, buffer)
}
