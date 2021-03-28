use std::fs;

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn get_default_config() -> String {
    return String::from(
        r###"---
name: ya

config:
  build:
    plugin: docker
    config:
      image: ya-builder:latest
      dockerfile: .config/docker/Dockerfile
      workdir: /app
      volumes:
        - $PWD:/app
"###,
    );
}

pub fn create_if_not_exists(path: &str) -> std::io::Result<()> {
    let fs_path = std::path::Path::new(path);
    let prefix = fs_path.parent().unwrap().to_str().unwrap();
    if !path_exists(&prefix) {
        fs::create_dir_all(&prefix)?;
    }
    if !path_exists(&path) {
        fs::write(path, get_default_config()).expect("Unable to write file");
    }

    Ok(())
}
