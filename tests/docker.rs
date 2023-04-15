#[cfg(test)]
mod docker {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn docker() -> Result<()> {
        ya().args(["-c", "examples/docker/.config/ya.yml", "docker_script"])
            .assert()
            .success()
            .stdout("This is running in a docker container!\r\n");

        Ok(())
    }
}
