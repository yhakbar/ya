#[cfg(test)]
mod error {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn error() -> Result<()> {
        ya().args(["error"])
            .current_dir("examples/error")
            .assert()
            .failure()
            .stderr("");

        Ok(())
    }
}
