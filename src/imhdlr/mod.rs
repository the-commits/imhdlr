use ext_php_rs::prelude::*;
use rayon::{iter::ParallelIterator, prelude::IntoParallelIterator};

mod utils;
use self::utils::process_squeeze_and_crop;
use crate::imhdlr::utils::{get_images, process_crop, process_squeeze};

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
pub fn imhdlr_squeeze(dir: &str, resize_width: u32, resize_height: u32, verbose: bool) {
    for images in get_images(dir) {
        images.into_par_iter().for_each(move |image| {
            if let Err(e) = process_squeeze(image, resize_width, resize_height, verbose) {
                eprintln!("Error processing image: {:?}", e);
            }
        });
    }
}

#[php_function]
pub fn imhdlr_crop(dir: &str, resize_width: u32, resize_height: u32, verbose: bool) {
    for images in get_images(dir) {
        images.into_par_iter().for_each(move |image| {
            process_crop(image, resize_width, resize_height, verbose);
        });
    }
}

#[php_function]
pub fn imhdlr_squeeze_and_crop(dir: &str, resize_width: u32, resize_height: u32, verbose: bool) {
    for images in get_images(dir) {
        images.into_par_iter().for_each(move |image| {
            process_squeeze_and_crop(image, resize_width, resize_height, verbose);
        });
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

#[cfg(test)]
pub(crate) mod tests;
