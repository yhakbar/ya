mod fs;

use fs::{path_exists, get_path_folder};

use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;

use crate::ya::parse_ya_from_file;

fn default_ya() -> String {
  include_str!("../../templates/default/ya.yml.hbs").to_string()
}

fn default_docker() -> String {
  include_str!("../../templates/default/docker/Dockerfile.hbs").to_string()
}

fn register_default_templates(handlebars: &mut Handlebars) {
  let default_ya = default_ya();
  let default_docker = default_docker();

  handlebars
    .register_template_string("default_ya", default_ya)
    .unwrap();
  handlebars
    .register_template_string("default_docker", default_docker)
    .unwrap();
}

pub fn handle_init(config: &str) -> std::io::Result<()> {
  let mut handlebars = Handlebars::new();

  register_default_templates(&mut handlebars);

  let mut data = HashMap::new();
  data.insert("name", "ya");

  if ! path_exists(config) {
    let mut f = File::create(&config).unwrap();
    handlebars.render_to_write("default_ya", &data, &mut f).unwrap();
  }

  let ya_file = parse_ya_from_file(&config).expect("failed to parse config file");
  
  match ya_file.deps {
    None => (),
    Some(deps) => {
      for dep in deps {
        let src = dep.src.to_str().unwrap();
        let file = dep.file;

        let config_path = Path::new(&config);
        let config_folder = get_path_folder(&config_path);
        let dep_path = &config_folder.join(&file);
        let dep_config = &dep_path.to_str().unwrap();

        if ! path_exists(&dep_config) {
          let mut f = File::create(&dep_config).unwrap();

          handlebars.render_to_write(&src, &data, &mut f).unwrap();
        }
      }
    }
  }

  Ok(())
}
