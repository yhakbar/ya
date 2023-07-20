use clap::Parser;

use std::path::PathBuf;

mod cmd;
mod config;
mod git;
mod validate;

use cmd::{run_command_from_config, RunCommandFlags};
use config::{get_config_path, parse_config_from_file, print_config_from_file};
use validate::{validate_config_file, validate_sd};

/// Automation tool for lazy people.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Suppress the output of `pre_msg` and `post_msg`.
    #[arg(short, long, default_value_t = false)]
    quiet: bool,

    /// The config file to use.
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Print the config file before running the command.
    #[arg(short, long, default_value_t = false)]
    print: bool,

    /// Search and displacements to make in the command before running it.
    /// Expects a key and value separated by an `=`. e.g. `--sd key=value`
    #[arg(long)]
    sd: Vec<String>,

    /// Print the command that will be executed before executing it.
    #[arg(short = 'x', long, default_value_t = false)]
    execution: bool,

    /// Don't add color to anything printed to stdout by ya.
    #[arg(long, default_value_t = false)]
    no_color: bool,

    /// The command in the config to use.
    #[arg()]
    command: Option<String>,

    /// The extra arguments to pass to the command
    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    extra_args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    validate_sd(&args.sd)?;

    let config_path = get_config_path(&args.config)?;

    if args.print {
        print_config_from_file(&config_path)?;
    }

    validate_config_file(&config_path)?;

    let config = parse_config_from_file(&config_path)?;

    if let Some(command_name) = args.command {
        run_command_from_config(
            &config,
            command_name,
            &RunCommandFlags {
                sd: args.sd,
                quiet: args.quiet,
                execution: args.execution,
                no_color: args.no_color,
            },
            args.extra_args.as_slice(),
        )?
    }

    Ok(())
}
