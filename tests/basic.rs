#[cfg(test)]
mod basic {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn basic() -> Result<()> {
        ya().args(["-c", "examples/basic/.config/ya.yml", "run"])
            .assert()
            .success()
            .stdout("Hey ya!\n");

        Ok(())
    }
}
