use glob::glob;
use std::path::PathBuf;

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