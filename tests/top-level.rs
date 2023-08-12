#[cfg(test)]
mod top_level {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn top_level() -> Result<()> {
        ya().args(["run"])
            .current_dir("examples/top-level")
            .assert()
            .success()
            .stdout("This file is not in a .config directory!\n");

        Ok(())
    }
}
