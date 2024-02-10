use image::{self, GenericImageView};
use regex::Regex;
use std::path::PathBuf;
use image::imageops::FilterType;

pub fn process_image(path: PathBuf, resize_width: u32, resize_height: u32) {
    let img = image::open(path.clone()).unwrap();
    let file_path = path.into_os_string().into_string().unwrap();
    let (width, height) = img.dimensions();

    if resize_width <= width && resize_height <= height {
        let resized = img.resize(resize_width, resize_height, FilterType::Triangle);
        let renamed = rename_image(file_path, resize_width, resize_height);
        match renamed {
            Some(path) => {
                dbg!(path.clone());
                resized.save(path).unwrap();
            }
            None => (),
        }
    }
}

fn rename_image(file_path: String, resize_width: u32, resize_height: u32) -> Option<String> {
    let path_split: Vec<&str> = file_path.rsplitn(2, '/').collect();
    let file_name_split: Vec<&str> = path_split[0].rsplitn(2, '.').collect();
    let re = Regex::new("-\\d+x\\d+").unwrap();
    if re.is_match(file_name_split[1]) {
        println!("File {} already contains resize dimensions, skipping.", file_path);
        None
    } else {
        let new_filename = format!("{}-{}x{}.{}", file_name_split[1], resize_width, resize_height, file_name_split[0]);
        let new_path = format!("{}/{}", path_split[1], new_filename);

        Some(new_path)
    }
}