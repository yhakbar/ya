use clap::Parser;
use ya::{
    cli::ya::YaArgs,
    cmd::{run_command_from_config, RunCommandFlags},
    config::{get_config_path, parse_config_from_file, print_config_from_file},
    validate::validate_config_file
};

fn main() -> anyhow::Result<()> {
    let args = YaArgs::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

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
                quiet: args.quiet,
                execution: args.execution,
            },
            args.extra_args.as_slice(),
            0,
        )?
    }

    Ok(())
}
