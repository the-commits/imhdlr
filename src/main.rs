use std::env;
use std::path::PathBuf;
use glob::{glob, GlobResult};
use image::GenericImageView;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

fn glob_images(dir: &String) -> Vec<PathBuf>{
    let mut images = vec![];
    let file_extensions = vec![".jpg", ".jpeg", ".gif", ".png"];

    for file_extension in file_extensions {
        let pattern = format!("{}{}", dir, file_extension);
        for entry in glob(&pattern).expect("Failed to read directories") {
            match entry {
                Ok(path) => images.push(path),
                Err(e) => println!("{:?}", e),
            }
        }
    }

    return images
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let directory_with_images = &args[1];
    let images = glob_images(directory_with_images);

}