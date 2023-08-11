use std::path::PathBuf;

pub fn validate_config_file(config_file: &PathBuf) -> anyhow::Result<()> {
    if !(config_file.exists() && config_file.is_file()) {
        return Err(anyhow::anyhow!("Config file {:?} not found", config_file));
    }
    Ok(())
}
