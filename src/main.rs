mod config;
mod yml;
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
    YML {},
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
        Ya::YML {} => {
            yml::print_some_yml();
        }
    }

    Ok(())
}
