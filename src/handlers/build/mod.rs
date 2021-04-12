use serde_derive::{Deserialize, Serialize};

use crate::configs::docker::build::DockerBuildConfig;
use crate::configs::shell::build::ShellBuildConfig;

use crate::ya::parse_ya_config_from_file;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "plugin", content = "config")]
pub enum BuildConfig {
    #[serde(rename = "docker")]
    DockerBuildConfig(DockerBuildConfig),
    #[serde(rename = "shell")]
    ShellBuildConfig(ShellBuildConfig),
}

pub trait Buildable {
    fn build(&self, arguments: &[String], no_arguments: bool);
}

impl Buildable for BuildConfig {
    fn build(&self, arguments: &[String], no_arguments: bool) {
        match self {
            BuildConfig::DockerBuildConfig(buildable) => {
                buildable.build(arguments, no_arguments);
            }
            BuildConfig::ShellBuildConfig(buildable) => {
                buildable.build(arguments, no_arguments);
            }
        }
    }
}

pub fn handle_build(config: &str, arguments: &[String], no_arguments: bool) -> std::io::Result<()> {
    let ya_config = parse_ya_config_from_file(&config).expect("failed to parse config file");
    let build_config = ya_config
        .build
        .expect("build configuration must be defined when using build command");

    build_config.build(arguments, no_arguments);

    Ok(())
}
