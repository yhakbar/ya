use colored::Colorize;
use serde_yaml::Value;
use std::{process::Command, path::PathBuf};

use crate::config::{parse_cmd, resolve_chdir, FullCommand, ParsedConfig, CommandType, parse_config_from_file};

const FROM_RECURSION_LIMIT: u64 = 10;

pub fn run_command_from_config(
    config: &Value,
    command_name: String,
    run_command_flags: &RunCommandFlags,
    extra_args: &[String],
) -> anyhow::Result<()> {
    let command_name = command_name.as_str();
    let cmd = config.get(command_name).ok_or(anyhow::anyhow!(
        "Command {} not found in config",
        command_name
    ))?;
    run_command(config, command_name, cmd, run_command_flags, extra_args)
}


fn get_full_command_from_parsed_command(parsed_command: CommandType, from: Option<String>, command_name: &str, recursion_depth: u64) -> Result<FullCommand, anyhow::Error> {
    match parsed_command {
        CommandType::SimpleCommand(cmd) => Ok(FullCommand {
            prog: "bash".to_string(),
            args: vec!["-c".to_string()],
            cmd: Some(cmd),
        }),
        CommandType::FullCommand(cmd) => Ok(cmd),
        CommandType::None => {
            if let Some(from) = from {
                let from = resolve_chdir(from)?;
                let from_path_buff = PathBuf::from(&from);
                let from_config = parse_config_from_file(from_path_buff.as_path())?;

                let parsed_cmd = from_config.get(command_name);

                if let Some(parsed_cmd) = parsed_cmd {
                    let from_command = parse_cmd(parsed_cmd)?;

                    let ParsedConfig {
                        parsed_command,
                        pre_msg: _,
                        post_msg: _,
                        pre_cmds: _,
                        post_cmds: _,
                        chdir: _,
                        from: _,
                    } = from_command;

                    return match parsed_command {
                        CommandType::SimpleCommand(cmd) => Ok(FullCommand {
                            prog: "bash".to_string(),
                            args: vec!["-c".to_string()],
                            cmd: Some(cmd),
                        }),
                        CommandType::FullCommand(cmd) => Ok(cmd),
                        CommandType::None => {
                            if (recursion_depth) >= FROM_RECURSION_LIMIT {
                                return Err(anyhow::anyhow!(
                                    "Recursion limit of `from` reached: {}",
                                    FROM_RECURSION_LIMIT
                                ))
                            }

                            return get_full_command_from_parsed_command(parsed_command, Some(from), command_name, recursion_depth + 1)
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!(
                        "Command `{}` not found in config specified by `from` field of file {}",
                        command_name,
                        &from,
                    ))
                }
            }
            Err(anyhow::anyhow!("You must provide one of: a string representing a command, a fully qualified command, or a `from` field"))
        }
    }
}

pub struct RunCommandFlags {
    pub sd: Vec<String>,
    pub quiet: bool,
    pub execution: bool,
    pub no_color: bool,
}

fn run_command(
    config: &Value,
    command_name: &str,
    cmd: &Value,
    run_command_flags: &RunCommandFlags,
    extra_args: &[String],
) -> anyhow::Result<()> {
    let command = parse_cmd(cmd)?;

    let ParsedConfig {
        parsed_command,
        pre_msg,
        post_msg,
        pre_cmds,
        post_cmds,
        chdir,
        from,
    } = command;

    let full_command = get_full_command_from_parsed_command(parsed_command, from, command_name, 0)?;

    let FullCommand {
        ref prog,
        ref args,
        ref cmd,
    } = full_command;

    let cmd = cmd.clone();

    if let Some(pre_cmds) = pre_cmds {
        for cmd in pre_cmds {
            run_command_from_config(config, cmd, run_command_flags, &[])?;
        }
    }

    if !run_command_flags.quiet {
        if let Some(msg) = pre_msg {
            println!("{}", msg);
        }
    }

    if run_command_flags.execution {
        let mut parsed_command = format!("$ {}", full_command);
        if !run_command_flags.no_color {
            parsed_command = parsed_command.blue().bold().to_string();
        }
        println!("{}", parsed_command);
    }

    let mut command_builder = Command::new(prog);

    command_builder.args(args);

    if let Some(cmd) = cmd {
        let cmd = run_command_flags.sd.iter().fold(cmd, |cmd, s| {
            let (key, value) = s.split_once('=').unwrap();
            cmd.replace(key, value)
        });
        command_builder.arg(cmd);
    }

    command_builder.args(extra_args);

    if let Some(chdir) = chdir {
        let chdir = resolve_chdir(chdir)?;
        command_builder.current_dir(chdir);
    }

    let result = command_builder.spawn()?.wait()?;

    if !result.success() {
        let msg = format!("Command `{}` failed with status {}", full_command, result);
        return Err(anyhow::anyhow!("{}", msg));
    }

    if !run_command_flags.quiet {
        if let Some(msg) = post_msg {
            println!("{}", msg);
        }
    }

    if let Some(post_cmds) = post_cmds {
        for cmd in post_cmds {
            run_command_from_config(config, cmd, run_command_flags, &[])?;
        }
    }

    Ok(())
}
