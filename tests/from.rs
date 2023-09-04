#[cfg(test)]
mod from {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn from_noop() -> Result<()> {
        ya().args(["run"])
            .current_dir("examples/from")
            .assert()
            .success()
            .stdout("Hey, from the from directory!\n");

        Ok(())
    }
    #[test]
    fn from_relative() -> Result<()> {
        ya().args(["run"])
            .current_dir("examples/from/relative")
            .assert()
            .success()
            .stdout("Hey, from the from directory!\n");

        Ok(())
    }
    #[test]
    fn from_git() -> Result<()> {
        ya().args(["run"])
            .current_dir("examples/from/git")
            .assert()
            .success()
            .stdout("Hey, from the from directory!\n");

        Ok(())
    }
    #[test]
    fn alternate() -> Result<()> {
        ya().args(["run"])
            .current_dir("examples/from/alternate")
            .assert()
            .success()
            .stdout("Alternate hey, from the from directory!\n");

        Ok(())
    }

    /// This test is ignored by default because it requires an installed `ya` binary to be accessible in PATH.
    #[test]
    #[ignore]
    fn chained() -> Result<()> {
        ya().args(["call"])
            .current_dir("examples/from/chained")
            .assert()
            .success()
            .stdout("Done\n");

        Ok(())
    }
}
