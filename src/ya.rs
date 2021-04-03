use serde_derive::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::configs::docker::build::DockerBuildConfig;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YaFile {
    name: String,
    config: YaConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YaConfig {
    pub build: Option<BuildConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildConfig {
    pub plugin: String,
    pub config: BuildConfigs,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildConfigs {
    DockerBuildConfig(DockerBuildConfig),
}

pub fn parse_ya_from_file(file: &str) -> Result<YaFile, Box<dyn Error>> {
    let f = File::open(file)?;
    let r = BufReader::new(f);
    let ya: YaFile = serde_yaml::from_reader(r)?;
    Ok(ya)
}

pub fn parse_ya_config_from_file(file: &str) -> Result<YaConfig, Box<dyn Error>> {
    let config = parse_ya_from_file(file).expect("Failed to parse ya from file").config;
    Ok(config)
}
