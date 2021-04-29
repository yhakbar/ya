#[cfg(test)]
mod run {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn run() -> Result<()> {
        ya().args(&["run", "ya", "-V"]).assert().success();

        Ok(())
    }
}
