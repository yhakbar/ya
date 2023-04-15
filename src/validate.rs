use std::path::PathBuf;

pub fn validate_sd(sd: &Vec<String>) -> anyhow::Result<()> {
    for s in sd {
        if s.splitn(2, '=').count() != 2 {
            return Err(anyhow::anyhow!("Invalid search and displacement: {:?}", s));
        }
    }
    Ok(())
}

pub fn validate_config_file(config_file: &PathBuf) -> anyhow::Result<()> {
    if ! (config_file.exists() && config_file.is_file()) {
        return Err(anyhow::anyhow!("Config file {:?} not found", config_file));
    }
    Ok(())
}