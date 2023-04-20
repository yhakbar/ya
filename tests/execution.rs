#[cfg(test)]
mod basic {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn basic() -> Result<()> {
        ya().args(["-x", "run"])
            .current_dir("examples/basic")
            .assert()
            .success()
            .stdout("$ bash -c \'echo \"Hey ya!\"\'\nHey ya!\n");

        Ok(())
    }
}
