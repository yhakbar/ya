use std::env;

use serde_derive::{Serialize, Deserialize};

use std::process::Command;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerBuildConfig {
    image: String,
    dockerfile: String,
    docker_context: String,
    workdir: String,
    volumes: Vec<String>,
}

impl DockerBuildConfig {
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
