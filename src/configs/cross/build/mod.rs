use log::warn;
use std::env;

extern crate yaml_rust;
use yaml_rust::yaml::Hash;

use std::process::Command;

#[derive(Debug, Default)]
pub struct CrossBuildConfig {
    targets: Vec<String>,
}

impl CrossBuildConfig {
    pub fn new(docker_build_config: &Hash) -> CrossBuildConfig {
        let mut config = CrossBuildConfig { targets: vec![] };

        for (key, value) in docker_build_config.iter() {
            match key.as_str().unwrap() {
                // "image" => config.image = value.as_str().unwrap().to_string(),
                "targets" => {
                    config.targets = vec![];
                    for volume in value.as_vec().unwrap() {
                        config.targets.push(volume.as_str().unwrap().to_string());
                    }
                }
                _ => warn!("Unsupported key {:?}", key),
            }
        }
        config
    }
}
