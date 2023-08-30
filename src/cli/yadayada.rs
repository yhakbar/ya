use clap::{Parser, Subcommand};

use std::path::PathBuf;


#[derive(Subcommand)]
pub enum TemplateSubcommand {
    /// List templates.
    #[command(about, alias = "ls")]
    List {
        /// The templates directory to list.
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },

    /// Stamp a saved template.
    #[command(about, arg_required_else_help(true))]
    Stamp {
        /// The templates directory to fetch from.
        #[arg(short, long)]
        dir: Option<PathBuf>,

        /// Strings to replace parameters with from the template.
        #[arg(short, long)]
        parameters: Option<Vec<String>>,

        /// The template to stamp.
        #[arg()]
        source: String,

        /// The target of the template.
        #[arg()]
        target: PathBuf,
    },

    /// Save a template from an existing file path.
    #[command(about, arg_required_else_help(true))]
    Save {
        /// The templates directory to save to.
        #[arg(short, long)]
        dir: Option<PathBuf>,

        /// Strings to replace with parameters in the template.
        #[arg(short, long)]
        parameters: Option<Vec<String>>,

        /// Save hidden files too.
        #[arg(short = 'H', long)]
        hidden: bool,

        /// The file path of the template.
        #[arg()]
        file: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum YadaYadaSubcommand {
    /// Install command completion for `ya` and `yadayada`.
    #[command(about, alias = "i")]
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
    #[command(about, alias = "k")]
    Keys {
        /// The config to print the keys of.
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Alias a command, and add to config.
    #[command(about, alias = "a")]
    Alias {
        /// The config to add the alias to.
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// The name of the alias.
        #[arg()]
        name: String,

        /// The command to alias.
        #[arg()]
        command: String,
    },

    /// Manage templates.
    #[command(about, alias = "t", arg_required_else_help(true))]
    Template {
        #[command(subcommand)]
        subcommand: Option<TemplateSubcommand>,
    },
}

/// yadayada - save yourself some chatter.
#[derive(Parser)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct YadaYadaArgs {
    /// No color.
    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    /// Subcommand of `yadayada`.
    #[command(subcommand)]
    pub subcommand: Option<YadaYadaSubcommand>,
}
