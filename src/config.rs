use colored::Colorize;
use serde_yaml::Mapping;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use crate::cmd::RunCommandFlags;
use crate::git::get_git_root;

const DEFAULT_CONFIG_NAMES: [&str; 2] = ["ya.yml", "ya.yaml"];
const DEFAULT_CONFIG_LOCATIONS: [&str; 3] = [".", ".config", "$GIT_ROOT/.config"];

pub const DEFAULT_PROG: &str = "bash";
pub const DEFAULT_ARGS: &[&str] = &["-c"];

pub fn get_config_path(path: &Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let path = match path {
        Some(path) => path,
        None => {
            for config_location in DEFAULT_CONFIG_LOCATIONS.iter() {
                let config_location = if config_location.starts_with("$GIT_ROOT") {
                    let git_root = get_git_root().unwrap_or(".".to_string());
                    config_location.replace("$GIT_ROOT", &git_root)
                } else {
                    config_location.to_string()
                };
                for config_name in DEFAULT_CONFIG_NAMES.iter() {
                    let path = Path::new(&config_location).join(config_name);
                    if path.exists() && path.is_file() {
                        return Ok(path.to_path_buf());
                    }
                }
            }

            return Err(anyhow::anyhow!(
                "Could not find config file in default locations. Please specify a config file."
            ));
        }
    };
    Ok(path.to_path_buf())
}

pub fn resolve_chdir(chdir: String) -> anyhow::Result<String> {
    if chdir.starts_with("$GIT_ROOT") {
        let git_root = get_git_root()?;
        let git_root = git_root.trim();
        let chdir = chdir.replace("$GIT_ROOT", git_root);
        return Ok(chdir);
    }

    Ok(chdir)
}

pub fn parse_config_from_file(path: &Path) -> anyhow::Result<Value> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let ya = serde_yaml::from_reader(r)?;
    Ok(ya)
}

pub fn print_config_from_file(path: &Path) -> anyhow::Result<()> {
    let config = parse_config_from_file(path)?;
    println!("---\n{}---\n", serde_yaml::to_string(&config)?);
    Ok(())
}

pub trait Validatable {
    fn validate(&self, m: Mapping, run_command_flags: &RunCommandFlags);
}

pub type SimpleCommand = String;

pub struct FullCommand {
    pub prog: String,
    pub args: Vec<String>,
    pub cmd: Option<String>,
    pub chdir: Option<String>,
}

impl Validatable for FullCommand {
    fn validate(&self, m: Mapping, run_command_flags: &RunCommandFlags) {
        if ! run_command_flags.quiet {
            let keys = m.keys().filter(|k| {
                let k = k.as_str().unwrap_or("");
                k != "prog" && k != "args" && k != "cmd" && k != "chdir"
            }).collect::<Vec<_>>();

            if ! keys.is_empty() {
                let keys = keys.iter().map(|k| k.as_str().unwrap_or("")).collect::<Vec<_>>().join(", ").red();
                eprintln!("{}: Ignoring invalid keys for a full command: {}", "Warning".bright_yellow().bold(), keys);
            }
        }
    }
}

impl TryFrom<serde_yaml::Mapping> for FullCommand {
    type Error = anyhow::Error;

    fn try_from(m: serde_yaml::Mapping) -> anyhow::Result<FullCommand> {
        let mapping_prog = m.get("prog");
        let mapping_args = m.get("args");
        let mapping_cmd = m.get("cmd");
        let mapping_chdir = m.get("chdir");

        let prog = match mapping_prog {
            Some(prog) =>
                prog.as_str()
                    .ok_or(anyhow::anyhow!("Invalid FullCommand: `prog` is not a string"))?
                    .to_string(),
            None => DEFAULT_PROG.to_string(),
        };

        let args = match mapping_args {
            Some(args) => args
                .as_sequence()
                .ok_or(anyhow::anyhow!("Invalid FullCommand: `args` is not a sequence"))?
                .iter()
                .map(|a| {
                    a.as_str()
                        .ok_or(anyhow::anyhow!(
                            "Invalid FullCommand: `arg` is not a string: {:?}",
                            a
                        ))
                        .map(|s| s.to_string())
                })
                .collect::<anyhow::Result<Vec<String>>>()?,
            None => DEFAULT_ARGS.iter().map(|s| s.to_string()).collect(),
        };

        let cmd = match mapping_cmd {
            Some(cmd) => Some(
                cmd.as_str()
                    .ok_or(anyhow::anyhow!("Invalid FromCommand: `cmd` is not a string"))?
                    .to_string(),
            ),
            None => None,
        };

        let chdir = mapping_chdir
            .map(|v| {
                v.as_str()
                    .ok_or(anyhow::anyhow!("Invalid FullCommand: `chdir` is not a string"))
            })
            .transpose()?;


        Ok(FullCommand {
            prog,
            args,
            cmd,
            chdir: chdir.map(|s| s.to_string()),
        })
    }
}

