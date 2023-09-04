#[cfg(test)]
mod sub {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn first() -> Result<()> {
        ya().args(["run", "first"])
            .current_dir("examples/sub")
            .assert()
            .success()
            .stdout("First\n");

        Ok(())
    }

    #[test]
    fn second() -> Result<()> {
        ya().args(["run", "second"])
            .current_dir("examples/sub")
            .assert()
            .success()
            .stdout("Second\n");

        Ok(())
    }

    #[test]
    fn third() -> Result<()> {
        ya().args(["run", "third"])
            .current_dir("examples/sub")
            .assert()
            .success()
            .stdout("Third\n");

        Ok(())
    }

    #[test]
    fn the_first() -> Result<()> {
        ya().args(["run", "the", "first"])
            .current_dir("examples/sub")
            .assert()
            .success()
            .stdout("The First\n");

        Ok(())
    }

    #[test]
    fn the_second() -> Result<()> {
        ya().args(["run", "the", "second"])
            .current_dir("examples/sub")
            .assert()
            .success()
            .stdout("The Second\n");

        Ok(())
    }

    #[test]
    fn the_third() -> Result<()> {
        ya().args(["run", "the", "third"])
            .current_dir("examples/sub")
            .assert()
            .success()
            .stdout("The Third\n");

        Ok(())
    }

    #[test]
    fn with_python() -> Result<()> {
        ya().args(["run", "with", "python"])
            .current_dir("examples/sub")
            .assert()
            .success()
            .stdout("apple is a fruit with 5 characters!\nbanana is a fruit with 6 characters!\nwatermelon is a fruit with 10 characters!\n");

        Ok(())
    }

    #[test]
    fn from() -> Result<()> {
        ya().args(["release", "version"])
            .current_dir("examples/sub/from")
            .assert()
            .success()
            .stdout(".config/ya/release.yml\nEnsuring dependency\nReleasing a version!\n");

        Ok(())
    }
}
