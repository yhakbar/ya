extern crate yaml_rust;
use yaml_rust::yaml::Hash;

use crate::configs::DockerBuildConfig;
use crate::yml::get_yml_from_file;

#[derive(Debug)]
pub struct Config {
    name: String,
    config: Hash,
    // Special configs
    // docker_build_config: DockerBuildConfig,
}

pub fn new_from_yml(doc: &yaml_rust::Yaml) -> Config {
    let mut config = Config {
        name: String::from(""),
        config: Hash::new(),
    };

    // TODO - Optimize
    if !doc["name"].is_badvalue() {
        config.name = doc["name"].as_str().unwrap().to_string();
    }
    if !doc["config"].is_badvalue() {
        let yml_config = &doc["config"];

        config.config = yml_config.as_hash().unwrap().clone();

        if !yml_config["build"].is_badvalue() {
            let build = &yml_config["build"];

            if !build["plugin"].is_badvalue() && build["plugin"].as_str().unwrap() == "docker" {
                let docker_build_config =
                    DockerBuildConfig::new(&build["config"].as_hash().unwrap());
                docker_build_config.build();
            }
        }
    }
    config
}

pub fn new_from_path(path: &str) -> Config {
    let yml = get_yml_from_file(path);
    return new_from_yml(&yml);
}
