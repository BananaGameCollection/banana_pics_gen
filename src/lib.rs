pub mod data;
pub mod config;
mod log;

use std::path::Path;
use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
use image::DynamicImage::ImageRgb16;
use crate::data::Matrix;
use crate::log::{GenComplete, GenStart, Logger};
use image::io::Reader as ImgReader;


pub fn auto_gen() {
    GenStart::print_stats();
}

pub fn mahjong(matrix: &Matrix) {
    println!("{:?}", matrix);
    GenStart::print_stats();
    let mut canvas = new_canvas(matrix);

    fill_canvas(&mut canvas, matrix);


    write_image(canvas);
    GenComplete::print_stats();
}

pub fn load_images() -> Vec<DynamicImage> {
    let mut images: Vec<DynamicImage> = Vec::new();

    load_image("");
    vec![]
}


fn load_image(file: &str) -> DynamicImage {
    ImgReader::open(file)
        .expect("error load image")
        .decode()
        .expect("error decode image")
}

pub fn new_canvas(matrix: &Matrix) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let mut img = RgbImage::new(matrix.width, matrix.height);
    let (h_dp, w_dp) = (matrix.height/matrix.row, matrix.width/matrix.column);

    for c in 1..matrix.column {
        let pos_x = c * w_dp;
        for h in 0..img.height() {
            img.put_pixel(pos_x, h, Rgb([255, 255, 255]));
        }
    }
    for r in 1..matrix.row {
        let pos_y = r * h_dp;
        for w in 0..img.width() {
            img.put_pixel(w, pos_y, Rgb([255, 255, 255]));
        }
    }

    img
}

pub fn fill_canvas(canvas: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, matrix: &Matrix) {

}

pub fn write_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let dir = "./banana_img/";
    if !Path::new(dir).is_dir() {
        std::fs::create_dir("./banana_img").expect("error making dir");
    }
    img.save(format!("{}banana_mahjong.jpg", dir)).expect("error saving")
}