use glob::glob;
use image::imageops::FilterType;
use image::GenericImageView;
use regex::Regex;
use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;

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

pub fn process_image(
    path: PathBuf,
    resize_width: u32,
    resize_height: u32,
    skip_names_with_dimensions: bool,
    verbose: bool,
) -> io::Result<()> {
    let img = match image::open(&path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Failed to open image {}: {}", path.display(), e);
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to open image"));
        }
    };

    let file_path = match path.into_os_string().into_string() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Failed to read the image path");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to read the image path",
            ));
        }
    };

    let (width, height) = img.dimensions();

    if resize_width <= width && resize_height <= height {
        let resized = img.resize(resize_width, resize_height, FilterType::Triangle);
        let mut validated_file_path = Some(file_path.clone());
        if skip_names_with_dimensions {
            validated_file_path = rename_image(file_path, resize_width, resize_height, verbose);
        }

        if let Some(path) = validated_file_path {
            if let Err(e) = resized.save(&path) {
                eprintln!("Failed to save resized image {}: {}", path, e);
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to save resized image",
                ));
            };
        }
    }

    Ok(())
}

fn rename_image(
    file_path: String,
    resize_width: u32,
    resize_height: u32,
    verbose: bool,
) -> Option<String> {
    let path_split: Vec<&str> = file_path.rsplitn(2, '/').collect();
    let file_name_split: Vec<&str> = path_split[0].rsplitn(2, '.').collect();
    let re = Regex::new("-\\d+x\\d+").unwrap();
    if re.is_match(file_name_split[1]) {
        if verbose {
            println!(
                "File {} already contains resize dimensions, skipping.",
                file_path
            );
        }
        None
    } else {
        let new_filename = format!(
            "{}-{}x{}.{}",
            file_name_split[1], resize_width, resize_height, file_name_split[0]
        );
        let new_path = format!("{}/{}", path_split[1], new_filename);

        Some(new_path)
    }
}
