use serde_yaml::Value;

use std::process::Command;

use crate::config::{parse_cmd, ParsedCommand};

pub fn run_command_from_config(
    config: &Value,
    command_name: String,
    sd: &[String],
    quiet: bool,
    extra_args: &[String],
) -> anyhow::Result<()> {
    let command_name = command_name.as_str();
    let cmd = config.get(command_name).ok_or(anyhow::anyhow!(
        "Command {} not found in config",
        command_name
    ))?;
    run_command(config, cmd, sd, quiet, extra_args)
}

fn run_command(config: &Value, cmd: &Value, sd: &[String], quiet: bool, extra_args: &[String]) -> anyhow::Result<()> {
    let command = parse_cmd(cmd)?;

    let ParsedCommand {
        prog,
        args,
        cmd,
        pre_msg,
        post_msg,
        pre_cmds,
        post_cmds,
    } = command;

    if let Some(pre_cmds) = pre_cmds {
        for cmd in pre_cmds {
            run_command_from_config(config, cmd, sd, quiet, &[])?;
        }
    }

    let cmd = sd.iter().fold(cmd, |cmd, s| {
        let (key, value) = s.split_once('=').unwrap();
        cmd.replace(key, value)
    });

    if !quiet {
        if let Some(msg) = pre_msg {
            println!("{}", msg);
        }
    }

    Command::new(prog).args(args).arg(cmd).args(extra_args).spawn()?.wait()?;

    if !quiet {
        if let Some(msg) = post_msg {
            println!("{}", msg);
        }
    }

    if let Some(post_cmds) = post_cmds {
        for cmd in post_cmds {
            run_command_from_config(config, cmd, sd, quiet, &[])?;
        }
    }

    Ok(())
}
