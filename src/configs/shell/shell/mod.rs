use serde_derive::{Serialize, Deserialize};
use std::process::{Command};

use crate::configs::shell::{StartInteractiveShell};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShellShellConfig {
    shell: Option<String>,
    command: Option<String>,
}

impl ShellShellConfig {
    pub fn shell(&self) {
        self.start_interactive_shell();
    }
}

impl StartInteractiveShell for ShellShellConfig {
    fn start_interactive_shell(&self) {
        let shell = match &self.shell {
            Some(shell) => &shell,
            None => "bash",
        };

        let child = match &self.command {
            Some(command) => {
                Command::new(&shell)
                .arg("-c")
                .arg(&command)
                .spawn()
                .unwrap()
            }
            None => {
                Command::new(&shell)
                .spawn()
                .unwrap()
            }
        };

        child.wait_with_output().unwrap();
    }
}
