use std::process::Command;

pub fn get_git_root() -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;
    let root = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(root)
}
