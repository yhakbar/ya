mod config;
mod configs;
mod fs;
mod yml;
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

fn init(config: &str) -> std::io::Result<()> {
    fs::create_if_not_exists(config)
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Ya::from_args();
    match args {
        Ya::Init { config } => {
            init(&config)?;
        }
        Ya::Build { config } => {
            let file_str = config.to_str();
            match file_str {
                None => panic!("path is not a valid UTF-8 sequence"),
                Some(s) => {
                    config::new_from_path(&s);
                }
            }
        }
    }

    Ok(())
}
