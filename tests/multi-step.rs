#[cfg(test)]
mod multi_step {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn multi_step() -> Result<()> {
        ya().args(["-c", "examples/multi-step/.config/ya.yml", "multi_step"])
            .assert()
            .success()
            .stdout("These are the pre-commands. They will run before the main command.\nMain command\nThese are the post-commands. They will run after the main command.\n");

        Ok(())
    }
}
