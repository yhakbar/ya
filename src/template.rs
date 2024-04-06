use std::{path::{Path, PathBuf}, fs::OpenOptions, io::Write};
use anyhow::{Result, bail};
use handlebars::Handlebars;
use dialoguer::{theme::ColorfulTheme, Confirm};

fn parse_templates_dir(dir: &Option<PathBuf>) -> Result<PathBuf> {
    if let Some(dir) = dir {
        if dir.exists() && dir.is_dir() {
            return Ok(dir.to_path_buf())
        } else {
            anyhow::bail!("Directory `{}` does not exist", dir.display());
        }
    }

    let current_dir = std::env::current_dir()?;
    let templates_dir = current_dir.join("templates");
    if templates_dir.exists() && templates_dir.is_dir() {
        return Ok(templates_dir)
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("No templates directory found. Would you like to create one?")
        .interact()
        .unwrap_or(false)
    {
        std::fs::create_dir(&templates_dir)?;
        return Ok(templates_dir)
    }

    anyhow::bail!("This command requires a `templates` directory. Please create one and try again.");
}

fn get_templates(dir: &Path) -> Result<Vec<String>> {
    let mut templates = Vec::new();

    for template in dir.read_dir()? {
        let template = template?;
        let template_name = template.file_name();
        if let Some(template_name) = template_name.to_str() {
            templates.push(template_name.to_string());
        } else {
            anyhow::bail!("Could not convert template name to string");
        }
    }

    Ok(templates)
}

pub fn list_templates(dir: Option<PathBuf>) -> Result<(), anyhow::Error> {
    let dir = parse_templates_dir(&dir)?;
    let templates = get_templates(&dir)?;
    for template in templates {
        println!("{}", template);
    }
    Ok(())
}

struct Parameter {
    name: String,
    value: String,
}

fn parse_parameters(parameters: Option<Vec<String>>) -> Result<Vec<Parameter>, anyhow::Error> {
    let parameters = if let Some(parameters) = parameters {
        parameters
            .iter()
            .map(|p| {
                let mut split = p.split('=');
                if split.clone().count() != 2 {
                    bail!("Invalid parameter format: `{}`", p);
                }
                let name = if let Some(name) = split.next() {
                    name
                } else {
                    bail!("Could not get parameter name from `{}`", p);
                };
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    bail!("Could not get parameter value from `{}`", p);
                };
                Ok(Parameter {
                    name: name.to_string(),
                    value: value.to_string(),
                })
            })
            .collect::<Result<Vec<Parameter>, anyhow::Error>>()?
    } else {
        vec![]
    };
    Ok(parameters)
}

