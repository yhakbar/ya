use crate::fs::create_if_not_exists;

fn get_default_dockerfile() -> String {
    String::from()
}

fn get_default_config() -> String {
    String::from(
        r###"---
name: ya

config:
  build:
    plugin: shell
    config:
      command: |
      docker build -t ya-builder -f .config/docker/Dockerfile .config/docker


  run:
    plugin: shell
    config:
      command: |
        if [[ "$(docker images -q ya-builder 2> /dev/null)" == "" ]]; then
          ya build
        fi
        docker run -it --rm --entrypoint bash -v $PWD:/app -w /app ya-builder

  shell:
    plugin: shell
    config:
      command: |
        if [[ "$(docker images -q ya-builder 2> /dev/null)" == "" ]]; then
          ya build
        fi
        docker run -it --rm --entrypoint bash -v $PWD:/app -w /app ya-builder
      #
      # Other Examples
      #
      # command: python3
      #
        
"###,
    )
}

pub fn handle_init(config: &str) -> std::io::Result<()> {
    create_if_not_exists(config, get_default_config())
}
