
use crate::ya::parse_ya_from_file;

pub fn handle_config(config: &str) -> std::io::Result<()> {
    let ya = parse_ya_from_file(&config);
    match ya {
        Ok(ya) => {
            let s = serde_yaml::to_string(&ya);
            match s {
                Ok(s) => {
                    println!("{}", s);
                }
                Err(e) => panic!("failed to serialize config: {:?}", e)
            }
        }
        Err(e) => panic!("failed to parse config: {:?}", e)
    }

    Ok(())
}
