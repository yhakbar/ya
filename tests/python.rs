#[cfg(test)]
mod python {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn python() -> Result<()> {
        ya().args(["-c", "examples/python/.config/ya.yml", "python_script"])
            .assert()
            .success()
            .stdout("apple is a fruit with 5 characters!\nbanana is a fruit with 6 characters!\nwatermelon is a fruit with 10 characters!\n");

        Ok(())
    }
}
