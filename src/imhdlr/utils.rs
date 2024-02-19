use glob::glob;
use image::io::Reader as ImageReader;
use image::{imageops, GenericImageView};
use std::io;
use std::path::PathBuf;

const FILE_EXTENSIONS: [&str; 10] = [
    ".jpg", ".jpeg", ".gif", ".png", ".avif", ".bmp", ".ico", ".tga", ".tiff", ".webp",
];

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

pub fn process_resize_exact(
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

pub fn process_crop(
    path: PathBuf,
    resize_width: u32,
    resize_height: u32,
    verbose: bool,
) -> io::Result<()> {
    if let Ok(reader) = ImageReader::open(&path) {
        if let Ok(mut image) = reader.decode() {
            let (width, height) = image.dimensions();
            let subimg = imageops::crop(
                &mut image,
                (width - resize_width) / 2,
                (height - resize_height) / 2,
                resize_width.min(width),
                resize_height.min(height),
            ); // resize_width, resize_height);
            let output_path = path.with_file_name(rename_image(
                path.file_name().unwrap().to_str().unwrap(),
                resize_width,
                resize_height,
            ));
            match subimg.to_image().save(output_path.clone()) {
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

pub fn process_resize_to_fill_crop(
    image_path: PathBuf,
    new_width: u32,
    new_height: u32,
    verbose: bool,
) -> io::Result<()> {
    if let Ok(reader) = ImageReader::open(&image_path) {
        if let Ok(image) = reader.decode() {
            let mut resized_image =
                image.resize_to_fill(new_width, new_height, image::imageops::FilterType::Lanczos3);
            let crop_width = new_width.min(new_width);
            let crop_height = new_height.min(new_height);
            let left = (new_width - crop_width) / 2;
            let top = (new_height - crop_height) / 2;
            let cropped_image = resized_image.crop(left, top, crop_width, crop_height);
            let output_path = image_path.with_file_name(rename_image(
                image_path.file_name().unwrap().to_str().unwrap(),
                new_width,
                new_height,
            ));
            match cropped_image.save(output_path.clone()) {
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

fn rename_image(file_path: &str, resize_width: u32, resize_height: u32) -> String {
    let path_split: Vec<&str> = file_path.rsplitn(2, '/').collect();
    let file_name_split: Vec<&str> = path_split[0].rsplitn(2, '.').collect();
    let new_filename = format!(
        "{}-{}x{}.{}",
        file_name_split[1], resize_width, resize_height, file_name_split[0]
    );

    new_filename
}
