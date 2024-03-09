use super::*;
use crate::imhdlr::utils::*;
use std::fs::remove_file;
use std::fs::{remove_dir_all, File};
use std::path::Path;

#[test]
fn it_remove_suffix() {
    assert_eq!(remove_suffix("test", '/'), "test");
    assert_eq!(remove_suffix("test/", '/'), "test");
}

#[test]
fn it_imhdlr_get_images() {
    create_dirs("tests_images/dir1/dir2/dir3");
    create_image("tests_images/dir1/dir2/dir3/image.png");
    let images = imhdlr_get("tests_images/");
    assert_eq!(&images[0], "tests_images/dir1/dir2/dir3/image.png");
    cleanup("tests_images");
}

#[test]
fn it_imhdlr_resize_images() {
    let img1 = "tests/images/dir1/annie-spratt-6wd1f4Zjo_0-unsplash-30x30.jpg";
    let img2 = "tests/images/dir1/dir2/alex-plesovskich-MHlxTsw5aKY-unsplash-30x30.jpg";
    remove_file(img1).unwrap();
    remove_file(img2).unwrap();
    imhdlr_squeeze("tests/images/", 30, 30, true, true);
    assert!(Path::new(img1).exists(), "File {} does not exist", img1);
    assert!(Path::new(img2).exists(), "File {} does not exist", img2);
}

fn create_image(file_path: &str) {
    File::create(file_path).expect("Failed to create file");
}

fn cleanup(dir: &str) {
    let result = remove_dir_all(dir);
    assert!(
        result.is_ok(),
        "{}",
        format!("Directory '{}' does not exist or cannot be deleted.", dir)
    );
}
