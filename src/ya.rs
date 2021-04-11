use serde_derive::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::build::BuildConfig;
use crate::shell::ShellConfig;
use crate::run::RunConfig;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YaFile {
    name: String,
    config: YaConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YaConfig {
    pub build: Option<BuildConfig>,
    pub run: Option<RunConfig>,
    pub shell: Option<ShellConfig>,
}

pub fn parse_ya_from_file(file: &str) -> Result<YaFile, Box<dyn Error>> {
    let f = File::open(file)?;
    let r = BufReader::new(f);
    let ya: YaFile = serde_yaml::from_reader(r)?;
    Ok(ya)
}

pub fn parse_ya_config_from_file(file: &str) -> Result<YaConfig, Box<dyn Error>> {
    let config = parse_ya_from_file(file).expect("failed to parse ya from file").config;
    Ok(config)
}
