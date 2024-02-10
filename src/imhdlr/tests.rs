use super::*;
use std::fs::{create_dir_all, remove_dir_all, File};
use crate::imhdlr::utils::*;

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
        Ok(_) => {}
    }
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
