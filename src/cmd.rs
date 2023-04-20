use serde_yaml::Value;
use std::{process::Command};
use colored::Colorize;

use crate::config::{parse_cmd, ParsedConfig, ParsedCommand};

pub fn run_command_from_config(
    config: &Value,
    command_name: String,
    sd: &[String],
    quiet: bool,
    execution: bool,
    no_color: bool,
    extra_args: &[String],
) -> anyhow::Result<()> {
    let command_name = command_name.as_str();
    let cmd = config.get(command_name).ok_or(anyhow::anyhow!(
        "Command {} not found in config",
        command_name
    ))?;
    run_command(config, cmd, sd, quiet, execution, no_color, extra_args)
}

fn run_command(
    config: &Value,
    cmd: &Value,
    sd: &[String],
    quiet: bool,
    execution: bool,
    no_color: bool,
    extra_args: &[String],
) -> anyhow::Result<()> {
    let command = parse_cmd(cmd)?;

    let ParsedConfig {
        parsed_command,
        pre_msg,
        post_msg,
        pre_cmds,
        post_cmds,
    } = command;

    let ParsedCommand {
        ref prog,
        ref args,
        ref cmd,
    } = parsed_command;

    let cmd = cmd.clone();

    if let Some(pre_cmds) = pre_cmds {
        for cmd in pre_cmds {
            run_command_from_config(config, cmd, sd, quiet, execution, no_color, &[])?;
        }
    }

    if !quiet {
        if let Some(msg) = pre_msg {
            println!("{}", msg);
        }
    }

    if execution {
        let mut parsed_command = format!("$ {}", parsed_command);
        if ! no_color {
            parsed_command = parsed_command.blue().bold().to_string();
        }
        println!("{}", parsed_command);
    }

    let result = match cmd {
        None => Command::new(prog)
            .args(args)
            .args(extra_args)
            .spawn()?
            .wait()?,
        Some(cmd) => {
            let cmd = sd.iter().fold(cmd, |cmd, s| {
                let (key, value) = s.split_once('=').unwrap();
                cmd.replace(key, value)
            });

            Command::new(prog)
                .args(args)
                .arg(cmd)
                .args(extra_args)
                .spawn()?
                .wait()?
        }
    };

    if !result.success() {
        let msg = format!("Command `{}` failed with status {}", parsed_command, result);
        return Err(anyhow::anyhow!("{}", msg));
    }

    if !quiet {
        if let Some(msg) = post_msg {
            println!("{}", msg);
        }
    }

    if let Some(post_cmds) = post_cmds {
        for cmd in post_cmds {
            run_command_from_config(config, cmd, sd, quiet, execution, no_color, &[])?;
        }
    }

    Ok(())
}
