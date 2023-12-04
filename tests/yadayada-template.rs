#[cfg(feature = "yadayada")]
#[cfg(test)]
mod template {
    use anyhow::Result;
    use assert_cmd::Command;
    use tempfile::tempdir;

    fn yadayada() -> Command {
        Command::cargo_bin("yadayada").expect("Error invoking yadayada")
    }

    #[test]
    fn list() -> Result<()> {
        let tmp = tempdir()?;

        let templates_dir = tmp.path().join("templates");
        std::fs::create_dir(&templates_dir)?;

        let template_path = templates_dir.join("basic");
        std::fs::create_dir(template_path)?;

        for command in ["template", "t"] {
            yadayada().args([command, "ls"])
            .current_dir(&tmp)
            .assert()
            .success()
            .stdout("basic\n");
        }

        Ok(())
    }

    #[test]
    fn save_wo_templates() -> Result<()> {
        let tmp = tempdir()?;

        let dummy_file_path = tmp.path().join("dummy.txt");
        std::fs::write(dummy_file_path, "dummy")?;

        yadayada().args(["template", "save", "dummy"])
        .current_dir(&tmp)
        .assert()
        .failure();

        Ok(())
    }
    #[test]
    fn save() -> Result<()> {
        let tmp = tempdir()?;

        let dummy_file_path = tmp.path().join("dummy.txt");
        std::fs::write(dummy_file_path, "dummy")?;

        let templates_dir = tmp.path().join("templates");
        std::fs::create_dir(&templates_dir)?;

        yadayada().args(["template", "save", "dummy.txt"])
        .current_dir(&tmp)
        .assert()
        .success();

        let template_file_path = templates_dir.join("dummy.txt");
        assert!(template_file_path.exists());
        let template_file_contents = std::fs::read_to_string(template_file_path.join("dummy.txt.hbs"))?;
        assert_eq!(template_file_contents, "dummy");

        Ok(())
    }
    #[test]
    fn save_w_params() -> Result<()> {
        let tmp = tempdir()?;

        let dummy_file_path = tmp.path().join("dummy.txt");
        std::fs::write(dummy_file_path, "dummy replaced content\n")?;

        let templates_dir = tmp.path().join("templates");
        std::fs::create_dir(&templates_dir)?;

        yadayada().args(["template", "save", "-p", "key=replaced", "dummy.txt"])
        .current_dir(&tmp)
        .assert()
        .success();

        let template_dir_path = templates_dir.join("dummy.txt");
        assert!(template_dir_path.exists());
        let template_file_contents = std::fs::read_to_string(template_dir_path.join("dummy.txt.hbs"))?;
        assert_eq!(template_file_contents, "dummy {{key}} content\n\n");

        Ok(())
    }
    #[test]
    fn save_and_load() -> Result<()> {
        let tmp = tempdir()?;

        let dummy_file_path = tmp.path().join("dummy.txt");
        std::fs::write(dummy_file_path, "This is some content pre-replacement\n")?;

        let templates_dir = tmp.path().join("templates");
        std::fs::create_dir(&templates_dir)?;

        {
            yadayada().args(["template", "save", "-p", "time=pre", "dummy.txt"])
            .current_dir(&tmp)
            .assert()
            .success();

            let template_file_path = templates_dir.join("dummy.txt");
            assert!(template_file_path.exists());
            let template_file_contents = std::fs::read_to_string(template_file_path.join("dummy.txt.hbs"))?;
            assert_eq!(template_file_contents, "This is some content {{time}}-replacement\n");
        }
        {
            yadayada().args(["template", "ls"])
            .current_dir(&tmp)
            .assert()
            .success()
            .stdout("dummy.txt\n");
        }
        {
            yadayada().args(["template", "stamp", "-p", "time=post", "dummy.txt", "."])
            .current_dir(&tmp)
            .assert()
            .success();

            let stamped_file_path = tmp.path().join("dummy.txt");
            assert!(stamped_file_path.exists());
            let stamped_file_contents = std::fs::read_to_string(&stamped_file_path)?;
            assert_eq!(stamped_file_contents, "This is some content post-replacement\n");
        }

        Ok(())
    }
}
