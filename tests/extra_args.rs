#[cfg(test)]
mod extra_args {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn extra_args() -> Result<()> {
        ya().args(["-c", "examples/extra-args/.config/ya.yml", "extra", "tester!"])
            .assert()
            .success()
            .stdout("Hello, tester!\n");

        Ok(())
    }
}
