use std::fs;

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn create_if_not_exists(path: &str, file_text: String) -> std::io::Result<()> {
    let fs_path = std::path::Path::new(path);
    let prefix = fs_path.parent().unwrap().to_str().unwrap();
    if !path_exists(&prefix) {
        fs::create_dir_all(&prefix)?;
    }
    if !path_exists(&path) {
        fs::write(path, file_text).expect("Unable to write file");
    }

    Ok(())
}
