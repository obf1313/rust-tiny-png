extern crate image;
use image::imageops::FilterType;
use image::GenericImageView;

fn main() {
    let img = image::open("input.jpg").expect("Failed to open image");

    let width = img.width();
    let height = img.height();

    let resized_img = img.resize(width, height, FilterType::Lanczos3);
    resized_img
        .save("output.jpg")
        .expect("Failed to save image");
}
