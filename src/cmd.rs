use colored::Colorize;
use serde_yaml::Value;
use std::{path::PathBuf, process::Command};

use crate::config::{
    parse_config_from_file, resolve_chdir, FullCommand, Command as ConfigCommand, CommandInput,
    DEFAULT_PROG, DEFAULT_ARGS, SimpleCommand, SubCommands, FromCommand,
};

const DEFAULT_RECURSION_LIMIT: u64 = 10;

pub fn run_command_from_config(
    config: &Value,
    command_name: &str,
    run_command_flags: &RunCommandFlags,
    extra_args: &[String],
) -> anyhow::Result<()> {
    let command_name = command_name;
    let cmd = config.get(command_name).ok_or(anyhow::anyhow!(
        "Command {} not found in config",
        command_name
    ))?;
    let command = ConfigCommand::try_from(CommandInput{ v: cmd.clone(), command_name: command_name.to_string(), run_command_flags })?;
    run_command(
        &command,
        run_command_flags,
        extra_args,
    )
}

pub struct RunCommandFlags {
    pub execution: bool,
    pub quiet: bool,
    pub config: PathBuf,
}

trait Printable {
    fn print_execution(&self, extra_args: &[String]);
}

trait Runnable {
    fn run(&self, run_command_flags: &RunCommandFlags, extra_args: &[String]) -> Result<(), anyhow::Error>;
}

impl Printable for SimpleCommand {
    fn print_execution(&self, extra_args: &[String]) {
        let mut parsed_command = format!("$ bash -c '{}'", self);
        for arg in extra_args {
            parsed_command.push_str(&format!(" {}", arg));
        }
        parsed_command = parsed_command.blue().bold().to_string();
        println!("{}", parsed_command);
    }
}

impl Runnable for SimpleCommand {
    fn run(&self, run_command_flags: &RunCommandFlags, extra_args: &[String]) -> Result<(), anyhow::Error> {
        if run_command_flags.execution {
            self.print_execution(extra_args);
        }

        let prog = DEFAULT_PROG.to_string();

        let result = Command::new(prog)
            .env("YA_CONFIG", run_command_flags.config.to_str().unwrap_or(""))
            .args(DEFAULT_ARGS.to_vec())
            .arg(self)
            .args(extra_args)
            .spawn()?.wait()?;

        if !result.success() {
            std::process::exit(result.code().unwrap_or(1));
        }
        Ok(())
    }
}

impl Printable for FullCommand {
    fn print_execution(&self, extra_args: &[String]) {
        let mut parsed_command = format!("$ {} {}", self.prog, self.args.join(" "));
        if let Some(cmd) = &self.cmd {
            parsed_command.push_str(&format!(" '{}'", cmd));
        }
        for arg in extra_args {
            parsed_command.push_str(&format!(" {}", arg));
        }
        parsed_command = parsed_command.blue().bold().to_string();
        println!("{}", parsed_command);
    }
}

impl Runnable for FullCommand {
    fn run(&self, run_command_flags: &RunCommandFlags, extra_args: &[String]) -> Result<(), anyhow::Error> {
        if run_command_flags.execution {
            self.print_execution(extra_args);
        }

        let prog = &self.prog.clone();
        let cmd = &self.cmd;
        let chdir = &self.chdir;

        let mut command = Command::new(prog);

        command
            .env("YA_CONFIG", run_command_flags.config.to_str().unwrap_or(""))
            .args(self.args.as_slice());

        if let Some(cmd) = cmd {
            command.arg(cmd);
        }

        command.args(extra_args);

        if let Some(chdir) = chdir {
            let chdir = resolve_chdir(chdir.to_string())?;
            command.current_dir(chdir);
        }

        let result = command.spawn()?.wait()?;

        if !result.success() {
            std::process::exit(result.code().unwrap_or(1));
        }

        Ok(())
    }
}

impl Runnable for SubCommands {
    fn run(&self, run_command_flags: &RunCommandFlags, extra_args: &[String]) -> Result<(), anyhow::Error> {
        let subcommand_name = extra_args
            .get(0)
            .ok_or(anyhow::anyhow!("No subcommand provided"))?;
        let SubCommands(subcommands) = self;
        let subcommand = subcommands
            .get(subcommand_name)
            .ok_or(anyhow::anyhow!(
                "Subcommand {} not found in config",
                subcommand_name
            ))?;
        subcommand.run(run_command_flags, &extra_args[1..])
    }
}

impl Runnable for FromCommand {
    fn run(&self, run_command_flags: &RunCommandFlags, extra_args: &[String]) -> Result<(), anyhow::Error> {
        let from = &self.from;
        let from = resolve_chdir(from.to_string())?;
        let from_path_buff = PathBuf::from(&from);
        let from_config = parse_config_from_file(from_path_buff.as_path())?;
        let command_name = &self.cmd;



        let recursion_depth = std::env::var("__YA_RECURSION_DEPTH").unwrap_or("0".to_string()).parse::<u64>().unwrap_or(0);
        std::env::set_var("__YA_RECURSION_DEPTH", (recursion_depth + 1).to_string());

        std::env::set_var("YA_CONFIG", from_path_buff.to_str().unwrap_or(""));

        run_command_from_config(
            &from_config,
            command_name,
            run_command_flags,
            extra_args,
        )?;
        Ok(())
    }
}

impl Runnable for ConfigCommand {
    fn run(&self, run_command_flags: &RunCommandFlags, extra_args: &[String]) -> Result<(), anyhow::Error> {
        match self {
            ConfigCommand::SimpleCommand(simple_cmd) => simple_cmd.run(run_command_flags, extra_args),
            ConfigCommand::FullCommand(full_cmd) => full_cmd.run(run_command_flags, extra_args),
            ConfigCommand::SubCommands(sub_cmds) => sub_cmds.run(run_command_flags, extra_args),
            ConfigCommand::FromCommand(from_cmd) => from_cmd.run(run_command_flags, extra_args),
        }
    }
}

fn run_command(
    command: &ConfigCommand,
    run_command_flags: &RunCommandFlags,
    extra_args: &[String],
) -> anyhow::Result<()> {
    let recursion_depth = std::env::var("__YA_RECURSION_DEPTH").unwrap_or("0".to_string()).parse::<u64>().unwrap_or(0);
    if (recursion_depth) >= DEFAULT_RECURSION_LIMIT {
        return Err(anyhow::anyhow!(
            "Recursion limit reached: {}",
            DEFAULT_RECURSION_LIMIT
        ));
    }

    command.run(run_command_flags, extra_args)
}
