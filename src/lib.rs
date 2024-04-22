// TODO: 这是啥意思
extern crate image;
extern crate oxipng;
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
    // let img: image::DynamicImage =
    //     image::load_from_memory(file_buffer).expect("Failed to load image");
    // let format = match file_type {
    //     // 正常
    //     "image/jpeg" => image::ImageFormat::Jpeg,
    //     // FIXME: 变大了
    //     "image/png" => image::ImageFormat::Png,
    //     // FIXME: 不动了
    //     "image/gif" => image::ImageFormat::Gif,
    //     _ => image::ImageFormat::Jpeg,
    // };
    // 初始化字节流
    let options = oxipng::Options {
        ..Default::default()
    };
    let buffer = match oxipng::optimize_from_memory(file_buffer, &options) {
        Ok(buffer) => buffer,
        Err(e) => {
            eprintln!("Failed to compress image: {}", e);
            file_buffer.to_vec()
        }
    };
    return buffer;
}
// 执行以下命令可以打包
// wasm-pack build --target web --out-name rust_tiny_png

// 运行 cargo test 测试
#[cfg(test)]
mod tests {
    use super::*;

    fn resize(in_name: &str, out_name: &str) {
        let mut file = File::open(in_name).expect("Failed to open image file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("Failed to read image file");
        let out_buffer = tiny_png(&buffer);
        // 将字节缓冲区写入文件
        let mut new_file = File::create(out_name).expect("Failed to create output file");
        new_file
            .write_all(&out_buffer)
            .expect("Failed to write buffer to file");
        assert!(buffer.len() > out_buffer.len());
    }

    #[test]
    pub fn test_jpg() {
        resize("input.jpg", "output.jpg");
    }

    #[test]
    pub fn test_png() {
        resize("input.png", "output.png");
    }

    #[test]
    pub fn test_gif() {
        resize("input.gif", "output.gif");
    }
}
