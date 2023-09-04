#[cfg(test)]
mod invalid {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn full_cmd() -> Result<()> {
        ya().args(["run"])
            .current_dir("examples/invalid/full-cmd")
            .assert()
            .success()
            .stdout("Hey!\n")
            .stderr("Warning: Ignoring invalid keys for a full command: fake\n");

        Ok(())
    }
    #[test]
    fn quiet() -> Result<()> {
        ya().args(["-q", "run"])
            .current_dir("examples/invalid/full-cmd")
            .assert()
            .success()
            .stdout("Hey!\n")
            .stderr("");

        Ok(())
    }
}
