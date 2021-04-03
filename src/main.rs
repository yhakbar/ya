#![deny(unused_crate_dependencies)]

mod config;
mod configs;
mod fs;
mod ya;
mod init;
mod build;

use crate::init::handle_init;
use crate::config::handle_config;
use crate::build::handle_build;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ya", about = "Tool to automate command virtualization")]
enum Ya {
    #[structopt(name = "init", about = "Initializes ya project")]
    Init {
        #[structopt(
            short = "c",
            long = "config",
            help = "Location of configuration file",
            required = false,
            default_value = ".config/ya/ya.yml"
        )]
        config: String,
    },
    #[structopt(name = "config", about = "Print out the ya file")]
    Config {
        #[structopt(
            short = "c",
            long = "config",
            help = "Location of configuration file",
            required = false,
            default_value = ".config/ya/ya.yml"
        )]
        config: PathBuf,
    },
    #[structopt(name = "build", about = "Runs a build according to ya configuration")]
    Build {
        #[structopt(
            short = "c",
            long = "config",
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
        Ya::Init { config } => {
            handle_init(&config)?;
        }
        Ya::Config { config } => {
            let config_str = config.to_str().expect("config path is not a valid UTF-8 sequence");
            handle_config(config_str).expect("failed to handle configuration command correctly");
        }
        Ya::Build { config } => {
            let config_str = config.to_str().expect("config path is not a valid UTF-8 sequence");
            handle_build(config_str).expect("failed to handle build command correctly");
        }
    }

    Ok(())
}
