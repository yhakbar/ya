use anyhow::{bail, Ok};
use clap::{CommandFactory, Parser};
use home::home_dir;
use serde_yaml::{Mapping, Value};
use std::{
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
};
use ya::{
    cli::{ya::YaArgs, yadayada::{YadaYadaArgs, YadaYadaSubcommand}},
    completion::build_fish_completion,
    config::get_config_path,
};

#[cfg(feature = "templating")]
use ya::{
    cli::yadayada::TemplateSubcommand,
    template::{list_templates, save_template, stamp_template}
};

fn main() -> anyhow::Result<()> {
    let args = YadaYadaArgs::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

    if let Some(subcommand) = args.subcommand {
        match subcommand {
            YadaYadaSubcommand::Install { shell, directory } => {
                let shell = match shell {
                    Some(shell) => shell,
                    None => {
                        let shell_path_str =
                            std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
                        let shell_path = PathBuf::from(shell_path_str);
                        let shell = shell_path.file_name().unwrap().to_str().unwrap();
                        shell.to_string()
                    }
                };
                match shell.as_str() {
                    "fish" => {
                        let directory = match directory {
                            Some(directory) => Ok(directory),
                            None => {
                                if let Some(home_dir) = home_dir() {
                                    let fish_dir = home_dir.join(".config/fish/completions");
                                    Ok(fish_dir)
                                } else {
                                    bail!("Could not find home directory");
                                }
                            }
                        }?;
                        if let Some(directory) = directory.to_str() {
                            let mut cmd = YaArgs::command();
                            build_fish_completion(&mut cmd, directory, "ya")?;
                            let mut cmd = YadaYadaArgs::command();
                            build_fish_completion(&mut cmd, directory, "yadayada")?;
                        } else {
                            bail!("Could not convert directory to string");
                        }
                    }
                    _ => {
                        bail!("Shell `{}` not supported for automatic installation. Please install completion manually.", shell);
                    }
                }
            }
            YadaYadaSubcommand::Keys { config } => {
                let config_path = get_config_path(&config)?;
                let config = ya::config::parse_config_from_file(&config_path)?;
                match config {
                    Value::Mapping(config) => {
                        let keys = config.keys();
                        for key in keys {
                            let key = key.as_str();
                            if let Some(key) = key {
                                println!("{}", key);
                            }
                        }
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }
            YadaYadaSubcommand::Alias {
                config,
                name,
                command,
            } => {
                let config_path = get_config_path(&config)?;
                let config = ya::config::parse_config_from_file(&config_path)?;
                match config {
                    Value::Mapping(config) => {
                        if config.contains_key(&name) {
                            bail!("Alias `{}` already exists", name);
                        }
                        let mut config_to_append = Mapping::new();
                        config_to_append.insert(Value::String(name), Value::String(command));
                        let mut file = OpenOptions::new().append(true).open(&config_path)?;
                        file.write_all(b"\n")?;
                        serde_yaml::to_writer(&mut file, &config_to_append)?;
                        return Ok(());
                    }
                    _ => return Ok(()),
                }
            }
            #[cfg(feature = "templating")]
            YadaYadaSubcommand::Template { subcommand } => {
                if let Some(subcommand) = subcommand {
                    match subcommand {
                        TemplateSubcommand::List { dir } => list_templates(dir)?,
                        TemplateSubcommand::Save {
                            file,
                            parameters,
                            hidden,
                            dir,
                        } => save_template(dir, file, parameters, hidden)?,
                        TemplateSubcommand::Stamp { dir, parameters, source, target } => stamp_template(source, target, dir, parameters)?,
                    }
                }
            }
        }
    }

    Ok(())
}
