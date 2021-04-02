mod config;
mod configs;
mod fs;
mod yml;
use std::path::PathBuf;
use structopt::StructOpt;
use yaml_rust::yaml::Yaml;

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
            let config: config::Config;
            match file_str {
                None => panic!("path is not a valid UTF-8 sequence"),
                Some(s) => {
                    config = config::new_from_path(&s);
                }
            }
            match config.config.get(&Yaml::from_str("build")) {
                Some(build) => {
                    let build_hash = build.as_hash().unwrap();
                    match build_hash.get(&Yaml::String("plugin".to_string())) {
                        Some(plugin) => {
                            let plugin_str = plugin.as_str().unwrap();
                            match plugin_str {
                                "docker" => {
                                    let docker_build_config =
                                        configs::docker::build::DockerBuildConfig::new(
                                            &build["config"].as_hash().unwrap(),
                                        );
                                    docker_build_config.build();
                                }
                                _ => {
                                    panic!("Plugin not supported");
                                }
                            }
                        }
                        None => {
                            panic!("No plugin for build config")
                        }
                    }
                }
                None => {
                    panic!("No build config");
                }
            }
        }
    }

    Ok(())
}
