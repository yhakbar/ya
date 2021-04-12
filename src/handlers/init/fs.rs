use std::fs;

use std::path::{Path, PathBuf};

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn get_path_folder(path: &Path) -> PathBuf {
    path.to_path_buf().pop();
    path.to_path_buf()
}

// pub fn get_path_prefix(path: &Path) -> String {
//     path.parent().unwrap().to_str().unwrap().to_string()
// }
