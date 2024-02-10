#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use glob::glob;
use std::path::PathBuf;
use std::str::FromStr;

const FILE_EXTENSIONS: [&str; 10] = [
    ".jpg",
    ".jpeg",
    ".gif",
    ".png",
    ".avif",
    ".bmp",
    ".ico",
    ".tga",
    ".tiff",
    ".webp",
];

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

fn remove_suffix(s: &str, c: char) -> &str {
    if !s.ends_with(c) {
        return s;
    }

    s.strip_suffix(c).unwrap()
}

fn load_images(pattern: &str) -> Vec<PathBuf> {
    glob(pattern)
        .expect("Failed to read directories")
        .filter_map(|x| x.ok())
        .collect()
}

fn glob_images(dir: String) -> Vec<Vec<PathBuf>> {
    let mut directories = Vec::new();
    for &file_extension in FILE_EXTENSIONS.iter() {
        let pattern = format!("{}{}", dir, file_extension);
        let images = load_images(&pattern);
        if !images.is_empty() {
            directories.push(images);
        }
    }
    directories
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_dir_all, create_dir_all, File};

    #[test]
    fn it_remove_suffix() {
        assert_eq!(remove_suffix("test", '/'), "test");
        assert_eq!(remove_suffix("test/", '/'), "test");
    }

    #[test]
    fn it_imhdlr_get_images() {
        create_dirs("tests_images/dir1/dir2/dir3");
        create_image("tests_images/dir1/dir2/dir3/image.png");
        let images = imhdlr_get_images("tests_images/");
        assert_eq!(&images[0], "tests_images/dir1/dir2/dir3/image.png");
        cleanup("tests_images");
    }

    fn create_dirs(dir: &str) {
        match create_dir_all(dir) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
        }
    }

    fn create_image(file_path: &str) {
        File::create(file_path).expect("Failed to create file");
    }

    fn cleanup(dir: &str) {
        let result = remove_dir_all(dir);
        assert!(result.is_ok(),"{}", format!("Directory '{}' does not exist or cannot be deleted.", dir));
    }
}