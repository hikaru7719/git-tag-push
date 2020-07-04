extern crate git_tag_version;
extern crate uuid;
use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn create_local_repository(path: &Path) {
    fs::create_dir(&path).unwrap();
    env::set_current_dir(&path).unwrap();
}

pub fn delete_local_repository(path: &Path) {
    fs::remove_dir(&path).unwrap();
}

pub fn new_config() -> git_tag_version::config::Config {
    git_tag_version::config::Config::new()
}

pub fn create_target_path_str() -> String {
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.to_str().unwrap();
    let uuid_v4 = Uuid::new_v4();
    format!(
        "{}/{}{}",
        current_dir_str,
        "temp_repository",
        uuid_v4.to_string()
    )
}

pub fn get_latest_version() -> String {
    let config = new_config();
    let mut list = git_tag_version::git::version_list(&config).unwrap();
    git_tag_version::version::latest_version(&mut list)
        .unwrap()
        .to_string()
}

#[test]
pub fn test_major_upgrade() {
    let mut config = new_config();
    config.major = true;

    let path_str = create_target_path_str();
    let path = Path::new(&path_str);
    create_local_repository(&path);
    git_tag_version::run::run(config).unwrap();
    assert_eq!(get_latest_version(), String::from("v1.0.0"));
    delete_local_repository(&path);
}

#[test]
pub fn test_minor_upgrade() {
    let mut config = new_config();
    config.minor = true;

    let path_str = create_target_path_str();
    let path = Path::new(&path_str);
    create_local_repository(&path);
    git_tag_version::run::run(config).unwrap();
    assert_eq!(get_latest_version(), String::from("v0.1.0"));
    delete_local_repository(&path);
}

#[test]
pub fn test_patch_upgrade() {
    let mut config = new_config();
    config.patch = true;

    let path_str = create_target_path_str();
    let path = Path::new(&path_str);
    create_local_repository(&path);
    git_tag_version::run::run(config).unwrap();
    assert_eq!(get_latest_version(), String::from("v0.0.1"));
    delete_local_repository(&path);
}
