use serde_derive::{Deserialize, Serialize};

use crate::configs::shell::shell::ShellShellConfig;

use crate::ya::parse_ya_config_from_file;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "plugin", content = "config")]
pub enum ShellConfig {
    #[serde(rename = "shell")]
    ShellShellConfig(ShellShellConfig),
}

pub trait Shellable {
    fn shell(&self);
}

impl Shellable for ShellConfig {
    fn shell(&self) {
        match self {
            ShellConfig::ShellShellConfig(shellable) => {
                shellable.shell();
            }
        }
    }
}

pub fn handle_shell(config: &str) -> std::io::Result<()> {
    let ya_config = parse_ya_config_from_file(&config).expect("failed to parse config file");
    let shell_config = ya_config
        .shell
        .expect("shell configuration must be defined when using shell command");

    shell_config.shell();

    Ok(())
}
