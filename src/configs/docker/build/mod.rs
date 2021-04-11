use std::env;

use serde_derive::{Serialize, Deserialize};

use std::process::Command;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerBuildConfig {
    image: Option<String>,
    dockerfile: Option<String>,
    docker_context: Option<String>,
    workdir: Option<String>,
    volumes: Option<Vec<String>>,
}

impl DockerBuildConfig {
    fn build_docker_image(&self) {
        let image = &self.image.as_ref().expect("image must be defined to build image");
        let dockerfile = &self.dockerfile.as_ref().expect("dockerfile must be defined to build image");
        let docker_context = &self.docker_context.as_ref().expect("docker_context must be defined to build image");
        let build_output = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(image)
            .arg("-f")
            .arg(dockerfile)
            .arg(docker_context)
            .output()
            .expect("failed to build image");
        println!("{}", String::from_utf8_lossy(&build_output.stdout));
        println!("{}", String::from_utf8_lossy(&build_output.stderr));
    }

    // TODO Not bothering with proper volume mount bindings right now, but I should.
    fn run_docker_container(&self) {
        let pwd = env::current_dir().unwrap().to_str().unwrap().to_string();
        let volume_mount = str::replace("$PWD:/app", "$PWD", &pwd);
        let workdir = &self.workdir.as_ref().expect("workdir must be defined to run container");
        let image = &self.image.as_ref().expect("image must be defined to run container");
        let build_output = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-v")
            .arg(&volume_mount)
            .arg("-w")
            .arg(workdir)
            .arg(image)
            .output()
            .expect("failed to run container");
        println!("{}", String::from_utf8_lossy(&build_output.stdout));
        println!("{}", String::from_utf8_lossy(&build_output.stderr));
    }

    pub fn build(&self, _arguments: &[String]) {
        self.build_docker_image();
        self.run_docker_container();
    }
}
