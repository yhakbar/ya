#[cfg(test)]
mod alias {
    use anyhow::Result;
    use assert_cmd::Command;
    use tempfile::tempdir;

    fn yadayada() -> Command {
        Command::cargo_bin("yadayada").expect("Error invoking yadayada")
    }

    #[test]
    fn alias() -> Result<()> {
        for command in ["alias", "a"] {
            let tmp = tempdir()?;

            let tmp_ya_yml = tmp.path().join("ya.yml");
            let mut value = serde_yaml::Mapping::new();
            value.insert(
                serde_yaml::Value::String("previous-command".to_string()),
                serde_yaml::Value::String("echo 'This is a previous command!'".to_string()),
            );
            serde_yaml::to_writer(
                std::fs::File::create(&tmp_ya_yml).expect("Error creating ya.yml"),
                &value,
            )?;

            let filename = tmp_ya_yml.to_str().expect("Error converting tmp_ya_yml to str");

            yadayada().args([command, "-c", filename, "echo-test", "echo 'test.'"])
                .assert()
                .success();

            let file_value = serde_yaml::from_reader::<_, serde_yaml::Value>(
                std::fs::File::open(&tmp_ya_yml).expect("Error opening ya.yml"),
            )?;

            let echo_test_value = file_value
                .get(&serde_yaml::Value::String("echo-test".to_string()))
                .expect("Error getting echo-test value");

            println!("{}", std::fs::read_to_string(&tmp_ya_yml).expect("Error reading ya.yml"));

            assert_eq!(
                echo_test_value,
                &serde_yaml::Value::String("echo 'test.'".to_string())
            );
        }

        Ok(())
    }
}
