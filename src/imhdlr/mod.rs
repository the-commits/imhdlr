use ext_php_rs::prelude::*;
use rayon::{iter::ParallelIterator, prelude::IntoParallelIterator};

mod utils;
use crate::imhdlr::utils::{
    get_images, process_crop, process_resize_exact, process_resize_to_fill,
};

#[php_function]
pub fn imhdlr_get(dir: &str) -> Vec<String> {
    let images = get_images(dir);
    images
        .into_iter()
        .flatten()
        .map(|path| path.to_string_lossy().into_owned())
        .collect()
}

#[php_function]
pub fn imhdlr_resize_exact(dir: &str, resize_width: u32, resize_height: u32, verbose: bool) {
    for images in get_images(dir) {
        images.into_par_iter().for_each(move |image| {
            if let Err(e) = process_resize_exact(image, resize_width, resize_height, verbose) {
                eprintln!("Error processing image: {:?}", e);
            }
        });
    }
}

#[php_function]
pub fn imhdlr_crop(dir: &str, resize_width: u32, resize_height: u32, verbose: bool) {
    for images in get_images(dir) {
        images.into_par_iter().for_each(move |image| {
            if let Err(e) = process_crop(image, resize_width, resize_height, verbose) {
                eprintln!("Error processing image: {:?}", e);
            }
        });
    }
}

#[php_function]
pub fn imhdlr_resize_to_fill(dir: &str, resize_width: u32, resize_height: u32, verbose: bool) {
    for images in get_images(dir) {
        images.into_par_iter().for_each(move |image| {
            if let Err(e) = process_resize_to_fill(image, resize_width, resize_height, verbose) {
                eprintln!("Error processing image: {:?}", e);
            }
        });
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

#[cfg(test)]
pub(crate) mod tests;
