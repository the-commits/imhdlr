mod image_processing;

#[deny(unsafe_code)]
use std::env;
use std::path::PathBuf;
use std::process::exit;
use glob::glob;
use image::GenericImageView;
use crate::image_processing::process_image;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("USAGE: imhdlr <directory_with_images/**/*> <resize_width> <resize_height>");
        return;
    }
    let directory_with_images = &args[1];
    let resize_width = &args[2];
    let resize_height = &args[3];

    let directories = glob_images(directory_with_images);

    let mut thread_handles = vec![];

    for images in directories {
        let thread_resize_width = resize_width
            .parse()
            .expect("Not a valid number for resize width");
        let thread_resize_height = resize_height
            .parse()
            .expect("Not a valid number for resize height");
        let handle = std::thread::spawn(move || {
            for image in images {
                process_image(image, thread_resize_width, thread_resize_height);
            }
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    exit(0);
}

fn glob_images(dir: &String) -> Vec<Vec<PathBuf>> {
    let mut directories = vec![];
    let file_extensions = vec![".jpg", ".jpeg", ".gif", ".png"];

    for file_extension in file_extensions {
        let pattern = format!("{}{}", dir, file_extension);
        let mut images = vec![];
        for entry in glob(&pattern).expect("Failed to read directories") {
            match entry {
                Ok(path) => images.push(path),
                Err(e) => println!("{:?}", e),
            }
        }
        if !images.is_empty() {
            directories.push(images);
        }
    }

    directories
}