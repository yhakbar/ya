use crate::fs::create_if_not_exists;

pub fn handle_init(config: &str) -> std::io::Result<()> {
    create_if_not_exists(config)
}
