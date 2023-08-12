#[cfg(test)]
mod chdir {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn chdir_noop() -> Result<()> {
        ya().args(["local"])
            .current_dir("examples/chdir")
            .assert()
            .success()
            .stdout("Base directory\n");

        Ok(())
    }
    #[test]
    fn chdir_dir1() -> Result<()> {
        ya().args(["dir1"])
            .current_dir("examples/chdir")
            .assert()
            .success()
            .stdout("Directory one\n");

        Ok(())
    }
    #[test]
    fn chdir_dir2() -> Result<()> {
        ya().args(["dir2"])
            .current_dir("examples/chdir")
            .assert()
            .success()
            .stdout("Directory two\n");

        Ok(())
    }
    #[test]
    fn chdir_git() -> Result<()> {
        ya().args(["git"])
            .current_dir("examples/chdir")
            .assert()
            .success()
            .stdout("Directory relative to git root\n");

        Ok(())
    }
}
