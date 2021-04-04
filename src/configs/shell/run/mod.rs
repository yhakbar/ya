use serde_derive::{Serialize, Deserialize};

use crate::configs::shell::{Shell, RunShellCommand};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShellRunConfig {
    shell: Option<String>,
    command: String,
}

impl ShellRunConfig {
    pub fn run(&self) {
        self.run_shell_command();
    }
}

impl Shell for ShellRunConfig {
    fn shell(&self) -> &str {
        match &self.shell {
            Some(shell) => &shell,
            None => "bash",
        }
    }
    fn command(&self) -> &str {
        &self.command
    }
}