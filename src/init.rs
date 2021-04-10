use crate::fs::create_if_not_exists;

fn get_default_config() -> String {
    String::from(
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
"###
    )
}

pub fn handle_init(config: &str) -> std::io::Result<()> {
    create_if_not_exists(config, get_default_config())
}
