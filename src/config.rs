
use crate::ya::parse_ya_from_file;

pub fn handle_config(config: &str) -> std::io::Result<()> {
    let ya = parse_ya_from_file(&config).expect("failed to parse config");
    let s = serde_yaml::to_string(&ya).expect("failed to serialize ya config");
    println!("{}", s);

    Ok(())
}
