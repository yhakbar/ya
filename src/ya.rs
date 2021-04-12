use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::handlers::build::BuildConfig;
use crate::handlers::run::RunConfig;
use crate::handlers::shell::ShellConfig;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YaFile {
    name: String,
    config: YaConfig,
    pub deps: Option<Vec<YaDep>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YaConfig {
    pub build: Option<BuildConfig>,
    pub run: Option<RunConfig>,
    pub shell: Option<ShellConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YaDep {
    pub name: Option<String>,
    pub src: PathBuf,
    pub file: PathBuf,
}

pub fn parse_ya_from_file(file: &str) -> Result<YaFile, Box<dyn Error>> {
    let f = File::open(file)?;
    let r = BufReader::new(f);
    let ya: YaFile = serde_yaml::from_reader(r)?;
    Ok(ya)
}

pub fn parse_ya_config_from_file(file: &str) -> Result<YaConfig, Box<dyn Error>> {
    let config = parse_ya_from_file(file)
        .expect("failed to parse ya from file")
        .config;
    Ok(config)
}
