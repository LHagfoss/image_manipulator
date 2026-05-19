use image::DynamicImage;
use std::io::Cursor;

/// resizes an image to exact width and height
pub fn resize(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
}

/// compresses an image into jpeg bytes in memory
pub fn compress(img: &DynamicImage, quality: u8) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, quality);
    encoder.encode_image(img).expect("Failed to compress image");

    buffer.into_inner()
}
