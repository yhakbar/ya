use serde_yaml::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

const DEFAULT_CONFIG_PATHS: [&str; 4] = [".config/ya.yml", ".config/ya.yaml", "ya.yml", "ya.yaml"];

pub fn get_config_path(path: &Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let path = match path {
        Some(path) => path,
        None => {
            for config_path in DEFAULT_CONFIG_PATHS.iter() {
                let path = Path::new(config_path);
                if path.exists() && path.is_file() {
                    return Ok(path.to_path_buf());
                }
            }

            return Err(anyhow::anyhow!(
                "Could not find config file in default locations. Please specify a config file."
            ));
        }
    };
    Ok(path.to_path_buf())
}

pub fn parse_config_from_file(path: &Path) -> anyhow::Result<Value> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let ya = serde_yaml::from_reader(r)?;
    Ok(ya)
}

pub fn print_config_from_file(path: &Path) -> anyhow::Result<()> {
    let config = parse_config_from_file(path)?;
    println!("---\n{}---\n", serde_yaml::to_string(&config)?);
    Ok(())
}

pub struct ParsedConfig {
    pub parsed_command: ParsedCommand,
    pub pre_msg: Option<String>,
    pub post_msg: Option<String>,
    pub pre_cmds: Option<Vec<String>>,
    pub post_cmds: Option<Vec<String>>,
}

pub struct ParsedCommand {
    pub prog: String,
    pub args: Vec<String>,
    pub cmd: Option<String>,
}

impl std::fmt::Display for ParsedCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display = self.prog.clone();
        for arg in self.args.iter() {
            display.push_str(&format!(" {}", arg));
        }
        if let Some(cmd) = &self.cmd {
            display.push_str(&format!(" '{}'", cmd));
        }
        write!(f, "{}", display)
    }
}

pub fn parse_cmd(cmd: &Value) -> anyhow::Result<ParsedConfig> {
    const DEFAULT_PROG_VALUE: &str = "bash";
    const DEFAULT_ARGS_VALUE: &[&str] = &["-c"];

    let default_prog: Value = Value::String(DEFAULT_PROG_VALUE.to_string());
    let default_args: Value = Value::Sequence(
        DEFAULT_ARGS_VALUE
            .iter()
            .map(|s| Value::String(s.to_string()))
            .collect(),
    );

    match cmd {
        Value::String(s) => Ok(ParsedConfig {
            parsed_command: ParsedCommand {
                prog: DEFAULT_PROG_VALUE.to_string(),
                args: DEFAULT_ARGS_VALUE.iter().map(|s| s.to_string()).collect(),
                cmd: Some(s.to_string()),
            },
            pre_msg: None,
            post_msg: None,
            pre_cmds: None,
            post_cmds: None,
        }),
        Value::Mapping(m) => {
            let config_prog = m.get("prog");

            let args_if_empty = match config_prog {
                Some(_) => Value::Sequence(vec![]),
                None => default_args,
            };

            let prog = config_prog
                .unwrap_or(&default_prog)
                .as_str()
                .ok_or(anyhow::anyhow!("Invalid Config: `prog` is not a string"))?;

            let args = m
                .get("args")
                .unwrap_or(&args_if_empty)
                .as_sequence()
                .ok_or(anyhow::anyhow!("Invalid Config: `args` is not a sequence"))?
                .iter()
                .map(|a| {
                    a.as_str()
                        .ok_or(anyhow::anyhow!(
                            "Invalid Config: `arg` is not a string: {:?}",
                            a
                        ))
                        .map(|s| s.to_string())
                })
                .collect::<anyhow::Result<Vec<String>>>()?;

            let cmd = m
                .get("cmd")
                .map(|v| {
                    v.as_str()
                        .ok_or(anyhow::anyhow!("Invalid Config: `cmd` is not a string"))
                })
                .transpose()?;

            let pre_msg = m
                .get("pre_msg")
                .map(|v| {
                    v.as_str()
                        .ok_or(anyhow::anyhow!("Invalid Config: `pre_msg` is not a string"))
                })
                .transpose()?;
            let post_msg = m
                .get("post_msg")
                .map(|v| {
                    v.as_str().ok_or(anyhow::anyhow!(
                        "Invalid Config: `post_msg` is not a string"
                    ))
                })
                .transpose()?;

            let pre_cmds = m
                .get("pre_cmds")
                .map(|v| {
                    v.as_sequence().ok_or(anyhow::anyhow!(
                        "Invalid Config: `pre_cmds` is not a sequence"
                    ))
                })
                .transpose()?;
            let pre_cmds = pre_cmds
                .map(|v| {
                    v.iter()
                        .map(|v| {
                            v.as_str().ok_or(anyhow::anyhow!(
                                "Invalid Config: pre_cmd is not a string: {:?}",
                                v
                            ))
                        })
                        .collect::<anyhow::Result<Vec<&str>>>()
                        .map(|v| v.iter().map(|s| s.to_string()).collect())
                })
                .transpose()?;

            let post_cmds = m
                .get("post_cmds")
                .map(|v| {
                    v.as_sequence().ok_or(anyhow::anyhow!(
                        "Invalid Config: `post_cmds` is not a sequence"
                    ))
                })
                .transpose()?;
            let post_cmds = post_cmds
                .map(|v| {
                    v.iter()
                        .map(|v| {
                            v.as_str().ok_or(anyhow::anyhow!(
                                "Invalid Config: `post_cmd` is not a string: {:?}",
                                v
                            ))
                        })
                        .collect::<anyhow::Result<Vec<&str>>>()
                        .map(|v| v.iter().map(|s| s.to_string()).collect())
                })
                .transpose()?;

            Ok(ParsedConfig {
                parsed_command: ParsedCommand {
                    prog: prog.to_string(),
                    args,
                    cmd: cmd.map(|s| s.to_string()),
                },
                pre_msg: pre_msg.map(|s| s.to_string()),
                post_msg: post_msg.map(|s| s.to_string()),
                pre_cmds,
                post_cmds,
            })
        }
        _ => Err(anyhow::anyhow!(
            "Command is not a string, or a valid yaml configuration for a command."
        )),
    }
}
