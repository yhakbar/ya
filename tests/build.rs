#[cfg(test)]
mod build {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn build() -> Result<()> {
        ya().args(&["build"]).assert().success();

        Ok(())
    }
}
