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
pub fn tiny_png(file_buffer: &[u8], file_type: &str) -> Vec<u8> {
    // 使用字节数组初始化图像
    let img: image::DynamicImage =
        image::load_from_memory(file_buffer).expect("Failed to load image");
    let width = img.width();
    let height = img.height();
    // 初始化字节流
    let mut buffer = Vec::new();
    // 闭包
    let mut resize = |filter: FilterType, format: image::ImageFormat| {
        let resized = img.resize(width, height, filter);
        resized
            .write_to(&mut Cursor::new(&mut buffer), format)
            .expect("Failed to write image to buffer");
        if buffer >= file_buffer.to_vec() {
            buffer = file_buffer.to_vec();
        }
    };
    match file_type {
        "image/jpeg" => {
            resize(FilterType::Triangle, image::ImageFormat::Jpeg);
        }
        "image/png" => {
            // TODO: 没有效果
            resize(FilterType::Triangle, image::ImageFormat::Png);
        }
        _ => {
            buffer = file_buffer.to_vec();
        }
    }
    return buffer;
}
// 执行以下命令可以打包
// wasm-pack build --target web --out-name rust_tiny_png

// 运行 cargo test 测试
#[cfg(test)]
mod tests {
    use super::*;

    fn resize(in_name: &str, out_name: &str, file_type: &str) {
        let mut file = File::open(in_name).expect("Failed to open image file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("Failed to read image file");
        let out_buffer = tiny_png(&buffer, file_type);
        // 将字节缓冲区写入文件
        let mut new_file = File::create(out_name).expect("Failed to create output file");
        new_file
            .write_all(&out_buffer)
            .expect("Failed to write buffer to file");
        assert!(buffer.len() >= out_buffer.len());
    }

    #[test]
    pub fn test_jpg() {
        resize("input.jpg", "output.jpg", "image/jpeg");
    }

    #[test]
    pub fn test_png() {
        resize("input.png", "output.png", "image/png");
    }

    #[test]
    pub fn test_gif() {
        resize("input.gif", "output.gif", "image/gif");
    }
}
