mod config;
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
            help = "Nest configuration in .config file",
            required = false,
            default_value = ".config/ya"
        )]
        config: String,
    },
    #[structopt(name = "yml", about = "Prints some yml")]
    YML {
        #[structopt(parse(from_os_str), help = "File to read")]
        file: PathBuf,
    },
}

fn init(config: &str) -> std::io::Result<()> {
    config::create_if_not_exists(config)
}

fn main() -> std::io::Result<()> {
    let args = Ya::from_args();
    match args {
        Ya::Init { config } => {
            init(&config)?;
        }
        Ya::YML { file } => {
            let file_str = file.to_str();
            match file_str {
                None => panic!("path is not a valid UTF-8 sequence"),
                Some(s) => yml::load_yml_configurations(&s),
            }
        }
    }

    Ok(())
}
