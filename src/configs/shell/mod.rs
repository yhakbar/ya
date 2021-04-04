pub mod run;
pub mod build;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

trait Shell {
    fn shell(&self) -> &str;
    fn command(&self) -> &str;
}

trait RunShellCommand {
    fn run_shell_command(&self);
}

impl<T> RunShellCommand for T where T: Shell {
    fn run_shell_command(&self) {
        let shell = self.shell();
        let command = self.command();

        let stdout = Command::new(shell)
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .stdout
            .unwrap();

        let reader = BufReader::new(stdout);

        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line));
    }
}
