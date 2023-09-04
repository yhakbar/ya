#[cfg(test)]
mod named {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    /// This test is ignored by default because it requires an installed `ya` binary to be accessible in PATH.
    #[test]
    #[ignore]
    fn named() -> Result<()> {
        ya().args([ "-c", ".config/ya/named.yml", "run"])
            .current_dir("examples/named")
            .assert()
            .success()
            .stdout("Running the test.\nTest passed.\n");

        Ok(())
    }
}
