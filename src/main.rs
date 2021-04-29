mod configs;
mod handlers;
mod ya;

use crate::handlers::build::handle_build;
use crate::handlers::config::handle_config;
use crate::handlers::init::handle_init;
use crate::handlers::run::handle_run;
use crate::handlers::shell::handle_shell;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ya", about = "Tool to automate command virtualization")]
enum Ya {
    #[structopt(name = "init", about = "Initializes ya project")]
    Init {
        #[structopt(
            short,
            long,
            help = "Location of configuration file",
            required = false,
            default_value = ".config/ya/ya.yml"
        )]
        config: String,
        #[structopt(short, long, help = "Name of this project")]
        name: Option<String>,
    },
    #[structopt(name = "config", about = "Print out the ya file")]
    Config {
        #[structopt(
            short,
            long,
            help = "Location of configuration file",
            required = false,
            default_value = ".config/ya/ya.yml"
        )]
        config: PathBuf,
    },
    #[structopt(name = "build", about = "Runs a build according to ya configuration")]
    Build {
        #[structopt(
            short,
            long,
            help = "Location of configuration file",
            required = false,
            default_value = ".config/ya/ya.yml"
        )]
        config: PathBuf,
        #[structopt(short, long, help = "Don't parse arguments e.g. $@")]
        no_arguments: bool,
        #[structopt(help = "Optional arguments to pass into command")]
        arguments: Vec<String>,
    },
    #[structopt(name = "run", about = "Runs a command according to ya configuration")]
    Run {
        #[structopt(
            short,
            long,
            help = "Location of configuration file",
            required = false,
            default_value = ".config/ya/ya.yml"
        )]
        config: PathBuf,
        #[structopt(short, long, help = "Don't parse arguments e.g. $@")]
        no_arguments: bool,
        #[structopt(help = "Optional arguments to pass into command")]
        arguments: Vec<String>,
    },
    #[structopt(name = "shell", about = "Starts a shell according to ya configuration")]
    Shell {
        #[structopt(
            short,
            long,
            help = "Location of configuration file",
            required = false,
            default_value = ".config/ya/ya.yml"
        )]
        config: PathBuf,
    },
}

fn main() -> std::io::Result<()> {
    let args = Ya::from_args();
    match args {
        Ya::Init { config, name } => {
            handle_init(&config, &name)?;
        }
        Ya::Config { config } => {
            let config_str = config
                .to_str()
                .expect("config path is not a valid UTF-8 sequence");
            handle_config(config_str).expect("failed to handle configuration command correctly");
        }
        Ya::Build {
            config,
            arguments,
            no_arguments,
        } => {
            let config_str = config
                .to_str()
                .expect("config path is not a valid UTF-8 sequence");
            handle_build(config_str, &arguments, no_arguments)
                .expect("failed to handle build command correctly");
        }
        Ya::Run {
            config,
            arguments,
            no_arguments,
        } => {
            let config_str = config
                .to_str()
                .expect("config path is not a valid UTF-8 sequence");
            handle_run(config_str, &arguments, no_arguments)
                .expect("failed to handle run command correctly");
        }
        Ya::Shell { config } => {
            let config_str = config
                .to_str()
                .expect("config path is not a valid UTF-8 sequence");
            handle_shell(config_str).expect("failed to handle shell command correctly");
        }
    }

    Ok(())
}
