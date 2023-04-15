#[cfg(test)]
mod base {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn base() -> Result<()> {
        ya().args(["-h"]).assert().success();
        ya().args(["--help"]).assert().success();
        ya().args(["-V"]).assert().success();
        ya().args(["--version"]).assert().success();

        Ok(())
    }
}
