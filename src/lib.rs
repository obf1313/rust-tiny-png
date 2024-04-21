// TODO: 这是啥意思
extern crate image;
use image::imageops::FilterType;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

/**
 * 压缩图片
 */
#[wasm_bindgen]
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
// 执行以下命令可以打包
// wasm-pack build --target web --out-name rust_tiny_png

// 运行 cargo test 测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_size() {
        let mut file = File::open("input.jpg").expect("Failed to open image file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("Failed to read image file");
        let out_buffer = tiny_png(&buffer);
        // 将字节缓冲区写入文件
        let mut new_file = File::create("output.jpg").expect("Failed to create output file");
        new_file
            .write_all(&out_buffer)
            .expect("Failed to write buffer to file");
        assert!(buffer.len() > out_buffer.len());
    }
}
