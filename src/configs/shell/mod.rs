pub mod run;
pub mod build;
#[allow(clippy::module_inception)]
pub mod shell;

use log::warn;
use std::env;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

trait Shell {
    fn shell(&self) -> &str;
    fn command(&self) -> &str;

    fn sanitize_shell(shell: &str) {
        // This insufficiently sanitizes the command being run
        if shell.contains("ya") {
            // This is a bad solution, but I don't know how to make a better one.
            // There should be some sort of catch that the command being run
            // within one of these commands isn't a ya command, as that can infinitely
            // recur.
            //
            warn!("Warning! we've got a ya in here");
        }
    }

    fn sanitize_command(command: &str) {
        // This insufficiently sanitizes the command being run
        if command.contains("ya") {
            // This is a bad solution, but I don't know how to make a better one.
            //
            // There should be some sort of catch that the command being run
            // within one of these commands isn't a ya command, as that can infinitely
            // recur.
            //
            warn!("Warning! we've got a ya in here");
        }
    }

    fn sanitize_shell_command(&self) {
        let shell = self.shell();
        let command = self.command();

        Self::sanitize_shell(shell);
        Self::sanitize_command(command);
    }
}

trait RunShellCommand {
    fn run_shell_command(&self);
}

trait StartInteractiveShell {
    fn start_interactive_shell(&self);
}

impl<T> RunShellCommand for T where T: Shell {
    fn run_shell_command(&self) {
        let shell = self.shell();
        let command = self.command();

        self.sanitize_shell_command();

        // This is how we'll avoid infinite recursion
        let recursion_check = "YA_SHELL_COMMAND";

        match env::var(&recursion_check) {
            Ok(_val) => panic!("environment variable {} detected. ya commands cannot be run within ya commands, as that might result in infinite recursion", recursion_check),
            Err(_e) => {
                let stdout = Command::new(&shell)
                .env(&recursion_check, "true")
                .arg("-c")
                .arg(&command)
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
            },
        }
    }
}
