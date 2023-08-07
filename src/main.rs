mod cli;
mod cmd;
mod config;
mod git;
mod validate;

use clap::Parser;
use cli::Args;
use cmd::{run_command_from_config, RunCommandFlags};
use config::{get_config_path, parse_config_from_file, print_config_from_file};
use validate::{validate_config_file, validate_sd};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

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
            command_name.as_str(),
            &RunCommandFlags {
                sd: args.sd,
                quiet: args.quiet,
                execution: args.execution,
            },
            args.extra_args.as_slice(),
            0,
        )?
    }

    Ok(())
}
