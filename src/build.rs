use serde_derive::{Serialize, Deserialize};

use crate::configs::docker::build::DockerBuildConfig;
use crate::ya::parse_ya_config_from_file;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "plugin", content = "config")]
pub enum BuildConfig {
    #[serde(rename = "docker")]
    DockerBuildConfig(DockerBuildConfig),
}

pub trait Buildable {
    fn build(&self);
}

impl Buildable for BuildConfig {
    fn build(&self) {
        match self {
            BuildConfig::DockerBuildConfig(buildable) => {
                buildable.build();
            }
        }
    }
}

pub fn handle_build(config: &str) -> std::io::Result<()> {
    let ya_config = parse_ya_config_from_file(&config).expect("failed to parse config file");
    let build_config = ya_config.build.expect("build configuration must be defined when using build command");

    build_config.build();

    Ok(())
}