fn save_file_to_template(
    template_relative_path: &Path,
    file: &Path,
    parameters: &[Parameter],
) -> Result<(), anyhow::Error> {
    let file_basename = if let Some(file_basename) = file.file_name() {
        file_basename.to_str()
    } else {
        bail!("Could not get file name");
    };

    let file_basename = if let Some(file_basename) = file_basename {
        file_basename
    } else {
        bail!("Could not convert file name to string");
    };

    let template_file_path = template_relative_path.join(format!("{}.hbs", file_basename));
    std::fs::copy(file, &template_file_path)?;
    let initial_content = std::fs::read_to_string(file)?;
    let mut file = OpenOptions::new().write(true).open(&template_file_path)?;
    let content = parameters
        .iter()
        .fold(initial_content, |content, parameter| {
            let placeholder = format!("{{{{{}}}}}", &parameter.name);
            content.replace(&parameter.value, &placeholder)
        });
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn parse_dir_name(dir: &Path) -> Result<&str> {
    let dir_name = if let Some(dir_name) = dir.file_name() {
        if let Some(dir_name) = dir_name.to_str() {
            dir_name
        } else {
            bail!("Could not convert directory name to string");
        }
    } else {
        bail!("Could not get directory name");
    };
    Ok(dir_name)
}

#[cfg(feature = "ignore")]
fn handle_dir_template(dir: &PathBuf, template_path: &Path, parameters: &[Parameter], hidden: bool) -> Result<(), anyhow::Error> {
    use ignore::WalkBuilder;

    let dir_prefix = parse_dir_prefix(dir)?.to_owned();
    let dir_name = parse_dir_name(dir)?.to_owned();
    for result in WalkBuilder::new(dir).hidden(hidden).build() {
        let entry = result?;

        let entry_path = entry.path();

        if !entry_path.is_file() {
            continue;
        }

        let entry_relative_path = entry_path.strip_prefix(&dir_prefix)?.strip_prefix(&dir_name)?;

        if let Some(parent_dir) = entry_relative_path.parent() {
            let template_relative_path_parent = template_path.join(parent_dir);
            std::fs::create_dir_all(
                &template_relative_path_parent
            )?;
            save_file_to_template(
                &template_relative_path_parent,
                entry_path,
                parameters,
            )?;
            return Ok(());
        }

        save_file_to_template(
            template_path,
            entry_path,
            parameters,
        )?;
    }
    Ok(())
}

fn handle_file_template(file: &Path, template_path: &Path, parameters: &[Parameter]) -> Result<(), anyhow::Error> {
    save_file_to_template(template_path, file, parameters)
}

fn save_target_to_template(
    file: &PathBuf,
    template_path: &Path,
    template_name: &str,
    parameters: &[Parameter],
    hidden: bool,
) -> Result<(), anyhow::Error> {
    if file.is_file() {
        handle_file_template(file, template_path, parameters)?;
        return Ok(());
    }
    if !file.is_dir() {
        bail!("Target is neither a file nor directory");
    }
    #[cfg(feature = "ignore")]
    handle_dir_template(file, template_path, parameters, hidden)?;
    println!("Template \"{}\" created", template_name);
    Ok(())
}


fn parse_dir_prefix(dir: &Path) -> Result<&Path> {
    let path_prefix = if let Some(path_prefix) = dir.parent() {
        path_prefix
    } else {
        bail!("Could not get path prefix");
    };
    Ok(path_prefix)
}

fn parse_template_name(file: &Path) -> Result<&str> {
    let template_name = if let Some(file_name) = file.file_name() {
        if let Some(file_name) = file_name.to_str() {
            file_name
        } else {
            bail!("Could not convert file name to string")
        }
    } else {
        bail!("Could not get file name")
    };
    Ok(template_name)
}

fn generate_yadayada_config(parameters: Vec<Parameter>, template_path: PathBuf) -> Result<(), anyhow::Error> {
    let inputs = serde_yaml::Sequence::from(
        parameters
            .iter()
            .map(|p| serde_yaml::Value::String(p.name.clone()))
            .collect::<Vec<_>>(),
    );
    let mut template_config = serde_yaml::Mapping::new();
    template_config.insert(
        serde_yaml::Value::String("inputs".to_string()),
        serde_yaml::Value::Sequence(inputs),
    );
    let mut config = serde_yaml::Mapping::new();
    config.insert(
        serde_yaml::Value::String("template".to_string()),
        serde_yaml::Value::Mapping(template_config),
    );
    let config_path = template_path.join(".config");
    std::fs::create_dir(&config_path)?;
    let yadayada_config_path = config_path.join("yadayada.yml");
    let mut file = std::fs::File::create(yadayada_config_path)?;
    serde_yaml::to_writer(&mut file, &config)?;
    Ok(())
}


pub fn save_template(dir: Option<PathBuf>, file: PathBuf, parameters: Option<Vec<String>>, hidden: bool) -> Result<(), anyhow::Error> {
    let templates_dir = parse_templates_dir(&dir)?;
    let template_name = parse_template_name(&file)?;
    let template_path = templates_dir.join(template_name);
    if template_path.exists() {
        bail!("Template `{}` already exists", template_name);
    }
    println!("Creating template \"{}\"", template_name);
    std::fs::create_dir(&template_path)?;
    let parameters = parse_parameters(parameters)?;
    save_target_to_template(
        &file,
        &template_path,
        template_name,
        &parameters,
        hidden,
    )?;
    generate_yadayada_config(parameters, template_path)?;
    Ok(())
}

fn stamp_template_to_target(
    template_path: &Path,
    target: &Path,
    parameters: &[Parameter],
) -> Result<(), anyhow::Error> {
    let mut hbs = Handlebars::new();
    let opts = handlebars::DirectorySourceOptions {
        tpl_extension: ".hbs".to_string(),
        ..Default::default()
    };
    hbs.register_templates_directory(template_path, opts)?;
    hbs.register_escape_fn(handlebars::no_escape);

    let yadayada_config_path = template_path.join(".config").join("yadayada.yml");
    let yadayada_config = if yadayada_config_path.exists() {
        Some(
            serde_yaml::from_reader::<_, serde_yaml::Value>(
                std::fs::File::open(&yadayada_config_path)?
            )?
        )
    } else {
        None
    };

    let mut data = serde_json::Map::new();

    if let Some(yadayada_config) = &yadayada_config {
        if let Some(inputs) = yadayada_config.get("template") {
            if let Some(inputs) = inputs.get("inputs") {
                let inputs = inputs.as_sequence().ok_or_else(|| anyhow::anyhow!("Inputs is not a sequence"))?;
                for input in inputs {
                    let input = input.as_str().ok_or_else(|| anyhow::anyhow!("Input is not a string"))?.to_string();
                    let value = parameters
                        .iter()
                        .find(|p| p.name == input)
                        .map(|p| p.value.clone());

                    if let Some(value) = value {
                        data.insert(input, serde_json::Value::String(value));
                    } else {
                        bail!("Parameter `{}` not defined", input);
                    }
                }
            }
        }
    }

    for template_name in hbs.get_templates().keys() {
        let content = hbs.render(template_name, &data)?;

        let target_file_path = target.join(template_name);

        if let Some(parent) = target_file_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(&target_file_path)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}

pub fn stamp_template(source: String, target: PathBuf, dir: Option<PathBuf>, parameters: Option<Vec<String>>) -> Result<(), anyhow::Error> {
    let templates_dir = parse_templates_dir(&dir)?;
    let template_path = templates_dir.join(&source);
    if !template_path.exists() {
        bail!("Template `{}` does not exist", source);
    }
    let parameters = parse_parameters(parameters)?;
    stamp_template_to_target(
        &template_path,
        &target,
        &parameters,
    )?;
    Ok(())
}

#[cfg(test)]
mod parse_dir_prefix {
    use tempfile::tempdir;
    use anyhow::Result;
    use super::parse_dir_prefix;

    #[test]
    fn basic() -> Result<()> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path().join("example");
        std::fs::create_dir(&dir_path)?;

        let path_prefix = parse_dir_prefix(&dir_path)?;

        assert_eq!(path_prefix, temp_dir.path());

        Ok(())
    }
}

#[cfg(test)]
mod save_target_to_template {
    use tempfile::tempdir;
    use anyhow::Result;
    use super::save_target_to_template;

    #[test]
    fn basic() -> Result<()> {
        let temp_dir = tempdir()?;

        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("test.txt");
        std::fs::create_dir(&template_path)?;

        let file_path = temp_dir.path().join("test.txt");
        std::fs::File::create(&file_path)?;

        save_target_to_template(
            &file_path,
            &template_path,
            "test",
            &[],
            false,
        )?;

        assert!(template_path.join("test.txt.hbs").exists());

        Ok(())
    }

    #[test]
    fn w_dir() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("example");
        std::fs::create_dir_all(&template_path)?;
        let dir_path = temp_dir.path().join("example");
        std::fs::create_dir_all(&dir_path)?;
        let file_path = dir_path.join("test.txt");
        std::fs::File::create(file_path)?;

        save_target_to_template(
            &dir_path,
            &template_path,
            "example",
            &[],
            false,
        )?;

        assert!(template_path.join("test.txt.hbs").exists());

        Ok(())
    }
}

