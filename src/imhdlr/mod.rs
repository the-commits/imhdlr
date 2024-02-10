use ext_php_rs::prelude::*;

mod utils;
use utils::{glob_images, remove_suffix};

#[php_function]
pub fn imhdlr_get_images(dir: &str) -> Vec<String> {
    let directory_with_images = remove_suffix(dir, '/').to_owned() + "/**/*";
    let images = glob_images(directory_with_images);
    images.into_iter().flatten().map(|path| path.to_string_lossy().into_owned()).collect()
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

#[cfg(test)]
pub(crate) mod tests;