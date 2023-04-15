#[cfg(test)]
mod sd {
    use anyhow::Result;
    use assert_cmd::Command;
    fn ya() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Error invoking ya")
    }

    #[test]
    fn sd() -> Result<()> {
        ya().args(["-c", "examples/sd/.config/ya.yml", "sd"])
            .assert()
            .success()
            .stdout("<person> loves <other person>!\n");

        Ok(())
    }
    #[test]
    fn sd_with_substitution() -> Result<()> {
        ya().args(["-c", "examples/sd/.config/ya.yml", "--sd", "<person>=tester" ,"--sd", "<other person>=ya", "sd"])
            .assert()
            .success()
            .stdout("tester loves ya!\n");

        Ok(())
    }
}
