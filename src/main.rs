use clap::Parser;

use std::path::PathBuf;

const DEFAULT_CONFIG_PATH: &str = ".config/ya.yml";

mod cmd;
mod config;
mod validate;

use config::parse_config_from_file;
use cmd::run_command_from_config;
use validate::{validate_sd, validate_config_file};

/// Automation tool for lazy people.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Suppress the output of `pre_msg` and `post_msg`.
    #[arg(short, long, default_value_t = false)]
    quiet: bool,

    /// The config file to use.
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH)]
    config: PathBuf,

    /// Print the config file before running the command.
    #[arg(short, long, default_value_t = false)]
    print: bool,

    /// Search and displacements to make in the command before running it.
    /// Expects a key and value separated by an `=`. e.g. `--sd key=value`
    #[arg(long)]
    sd: Vec<String>,

    /// The command in the config to use.
    #[arg()]
    command: Option<String>,
}

fn main() -> anyhow::Result<()>{
    let args = Args::parse();

    validate_sd(&args.sd)?;

    if args.print {
        let config = parse_config_from_file(&args.config)?;
        println!("---\n{}---\n", serde_yaml::to_string(&config)?);
    }

    validate_config_file(&args.config)?;

    let config = parse_config_from_file(&args.config)?;

    if let Some(command_name) = args.command {
        run_command_from_config(&config, command_name, args.sd, args.quiet)?
    }

    Ok(())
}
