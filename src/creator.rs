use std::path::Path;
use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
use image::imageops::FilterType;
use image::io::Reader as ImgReader;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use walkdir::WalkDir;
use crate::data::{Matrix, Mode};

pub fn new_canvas(matrix: &Matrix) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    if matrix.column %2 != 0 && matrix.row %2 != 0 {
        panic!("NO RESOLVE ARGUMENTS!")
    }
    let canvas = RgbImage::new(matrix.width, matrix.height);

    // Generate white lines to separate cells
    //
    //
    //
    // let (h_pc, w_pc) = pixel_per_cell(matrix);
    //
    // for c in 1..matrix.column {
    //     let pos_x = c * w_pc;
    //     for h in 0..canvas.height() {
    //         canvas.put_pixel(pos_x, h, Rgb([255, 255, 255]));
    //     }
    // }
    // for r in 1..matrix.row {
    //     let pos_y = r * h_pc;
    //     for w in 0..canvas.width() {
    //         canvas.put_pixel(w, pos_y, Rgb([255, 255, 255]));
    //     }
    // }

    canvas
}

pub fn load_images(mode: &Mode) -> Vec<DynamicImage> {
    let folder = match mode {
        Mode::Default => { "./res/all" }
        Mode::Lu => { "./res/lu" }
        Mode::Banana => { "./res/.banana" }
        Mode::Strange => { "./res/.strange" }
    };
    let mut images: Vec<DynamicImage> = Vec::new();

    for entry in WalkDir::new(folder).into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file()) {
        images.push(load_image(entry.path()));
    }
    images
}

pub fn fit_images(images: &mut Vec<DynamicImage>, matrix: &Matrix) {
    let (h_pc, w_pc) = pixel_per_cell(matrix);
    if h_pc < 4 && w_pc < 4 {
        panic!("Size Too Small to Show!")
    }
    images.iter_mut().for_each(|pic| {
        *pic = pic.resize_to_fill(w_pc-16, h_pc-16, FilterType::Nearest);
    });
}

pub fn adapt_images(images: &mut Vec<DynamicImage>, matrix: &Matrix, _seed: u64) -> Vec<DynamicImage> {
    let mut new: Vec<DynamicImage> = Vec::new();
    let pics_count = matrix.column * matrix.row;
    let mut rng = thread_rng();

    'lo:
    loop {
        for pic in images.iter() {
            new.push(pic.clone());
            new.push(pic.clone());
            if new.len() >= pics_count as usize {
                break 'lo;
            }
        }
    }
    new.shuffle(&mut rng);
    new.shuffle(&mut rng);
    new
}

pub fn fill_canvas(canvas: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, matrix: &Matrix, images: &Vec<DynamicImage>) {
    let (h_pc, w_pc) = pixel_per_cell(matrix);

    for (index, img) in images.iter().enumerate() {
        let index = index as u32;
        print!("swap: {}\r\n", index + 1);
        let row = index / matrix.column;
        let col = index - row * matrix.column;
        let img = img.clone().into_rgb8();

        for w in 0..img.width() {
            for h in 0..img.height() {
                let pos_x = col * w_pc + w;
                let mut pos_y = row * h_pc + h;
                if pos_y > matrix.height-1 { pos_y = matrix.height-1 };
                canvas.put_pixel(pos_x, pos_y, *img.get_pixel(w, h));
            }
        }
    }
    println!();

}

pub fn write_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let dir = "./banana_img/";
    if !Path::new(dir).is_dir() {
        std::fs::create_dir("./banana_img").expect("error making dir");
    }
    img.save(format!("{}banana_mahjong.jpg", dir)).expect("error saving")
}


fn load_image(file: &Path) -> DynamicImage {
    ImgReader::open(file)
        .expect("error load image")
        .decode()
        .expect("error decode image")
}

fn pixel_per_cell(matrix: &Matrix) -> (u32, u32) {
    // height/cell              width/cell
    (matrix.height/matrix.row, matrix.width/matrix.column)
}
