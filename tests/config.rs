#[cfg(test)]
mod shell {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn shell() -> Result<()> {
        ya().args(&["shell"]).assert().success();

        Ok(())
    }
}
