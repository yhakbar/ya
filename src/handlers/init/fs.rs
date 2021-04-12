use std::fs;

use std::path::{Path, PathBuf};
use std::fs::create_dir_all;

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn get_path_folder(path: &Path) -> PathBuf {
    path.to_path_buf().parent().unwrap().to_path_buf()
}

pub fn get_path_prefix(path: &Path) -> String {
    path.parent().unwrap().to_str().unwrap().to_string()
}

pub fn create_path_to_file(path: &str) {
    let fs_path = Path::new(&path);
    let prefix = get_path_prefix(&fs_path);
    if !path_exists(&prefix) {
        create_dir_all(&prefix).unwrap();
    }
}
