extern crate image;
use std::io::Cursor;

use image::imageops::FilterType;

/**
 * 压缩图片
 */
pub fn tiny_png(file_buffer: &[u8]) -> Vec<u8> {
    // 使用字节数组初始化图像
    let img = image::load_from_memory(file_buffer).expect("Failed to load image");
    let width = img.width();
    let height = img.height();
    // 压缩图片
    let resized = img.resize(width, height, FilterType::Lanczos3);
    // 初始化字节流
    let mut buffer = Vec::new();
    resized
        .write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Jpeg)
        .expect("Failed to write image to buffer");
    return buffer;
}
