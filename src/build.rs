use crate::ya::BuildConfigs::DockerBuildConfig;
use crate::ya::parse_ya_config_from_file;

pub fn handle_build(config: &str) -> std::io::Result<()> {
    let ya_config = parse_ya_config_from_file(&config);
    match ya_config {
        Ok(ya_config) => {
            match ya_config.build {
                Some(build) => {
                    match build.plugin.as_ref() {
                        "docker" => {
                            match build.config {
                                DockerBuildConfig(docker) => {
                                    docker.build();
                                }
                            }
                        }
                        _ => {
                            println!("Plugin unsupported");
                        }
                    }
                }
                None => {
                    println!("Invalid configuration");
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    Ok(())
}