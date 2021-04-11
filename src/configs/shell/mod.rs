pub mod run;
pub mod build;
#[allow(clippy::module_inception)]
pub mod shell;

use std::env;
use log::warn;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::{thread, time};

trait Shell {
    fn shell(&self) -> &str;
    fn command(&self) -> &str;
    fn argument_replacement_key(&self) -> &str;

    fn sanitize_shell(_shell: &str) {
        // Not actually sanitizing anything yet. Not sure what to sanitize.
        ();
    }

    fn sanitize_command(_command: &str) {
        // Not actually sanitizing anything yet. Not sure what to sanitize.
        ();
    }

    fn sanitize_shell_command(&self) {
        let shell = self.shell();
        let command = self.command();

        Self::sanitize_shell(shell);
        Self::sanitize_command(command);
    }
}

trait RunShellCommand {
    fn run_shell_command(&self, arguments: &Vec<String>);
}

trait StartInteractiveShell {
    fn start_interactive_shell(&self);
}

impl<T> RunShellCommand for T where T: Shell {
    fn run_shell_command(&self, arguments: &Vec<String>) {
        let shell = self.shell();
        let command = self.command();
        let argument_replacement_key = self.argument_replacement_key();

        self.sanitize_shell_command();

        // This is how we'll avoid infinite recursion
        let recursion_check = "YA_SHELL_COMMAND";

        match env::var(&recursion_check) {
            Ok(parent_command) => {
                if parent_command == command {
                    panic!("Environment variable {}='{}'\nI'm being asked to run '{}'\nInfinite recursion likely.", recursion_check, parent_command, command);
                }
                // Let's wait any time there is a chance of infinite recursion
                warn!("recursive ya call {} from {}", command, parent_command);
                let ten_millis = time::Duration::from_millis(10);
                thread::sleep(ten_millis);
            }
            Err(_e) => (),
        }

        let subbed_command = &command.replace(&argument_replacement_key, &arguments.join(" "));

        let stdout = Command::new(&shell)
                .env(&recursion_check, &command)
                .arg("-c")
                .arg(&subbed_command)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .stdout
                .unwrap();

        // let stdout = match arguments {
        //     Some(arguments) => {
        //         let subbed_command = &command.replace(&argument_replacement_key, &arguments.join(" "));

        //         Command::new(&shell)
        //         .env(&recursion_check, &command)
        //         .arg("-c")
        //         .arg(&subbed_command)
        //         .stdout(Stdio::piped())
        //         .spawn()
        //         .unwrap()
        //         .stdout
        //         .unwrap()
        //     }
        //     None => {
        //         Command::new(&shell)
        //         .env(&recursion_check, &command)
        //         .arg("-c")
        //         .arg(&command)
        //         .stdout(Stdio::piped())
        //         .spawn()
        //         .unwrap()
        //         .stdout
        //         .unwrap()
        //     }
        // };

        let reader = BufReader::new(stdout);

        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line));
    }
}
