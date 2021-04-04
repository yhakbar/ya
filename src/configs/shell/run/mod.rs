use serde_derive::{Serialize, Deserialize};

use std::process::Command;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShellRunConfig {
    shell: Option<String>,
    command: String,
}

impl ShellRunConfig {
    fn run_shell_command(&self) {
        let shell = match &self.shell {
            Some(shell) => shell,
            None => "bash",
        };
        let command = &self.command;
        let build_output = Command::new(shell)
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to run command");
        println!("{}", String::from_utf8_lossy(&build_output.stdout));
        println!("{}", String::from_utf8_lossy(&build_output.stderr));
    }

    pub fn run(&self) {
        self.run_shell_command();
    }
}
