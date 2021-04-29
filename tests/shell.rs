#[cfg(test)]
mod config {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn config() -> Result<()> {
        ya().args(&["config"]).assert().success();

        Ok(())
    }
}
