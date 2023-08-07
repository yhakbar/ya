#[cfg(not(feature = "gh-release"))]
use std::io::Error;

#[cfg(feature = "gh-release")]
use clap::CommandFactory;

#[cfg(feature = "gh-release")]
include!("src/cli.rs");
#[cfg(feature = "gh-release")]
include!("src/completion.rs");

#[cfg(feature = "gh-release")]
fn build_completions() -> Result<(), Error> {
    let release_dir = "completions/release";

    let mut cmd = Args::command();

    build_templated_completions(&mut cmd, release_dir)?;

    Ok(())
}

fn main() -> Result<(), Error> {
    #[cfg(feature = "gh-release")]
    build_completions()?;

    Ok(())
}
