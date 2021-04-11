use serde_derive::{Serialize, Deserialize};

use crate::configs::shell::run::ShellRunConfig;
use crate::ya::parse_ya_config_from_file;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "plugin", content = "config")]
pub enum RunConfig {
    #[serde(rename = "shell")]
    ShellRunConfig(ShellRunConfig),
}

pub trait Runnable {
    fn run(&self, argument: &Option<String>);
}

impl Runnable for RunConfig {
    fn run(&self, argument: &Option<String>) {
        match self {
            RunConfig::ShellRunConfig(runnable) => {
                runnable.run(argument);
            }
        }
    }
}

pub fn handle_run(config: &str, argument: &Option<String>) -> std::io::Result<()> {
    let ya_config = parse_ya_config_from_file(&config).expect("failed to parse config file");
    let run_config = ya_config.run.expect("run configuration must be defined when using run command");

    run_config.run(argument);

    Ok(())
}
