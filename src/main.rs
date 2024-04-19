extern crate image;
use rust_tiny_png::tiny_png;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // 读取图像文件到字节数组
    let mut file = File::open("input.jpg").expect("Failed to open image file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read image file");
    let out_buffer = tiny_png(&buffer);
    // 将字节缓冲区写入文件
    let mut file = File::create("output.jpg").expect("Failed to create output file");
    file.write_all(&out_buffer)
        .expect("Failed to write buffer to file");
}
