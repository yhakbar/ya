#[cfg(feature = "yadayada")]
#[cfg(test)]
mod keys {
    use anyhow::Result;
    use assert_cmd::Command;
    fn yadayada() -> Command {
        Command::cargo_bin("yadayada").expect("Error invoking yadayada")
    }

    #[test]
    fn keys() -> Result<()> {
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
