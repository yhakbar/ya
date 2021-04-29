#[cfg(test)]
mod init {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn init() -> Result<()> {
        ya().args(&["init"]).assert().success();

        Ok(())
    }
}
