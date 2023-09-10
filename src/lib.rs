pub mod data;
pub mod config;
mod log;
mod creator;

use crate::creator::{adapt_images, fill_canvas, fit_images, load_images, new_canvas, write_image};
use crate::data::{Matrix, Mode};
use crate::log::{GenCanceled, GenComplete, GenStart, GenWrapping, Logger};


pub fn auto_gen() {
    GenStart::print_stats();
    GenCanceled::print_stats();
}

pub fn mahjong(matrix: &Matrix, mode: &Mode, seed: u64) {
    println!("matrix: {:?}", matrix);
    GenStart::print_stats();

    let mut canvas = new_canvas(matrix);
    let mut img_sources = load_images(mode);

    fit_images(&mut img_sources, matrix);

    let new_images = adapt_images(&mut img_sources, matrix, seed);

    fill_canvas(&mut canvas, matrix, &new_images);

    GenWrapping::print_stats();

    write_image(canvas);

    GenComplete::print_stats();
}

