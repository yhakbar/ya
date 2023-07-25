use colored::Colorize;
use serde_yaml::Value;
use std::{process::Command, path::PathBuf};

use crate::config::{parse_cmd, resolve_chdir, FullCommand, ParsedConfig, CommandType, parse_config_from_file};

const FROM_RECURSION_LIMIT: u64 = 10;

pub fn run_command_from_config(
    config: &Value,
    command_name: &str,
    run_command_flags: &RunCommandFlags,
    extra_args: &[String],
    recursion_depth: u64,
) -> anyhow::Result<()> {
    let command_name = command_name;
    let cmd = config.get(command_name).ok_or(anyhow::anyhow!(
        "Command {} not found in config",
        command_name
    ))?;
    run_command(config, command_name, cmd, run_command_flags, extra_args, recursion_depth)
}


fn get_full_command_from_parsed_command(parsed_command: CommandType) -> Option<FullCommand> {
    match parsed_command {
        CommandType::SimpleCommand(cmd) => Some(FullCommand {
            prog: "bash".to_string(),
            args: vec!["-c".to_string()],
            cmd: Some(cmd),
        }),
        CommandType::FullCommand(cmd) => Some(cmd),
        CommandType::None => None,
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
    recursion_depth: u64,
) -> anyhow::Result<()> {
    if (recursion_depth) >= FROM_RECURSION_LIMIT {
        return Err(anyhow::anyhow!(
            "Recursive command calls reached: {}",
            FROM_RECURSION_LIMIT
        ))
    }

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

    let full_command_option = get_full_command_from_parsed_command(parsed_command);

    let full_command;

    if let Some(full_command_option) = full_command_option {
        full_command = full_command_option;
    } else if let Some(from) = from {
        let from = resolve_chdir(from)?;
        let from_path_buff = PathBuf::from(&from);
        let from_config = parse_config_from_file(from_path_buff.as_path())?;

        run_command_from_config(&from_config, command_name, run_command_flags, extra_args, recursion_depth + 1)?;
        return Ok(())
    } else {
        return Err(anyhow::anyhow!("You must provide one of: a string representing a command, a fully qualified command, or a `from` field"))

    }

    let FullCommand {
        ref prog,
        ref args,
        ref cmd,
    } = full_command;

    let cmd = cmd.clone();

    if let Some(pre_cmds) = pre_cmds {
        for cmd in pre_cmds {
            run_command_from_config(config, &cmd, run_command_flags, &[], recursion_depth + 1)?;
        }
    }

    if !run_command_flags.quiet {
        if let Some(msg) = pre_msg {
            println!("{}", msg);
        }
    }

    let mut final_args = args.clone().to_vec();

    if let Some(cmd) = cmd {
        let cmd = run_command_flags.sd.iter().fold(cmd, |cmd, s| {
            let (key, value) = s.split_once('=').unwrap();
            cmd.replace(key, value)
        });
        final_args.push(cmd);
    }

    final_args.extend_from_slice(extra_args);

    if run_command_flags.execution {
        let mut parsed_command = format!("$ {} {}", prog, final_args.join(" "));
        if !run_command_flags.no_color {
            parsed_command = parsed_command.blue().bold().to_string();
        }
        println!("{}", parsed_command);
    }

    let mut command_builder = Command::new(prog);

    command_builder.args(final_args);

    if let Some(chdir) = chdir {
        let chdir = resolve_chdir(chdir)?;
        command_builder.current_dir(chdir);
    }

    let result = command_builder.spawn()?.wait()?;

    if !result.success() {
        std::process::exit(result.code().unwrap_or(1));
    }

    if !run_command_flags.quiet {
        if let Some(msg) = post_msg {
            println!("{}", msg);
        }
    }

    if let Some(post_cmds) = post_cmds {
        for cmd in post_cmds {
            run_command_from_config(config, &cmd, run_command_flags, &[], recursion_depth + 1)?;
        }
    }

    Ok(())
}
