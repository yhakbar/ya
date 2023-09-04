#[cfg(feature = "yadayada")]
#[cfg(test)]
mod base {
    use anyhow::Result;
    use assert_cmd::Command;
    fn yadayada() -> Command {
        Command::cargo_bin("yadayada").expect("Error invoking yadayada")
    }

    #[test]
    fn base() -> Result<()> {
        yadayada().args(["-h"]).assert().success();
        yadayada().args(["--help"]).assert().success();
        yadayada().args(["-V"]).assert().success();
        yadayada().args(["--version"]).assert().success();

        Ok(())
    }
}