#[cfg(test)]
mod handle_file_template {
    use tempfile::tempdir;
    use anyhow::Result;
    use crate::template::Parameter;

    use super::handle_file_template;

    #[test]
    fn basic() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("test.txt");
        std::fs::create_dir(&template_path)?;
        let file_path = temp_dir.path().join("test.txt");
        std::fs::File::create(&file_path)?;

        handle_file_template(
            &file_path,
            &template_path,
            &[],
        )?;

        assert!(template_path.join("test.txt.hbs").exists());

        Ok(())
    }

    #[test]
    fn with_parameters() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("test.txt");
        std::fs::create_dir(&template_path)?;
        let file_path = temp_dir.path().join("test.txt");
        std::fs::File::create(&file_path)?;

        handle_file_template(
            &file_path,
            &template_path,
            &[Parameter {
                name: "test".to_string(),
                value: "1".to_string(),
            }],
        )?;

        assert!(template_path.join("test.txt.hbs").exists());

        Ok(())
    }
}

#[cfg(test)]
mod handle_dir_template {
    use tempfile::tempdir;
    use anyhow::Result;
    use crate::template::Parameter;

    use super::handle_dir_template;

    #[test]
    fn basic() -> Result<()> {
        let temp_dir = tempdir()?;

        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("test");
        std::fs::create_dir(&template_path)?;

        let dir_path = temp_dir.path().join("test");
        std::fs::create_dir(&dir_path)?;
        let file_path = dir_path.join("test.txt");
        std::fs::File::create(file_path)?;

        handle_dir_template(
            &dir_path,
            &template_path,
            &[],
            false,
        )?;

        assert!(template_path.join("test.txt.hbs").exists());

        Ok(())
    }

