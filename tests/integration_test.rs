extern crate git_tag_version;
use std::env;
use std::fs;
use std::path::Path;

pub fn create_local_repository() {
    let path = Path::new("./repository");
    fs::create_dir(&path).unwrap();
    env::set_current_dir(&path).unwrap();
}

pub fn delete_local_repository() {
    let path = Path::new("./repository");
    fs::remove_dir(&path).unwrap();
}

#[test]
pub fn test_run() {
    create_local_repository();
    delete_local_repository();
}
