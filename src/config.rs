use std::fs;

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn create_if_not_exists(path: &str) -> std::io::Result<()> {
    if !path_exists(&path) {
        fs::create_dir_all(&path)?;
    }
    Ok(())
}
