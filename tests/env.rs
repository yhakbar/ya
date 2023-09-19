#[cfg(test)]
mod envs {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn envs() -> Result<()> {
        ya().args(["envs"])
            .current_dir("examples/envs")
            .assert()
            .success()
            .stdout("custom value\n");

        Ok(())
    }
}
