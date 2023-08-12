#[cfg(test)]
mod basic {
    use anyhow::Result;
    use assert_cmd::Command;
    fn yadayada() -> Command {
        Command::cargo_bin("yadayada").expect("Error invoking yadayada")
    }

    #[test]
    fn basic() -> Result<()> {
        for command in ["keys", "k"] {
            yadayada().args([command])
                .current_dir("examples/basic")
                .assert()
                .success()
                .stdout("run\n");
        }

        Ok(())
    }
}