pub struct SubCommands(pub HashMap<String, Command>);

impl Validatable for SubCommands {
    fn validate(&self, m: Mapping, run_command_flags: &RunCommandFlags) {
        if ! run_command_flags.quiet {
            let keys = m.keys().filter(|k| {
                let k = k.as_str().unwrap_or("");
                k != "sub"
            }).collect::<Vec<_>>();

            if ! keys.is_empty() {
                let keys = keys.iter().map(|k| k.as_str().unwrap_or("")).collect::<Vec<_>>().join(", ").red();
                eprintln!("{}: Ignoring invalid keys for sub commands: {}", "Warning".bright_yellow().bold(), keys);
            }
        }
    }
}

struct SubCommandsInput<'a> {
    pub m: Mapping,
    pub run_command_flags: &'a RunCommandFlags,
}

impl TryFrom<SubCommandsInput<'_>> for SubCommands {
    type Error = anyhow::Error;

    fn try_from(input: SubCommandsInput) -> anyhow::Result<SubCommands> {
        if let Some(sub) = input.m.get("sub") {
            if let Some(sub) = sub.as_mapping() {
                let mut commands = HashMap::new();

                for (k, v) in sub.iter() {
                    let k = k
                        .as_str()
                        .ok_or(anyhow::anyhow!(
                            "Invalid SubCommands: key is not a string"
                        ))?
                        .to_string();

                    let v = Command::try_from(CommandInput{ v: v.clone(), command_name: k.clone(), run_command_flags: input.run_command_flags })?;

                    commands.insert(k, v);
                }

                return Ok(SubCommands(commands));
            }
        }

        Err(anyhow::anyhow!(
            "Command is an invalid yaml configuration for a subcommand."
        ))
    }
}

pub struct FromCommand {
    pub from: String,
    pub cmd: String,
}

pub struct FromCommandInput {
    pub m: serde_yaml::Mapping,
    pub command_name: String,
}

impl Validatable for FromCommand {
    fn validate(&self, m: Mapping, run_command_flags: &RunCommandFlags) {
        if ! run_command_flags.quiet {
            let keys = m.keys().filter(|k| {
                let k = k.as_str().unwrap_or("");
                k != "from" && k != "cmd"
            }).collect::<Vec<_>>();

            if !keys.is_empty() {
                let keys = keys.iter().map(|k| k.as_str().unwrap_or("")).collect::<Vec<_>>().join(", ").red();
                eprintln!("{}: Ignoring invalid keys for a from command: {}", "Warning".bright_yellow().bold(), keys);
            }
        }
    }
}

impl TryFrom<FromCommandInput> for FromCommand {
    type Error = anyhow::Error;

    fn try_from(input: FromCommandInput) -> anyhow::Result<FromCommand> {
        let from = input.m
            .get("from")
            .ok_or(anyhow::anyhow!("Invalid FromCommand: `from` not found"))?
            .as_str()
            .ok_or(anyhow::anyhow!("Invalid FromCommand: `from` is not a string"))?
            .to_string();

        let mapping_cmd = input.m.get("cmd");

        let cmd = match mapping_cmd {
            Some(cmd) =>
                cmd.as_str()
                    .ok_or(anyhow::anyhow!("Invalid FromCommand: `cmd` is not a string"))?
                    .to_string(),
            None => input.command_name,
        };

        Ok(FromCommand { from, cmd })
    }
}

pub enum Command {
    SimpleCommand(SimpleCommand),
    FullCommand(FullCommand),
    SubCommands(SubCommands),
    FromCommand(FromCommand),
}

pub struct CommandInput<'a> {
    pub v: Value,
    pub command_name: String,
    pub run_command_flags: &'a RunCommandFlags,
}

impl TryFrom<CommandInput<'_>> for Command {
    type Error = anyhow::Error;

    fn try_from(input: CommandInput) -> anyhow::Result<Command> {
        match input.v {
            Value::String(s) => Ok(Command::SimpleCommand(s.to_string())),
            Value::Mapping(m) => {
                if let Ok(from_command) = FromCommand::try_from(FromCommandInput{ m: m.clone(), command_name: input.command_name.clone() }) {
                    from_command.validate(m, input.run_command_flags);
                    return Ok(Command::FromCommand(from_command));
                }

                if let Ok(sub_command) = SubCommands::try_from(SubCommandsInput{ m: m.clone(), run_command_flags: input.run_command_flags }) {
                    sub_command.validate(m, input.run_command_flags);
                    return Ok(Command::SubCommands(sub_command));
                }

                if let Ok(full_command) = FullCommand::try_from(m.clone()) {
                    full_command.validate(m, input.run_command_flags);
                    return Ok(Command::FullCommand(full_command));
                }

                Err(anyhow::anyhow!(
                    "Command is an invalid yaml configuration for a command."
                ))
            }
            _ => Err(anyhow::anyhow!(
                "Command is not a string, or a valid yaml configuration for a command."
            )),
        }
    }
}
