use serde_derive::{Serialize, Deserialize};

use crate::configs::shell::{Shell, RunShellCommand};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShellRunConfig {
    shell: Option<String>,
    command: String,
    argument_replacement_key: Option<String>,
}

impl ShellRunConfig {
    pub fn run(&self, arguments: &Vec<String>) {
        self.run_shell_command(arguments);
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
    fn argument_replacement_key(&self) -> &str {
        match &self.argument_replacement_key {
            Some(argument_replacement_key) => &argument_replacement_key,
            None => "$@",
        }
    }
}