    #[test]
    fn with_parameters() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("test");
        std::fs::create_dir(&template_path)?;
        let file_path = temp_dir.path().join("test.txt");
        std::fs::File::create(&file_path)?;

        handle_dir_template(
            &file_path,
            &template_path,
            &[Parameter {
                name: "test".to_string(),
                value: "1".to_string(),
            }],
            false,
        )?;

        assert!(template_path.join("test.txt.hbs").exists());

        Ok(())
    }
}

#[cfg(test)]
mod save_file_to_template {
    use tempfile::tempdir;
    use anyhow::Result;
    use crate::template::Parameter;

    use super::save_file_to_template;

    #[test]
    fn basic() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("test");
        std::fs::create_dir(&template_path)?;
        let file_path = temp_dir.path().join("test.txt");
        std::fs::File::create(&file_path)?;

        save_file_to_template(
            &template_path,
            &file_path,
            &[],
        )?;

        assert!(template_path.join("test.txt.hbs").exists());

        Ok(())
    }

    #[test]
    fn with_parameters() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("test");
        std::fs::create_dir(&template_path)?;
        let file_path = temp_dir.path().join("test.txt");
        std::fs::File::create(&file_path)?;

        save_file_to_template(
            &template_path,
            &file_path,
            &[Parameter {
                name: "test".to_string(),
                value: "1".to_string(),
            }],
        )?;

        assert!(template_path.join("test.txt.hbs").exists());

        Ok(())
    }

    #[test]
    fn with_more_parameters() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("test");
        std::fs::create_dir(&template_path)?;

        let contents = "test 1 2 finish";

        let file_path = template_path.join("test.txt");
        std::fs::write(&file_path, contents)?;

        save_file_to_template(
            &template_path,
            &file_path,
            &[
                Parameter {
                    name: "test".to_string(),
                    value: "1".to_string(),
                },
                Parameter {
                    name: "other_test".to_string(),
                    value: "2".to_string(),
                },
            ],
        )?;

        assert!(template_path.join("test.txt.hbs").exists());

        let template_file_contents = std::fs::read_to_string(template_path.join("test.txt.hbs"))?;
        assert_eq!(template_file_contents, "test {{test}} {{other_test}} finish");

        Ok(())
    }

    #[test]
    fn in_dir() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        let template_path = templates_dir.join("example");
        std::fs::create_dir_all(&template_path)?;
        let dir_path = temp_dir.path().join("example");
        std::fs::create_dir_all(&dir_path)?;
        let file_path = dir_path.join("test.txt");
        std::fs::File::create(&file_path)?;

        let template_file_dir = template_path.join("example");
        std::fs::create_dir_all(template_file_dir)?;

        save_file_to_template(
            &template_path,
            &file_path,
            &[],
        )?;

        assert!(template_path.exists());
        assert!(template_path.join("test.txt.hbs").exists());

        Ok(())
    }
}

