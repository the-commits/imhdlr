use ext_php_rs::prelude::*;
use rayon::{iter::ParallelIterator, prelude::IntoParallelIterator};

mod utils;
use crate::imhdlr::utils::{get_images, process_image};

#[php_function]
pub fn imhdlr_get_images(dir: &str) -> Vec<String> {
    let images = get_images(dir);
    images
        .into_iter()
        .flatten()
        .map(|path| path.to_string_lossy().into_owned())
        .collect()
}

#[php_function]
pub fn imhdlr_resize_images(
    dir: &str,
    resize_width: u32,
    resize_height: u32,
    skip_names_with_dimensions: bool,
    verbose: bool,
) {
    for images in get_images(dir) {
        images.into_par_iter().for_each(move |image| {
            if let Err(e) = process_image(
                image,
                resize_width,
                resize_height,
                skip_names_with_dimensions,
                verbose,
            ) {
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
