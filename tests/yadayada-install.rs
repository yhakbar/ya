#[cfg(test)]
mod basic {
    use anyhow::Result;
    use assert_cmd::Command;
    use tempdir::TempDir;

    fn yadayada() -> Command {
        Command::cargo_bin("yadayada").expect("Error invoking yadayada")
    }

    #[test]
    fn basic() -> Result<()> {
        for command in ["alias", "a"] {
            // Create a temporary directory.
            // We want to write a yaml file "ya.yml" to it.
            // We want to run yadayada from it.
            // After we run yadayada, we expect an alias to be created in ya.yml.
            // We expect the alias to be "echo-test".

            // Create a temporary directory.
            // We want to write a yaml file "ya.yml" to it.

            let prefix = format!("yadayada-{}-test", &command);

            let tmp = TempDir::new(&prefix)?;

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
