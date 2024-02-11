use ext_php_rs::props::Prop;
use fast_image_resize as fr;
use fast_image_resize::{DifferentTypesOfPixelsError, DynamicImageView, Image};
use glob::glob;
use image::io::Reader as ImageReader;
use image::{imageops, GenericImageView, ImageBuffer, ImageResult, Rgba};
use regex::Regex;
use std::fs::{create_dir_all, rename};
use std::io;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::process::exit;

const FILE_EXTENSIONS: [&str; 10] = [
    ".jpg", ".jpeg", ".gif", ".png", ".avif", ".bmp", ".ico", ".tga", ".tiff", ".webp",
];

pub fn create_dirs(dir: &str) {
    match create_dir_all(dir) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }
}

pub fn get_images(dir: &str) -> Vec<Vec<PathBuf>> {
    glob_images(remove_suffix(dir, '/').to_owned() + "/**/*")
}

pub fn remove_suffix(s: &str, c: char) -> &str {
    if !s.ends_with(c) {
        return s;
    }
    s.strip_suffix(c).unwrap()
}

pub fn load_images(pattern: &str) -> Vec<PathBuf> {
    glob(pattern)
        .expect("Failed to read directories")
        .filter_map(|x| x.ok())
        .collect()
}

pub fn glob_images(dir: String) -> Vec<Vec<PathBuf>> {
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

pub fn process_squeeze(
    path: PathBuf,
    resize_width: u32,
    resize_height: u32,
    verbose: bool,
) -> io::Result<()> {
    if let Ok(image) = ImageReader::open(&path) {
        if let Ok(image) = image.decode() {
            let resized_image = image.resize_exact(
                resize_width,
                resize_height,
                image::imageops::FilterType::Lanczos3,
            );
            let output_path = path.with_file_name(rename_image(
                path.file_name().unwrap().to_str().unwrap(),
                resize_width,
                resize_height,
            ));
            match resized_image.save(output_path.clone()) {
                Ok(()) => {
                    if verbose {
                        println!("{} saved", output_path.display());
                    }
                }
                Err(e) => {
                    eprintln!("An error occurred when saving. {}", e);
                }
            }
        }
    }
    Ok(())
}

pub fn process_crop(path: PathBuf, resize_width: u32, resize_height: u32) {
    if let Ok(reader) = ImageReader::open(&path) {
        if let Ok(mut image) = reader.decode() {
            let subimg = imageops::crop(&mut image, 0, 0, resize_width, resize_height);
            let output_path = path.with_file_name(rename_image(
                path.file_name().unwrap().to_str().unwrap(),
                resize_width,
                resize_height,
            ));
            subimg.to_image().save(output_path).unwrap();
        }
    }
}

fn rename_image(file_path: &str, resize_width: u32, resize_height: u32) -> String {
    let path_split: Vec<&str> = file_path.rsplitn(2, '/').collect();
    let file_name_split: Vec<&str> = path_split[0].rsplitn(2, '.').collect();
    let new_filename = format!(
        "{}-{}x{}.{}",
        file_name_split[1], resize_width, resize_height, file_name_split[0]
    );

    new_filename
}
