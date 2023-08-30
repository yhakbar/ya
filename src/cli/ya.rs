use clap::Parser;

use std::path::PathBuf;

/// ya - yet another command runner.
#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
pub struct YaArgs {
    /// Suppress extra output.
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// The config file.
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Print the config file before running.
    #[arg(short, long, default_value_t = false)]
    pub print: bool,

    /// Print the executed command before executing it.
    #[arg(short = 'x', long, default_value_t = false)]
    pub execution: bool,

    /// No color.
    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    /// The command to run.
    #[arg()]
    pub command: Option<String>,

    /// The extra arguments to pass to the command.
    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    pub extra_args: Vec<String>,
}
