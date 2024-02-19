use super::*;
use crate::imhdlr::utils::*;
use std::fs::{create_dir_all, remove_dir_all, remove_file, File};
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
fn it_imhdlr_crop() {
    let img1 = "tests/images/dir1/annie-spratt-6wd1f4Zjo_0-unsplash-crop-400x400.jpg";
    let img2 = "tests/images/dir1/dir2/alex-plesovskich-MHlxTsw5aKY-unsplash-crop-400x400.jpg";
    if file_exists(img1) {
        remove_file(img1).unwrap();
    }
    if file_exists(img2) {
        remove_file(img2).unwrap();
    }
    imhdlr_crop("tests/images/", 400, 400, true);
    assert!(Path::new(img1).exists(), "File {} does not exist", img1);
    assert!(Path::new(img2).exists(), "File {} does not exist", img2);
}

#[test]
fn it_imhdlr_resize_exact() {
    let img1 = "tests/images/dir1/annie-spratt-6wd1f4Zjo_0-unsplash-400x400.jpg";
    let img2 = "tests/images/dir1/dir2/alex-plesovskich-MHlxTsw5aKY-unsplash-400x400.jpg";
    if file_exists(img1) {
        remove_file(img1).unwrap();
    }
    if file_exists(img2) {
        remove_file(img2).unwrap();
    }
    imhdlr_resize_exact("tests/images/", 400, 400, true);
    assert!(Path::new(img1).exists(), "File {} does not exist", img1);
    assert!(Path::new(img2).exists(), "File {} does not exist", img2);
}

#[test]
fn it_imhdlr_resize_to_fill_crop() {
    let img1 = "tests/images/dir1/annie-spratt-6wd1f4Zjo_0-unsplash-crop-400x400.jpg";
    let img2 = "tests/images/dir1/dir2/alex-plesovskich-MHlxTsw5aKY-unsplash-crop-400x400.jpg";
    if file_exists(img1) {
        remove_file(img1).unwrap();
    }
    if file_exists(img2) {
        remove_file(img2).unwrap();
    }
    imhdlr_resize_to_fill_crop("tests/images/", 400, 400, true);
    assert!(Path::new(img1).exists(), "File {} does not exist", img1);
    assert!(Path::new(img2).exists(), "File {} does not exist", img2);
}

fn create_image(file_path: &str) {
    File::create(file_path).expect("Failed to create file");
}

fn create_dirs(path: &str) {
    let result = create_dir_all(path);
    assert!(
        result.is_ok(),
        "{}",
        format!("Directory '{}' already exists or cannot be created.", path)
    );
}

fn cleanup(dir: &str) {
    let result = remove_dir_all(dir);
    assert!(
        result.is_ok(),
        "{}",
        format!("Directory '{}' does not exist or cannot be deleted.", dir)
    );
}

fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}