#[cfg(test)]
mod parse_parameters {
    use anyhow::Result;
    use super::parse_parameters;

    #[test]
    fn basic() -> Result<()> {
        let parameters = parse_parameters(Some(vec!["test=1".to_string()]))?;

        assert_eq!(parameters.len(), 1);
        assert_eq!(parameters[0].name, "test");
        assert_eq!(parameters[0].value, "1");

        Ok(())
    }

    #[test]
    fn with_two_parameters() -> Result<()> {
        let parameters = parse_parameters(Some(vec!["test=1".to_string(), "test2=2".to_string(), "key=to_be_replaced".to_string()]))?;

        assert_eq!(parameters.len(), 3);
        assert_eq!(parameters[0].name, "test");
        assert_eq!(parameters[0].value, "1");
        assert_eq!(parameters[1].name, "test2");
        assert_eq!(parameters[1].value, "2");
        assert_eq!(parameters[2].name, "key");
        assert_eq!(parameters[2].value, "to_be_replaced");

        Ok(())
    }

    #[test]
    fn with_no_parameters() -> Result<()> {
        let parameters = parse_parameters(None)?;

        assert_eq!(parameters.len(), 0);

        Ok(())
    }

    #[test]
    fn with_invalid_parameter() -> Result<()> {
        let parameters = parse_parameters(Some(vec!["test".to_string()]));

        assert!(parameters.is_err());

        Ok(())
    }
}

#[cfg(test)]
mod list_templates {
    use tempfile::tempdir;
    use anyhow::Result;
    use super::list_templates;

    #[test]
    fn basic() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(templates_dir)?;

        list_templates(Some(temp_dir.path().to_path_buf()))?;

        // I'm not testing println. I'm OK with that.

        Ok(())
    }
}

#[cfg(test)]
mod get_templates {
    use tempfile::tempdir;
    use anyhow::Result;
    use super::get_templates;

    #[test]
    fn basic() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;

        let templates = get_templates(&templates_dir)?;

        assert_eq!(templates.len(), 0);

        Ok(())
    }

    #[test]
    fn with_one_template() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        std::fs::File::create(templates_dir.join("test.hbs"))?;

        let templates = get_templates(&templates_dir)?;

        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0], "test.hbs");

        Ok(())
    }

    #[test]
    fn with_two_templates() -> Result<()> {
        let temp_dir = tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir(&templates_dir)?;
        std::fs::File::create(templates_dir.join("test.hbs"))?;
        std::fs::File::create(templates_dir.join("test2.hbs"))?;

        let templates = get_templates(&templates_dir)?;

        assert_eq!(templates.len(), 2);

        let filename1 = "test.hbs".to_owned();
        let filename2 = "test2.hbs".to_owned();

        assert!(templates.contains(&filename1));
        assert!(templates.contains(&filename2));

        Ok(())
    }
}

#[cfg(test)]
mod parse_templates_dir {
    use tempfile::tempdir;
    use anyhow::Result;
    use super::parse_templates_dir;

    #[test]
    fn basic() -> Result<()> {
        let temp_dir = tempdir()?;

        let templates_dir = parse_templates_dir(&Some(temp_dir.path().to_path_buf()))?;

        assert_eq!(templates_dir, temp_dir.path().to_path_buf());

        Ok(())
    }
}
