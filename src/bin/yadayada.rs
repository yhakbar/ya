use anyhow::Ok;
use clap::{CommandFactory, Parser, Subcommand};
use home::home_dir;
use serde_yaml::Value;
use std::path::PathBuf;
use ya::{cli::Args, completion::build_fish_completion, config::get_config_path};

#[derive(Subcommand)]
pub enum YadaYadaSubcommand {
    /// Install command completion for `ya` and `yadayada`.
    #[command(about, long_about = None, alias = "i")]
    Install {
        /// The shell to install command completion for.
        #[arg(short, long)]
        shell: Option<String>,

        /// The directory to install command completion to.
        /// Defaults to best guess for the shell.
        #[arg(short, long)]
        directory: Option<PathBuf>,
    },

    /// Print keys of a config.
    #[command(about, long_about = None, alias = "k")]
    Keys {
        /// The config to print the keys of.
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
}

/// Tool to manage command completion for `ya`.
#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
pub struct YadaYadaArgs {
    /// No color.
    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    /// Subcommand of `yadayada`.
    #[command(subcommand)]
    pub subcommand: Option<YadaYadaSubcommand>,
}

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
                                    return Err(anyhow::anyhow!("Could not find home directory"));
                                }
                            }
                        }?;
                        if let Some(directory) = directory.to_str() {
                            let mut cmd = Args::command();
                            build_fish_completion(&mut cmd, directory, "ya")?;
                            let mut cmd = YadaYadaArgs::command();
                            build_fish_completion(&mut cmd, directory, "yadayada")?;
                        } else {
                            return Err(anyhow::anyhow!("Could not convert directory to string"));
                        }
                    }
                    _ => {
                        return Err(anyhow::anyhow!("Shell `{}` not supported for automatic installation. Please install completion manually.", shell));
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
        }
    }

    Ok(())
}
