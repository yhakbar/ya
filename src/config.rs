extern crate yaml_rust;
use yaml_rust::yaml::{Hash, Yaml};

use crate::yml::get_yml_from_file;

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub config: Hash,
    // Special configs
    // docker_build_config: DockerBuildConfig,
}

pub fn new_from_yml(doc: &yaml_rust::Yaml) -> Config {
    let mut config = Config {
        name: String::from(""),
        config: Hash::new(),
    };
    let doc_hash = doc.as_hash().unwrap();

    match doc_hash.get(&Yaml::String("name".to_string())) {
        Some(name) => {
            config.name = name.as_str().unwrap().to_string();
        }
        None => {
            panic!("Name not found in file config");
        }
    }

    match doc_hash.get(&Yaml::String("config".to_string())) {
        Some(ya_config) => {
            config.config = ya_config.as_hash().unwrap().clone();
        }
        None => {
            panic!("Config not found in file config");
        }
    }

    config
}

pub fn new_from_path(path: &str) -> Config {
    let yml = get_yml_from_file(path);
    return new_from_yml(&yml);
}
