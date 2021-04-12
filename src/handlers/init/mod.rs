mod fs;

use fs::{path_exists, get_path_folder, create_path_to_file};

use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::env;

use crate::ya::parse_ya_from_file;

fn default() -> String {
  include_str!("../../templates/default/ya.yml.hbs").to_string()
}

fn default_docker() -> String {
  include_str!("../../templates/default/docker/Dockerfile.hbs").to_string()
}

fn register_default_templates(handlebars: &mut Handlebars) {
  let default = default();
  let default_docker = default_docker();

  handlebars
    .register_template_string("default", default)
    .unwrap();
  handlebars
    .register_template_string("default_docker", default_docker)
    .unwrap();
}

pub fn handle_init(config: &str, name: &Option<String>) -> std::io::Result<()> {
  let mut handlebars = Handlebars::new();

  register_default_templates(&mut handlebars);

  let mut replacement_keys = HashMap::new();

  let curr_dir = env::current_dir().unwrap();
  let dir_name = curr_dir.file_name().unwrap().to_str().unwrap().to_string();

  let name = match name {
    Some(name) => name,
    None => &dir_name
  };

  replacement_keys.insert("name", name);

  if ! path_exists(&config) {
    let path_prefix = create_path_to_file(&config);

    replacement_keys.insert("workdir", &path_prefix);

    let mut f = File::create(&config).unwrap();
    handlebars.render_to_write("default", &replacement_keys, &mut f).unwrap();
  }

  let ya_file = parse_ya_from_file(&config).expect("failed to parse config file");
  
  match ya_file.deps {
    None => (),
    Some(deps) => {
      for dep in deps {
        let mut replacement_keys = HashMap::new();
        replacement_keys.insert("name", name);

        let src = dep.src.unwrap();
        let file = dep.file.unwrap();

        let config_path = Path::new(&config);
        let config_folder = get_path_folder(&config_path);

        let dep_path = &config_folder.join(&file);
        let dep_config = &dep_path.to_str().unwrap();

        if ! path_exists(&dep_config) {
          create_path_to_file(&dep_config);
          let mut f = File::create(&dep_config).unwrap();
          handlebars.render_to_write(&src, &replacement_keys, &mut f).unwrap();
        }
      }
    }
  }

  Ok(())
}
