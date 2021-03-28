use log::warn;
use std::env;

extern crate yaml_rust;
use yaml_rust::yaml::Hash;

use std::process::Command;

#[derive(Debug, Default)]
pub struct DockerBuildConfig {
    image: String,
    dockerfile: String,
    docker_context: String,
    workdir: String,
    volumes: Vec<String>,
}

impl DockerBuildConfig {
    pub fn new(docker_build_config: &Hash) -> DockerBuildConfig {
        let mut config = DockerBuildConfig {
            image: String::from("ya-builder"),
            dockerfile: String::from("Dockerfile"),
            docker_context: String::from("."),
            workdir: String::from("/app"),
            volumes: vec![],
        };

        for (key, value) in docker_build_config.iter() {
            match key.as_str().unwrap() {
                "image" => config.image = value.as_str().unwrap().to_string(),
                "dockerfile" => config.dockerfile = value.as_str().unwrap().to_string(),
                "docker_context" => config.docker_context = value.as_str().unwrap().to_string(),
                "workdir" => config.workdir = value.as_str().unwrap().to_string(),
                "volumes" => {
                    config.volumes = vec![];
                    for volume in value.as_vec().unwrap() {
                        config.volumes.push(volume.as_str().unwrap().to_string());
                    }
                }
                _ => warn!("Unsupported key {:?}", key),
            }
        }
        config
    }

    fn build_docker_image(&self) {
        let build_output = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(&self.image)
            .arg("-f")
            .arg(&self.dockerfile)
            .arg(&self.docker_context)
            .output()
            .expect("failed to build image");
        println!("{}", String::from_utf8_lossy(&build_output.stdout));
        println!("{}", String::from_utf8_lossy(&build_output.stderr));
    }

    // TODO Not bothering with proper volume mount bindings right now, but I should.
    fn run_docker_container(&self) {
        let pwd = env::current_dir().unwrap().to_str().unwrap().to_string();
        let volume_mount = str::replace("$PWD:/app", "$PWD", &pwd);
        let build_output = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-v")
            .arg(volume_mount)
            .arg("-w")
            .arg(&self.workdir)
            .arg(&self.image)
            .output()
            .expect("failed to run container");
        println!("{}", String::from_utf8_lossy(&build_output.stdout));
        println!("{}", String::from_utf8_lossy(&build_output.stderr));
    }

    pub fn build(&self) {
        self.build_docker_image();
        self.run_docker_container();
    }
}
