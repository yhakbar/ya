#[cfg(feature = "completion")]
use clap_complete::{
    generate_to,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};
#[cfg(feature = "templating")]
use handlebars::Handlebars;
#[cfg(feature = "templating")]
use serde_json::json;
#[cfg(feature = "templating")]
use std::env::temp_dir;
#[cfg(feature = "templating")]
use std::io::Error;

#[cfg(feature = "templating")]
pub fn build_fish_completion(
    cmd: &mut clap::Command,
    release_dir: &str,
    bin_name: &str,
) -> Result<(), Error> {
    if let Some(outdir) = temp_dir().to_str() {
        let path = generate_to(Fish, cmd, bin_name, outdir)?;
        let template = match bin_name {
            "ya" => Some("ya.fish.hbs"),
            _ => None,
        };
        template_completion(path, release_dir, template)?
    }
    Ok(())
}

#[cfg(feature = "templating")]
pub fn build_bash_completion(
    cmd: &mut clap::Command,
    release_dir: &str,
    bin_name: &str,
) -> Result<(), Error> {
    if let Some(outdir) = temp_dir().to_str() {
        let path = generate_to(Bash, cmd, bin_name, outdir)?;
        template_completion(path, release_dir, None)?
    }
    Ok(())
}

#[cfg(feature = "templating")]
pub fn build_elvish_completion(
    cmd: &mut clap::Command,
    release_dir: &str,
    bin_name: &str,
) -> Result<(), Error> {
    if let Some(outdir) = temp_dir().to_str() {
        let path = generate_to(Elvish, cmd, bin_name, outdir)?;
        template_completion(path, release_dir, None)?
    }
    Ok(())
}

#[cfg(feature = "templating")]
pub fn build_zsh_completion(
    cmd: &mut clap::Command,
    release_dir: &str,
    bin_name: &str,
) -> Result<(), Error> {
    if let Some(outdir) = temp_dir().to_str() {
        let path = generate_to(Zsh, cmd, bin_name, outdir)?;
        template_completion(path, release_dir, None)?
    }
    Ok(())
}

#[cfg(feature = "templating")]
pub fn build_powershell_completion(
    cmd: &mut clap::Command,
    release_dir: &str,
    bin_name: &str,
) -> Result<(), Error> {
    if let Some(outdir) = temp_dir().to_str() {
        let path = generate_to(PowerShell, cmd, bin_name, outdir)?;
        template_completion(path, release_dir, None)?
    }
    Ok(())
}

#[cfg(feature = "templating")]
pub fn template_completion(
    generated_path: std::path::PathBuf,
    outdir: &str,
    template: Option<&str>,
) -> Result<(), Error> {
    let tpl_str = match template {
        Some("ya.fish.hbs") => include_str!("../completions/templates/ya.fish.hbs"),
        Some(_) => include_str!("../completions/templates/generated.hbs"),
        None => include_str!("../completions/templates/generated.hbs"),
    };

    let template = template.unwrap_or("generated.hbs");

    let mut handlebars = Handlebars::new();
    match handlebars.register_template_string(template, tpl_str) {
        Ok(_) => {}
        Err(e) => {
            println!("Error registering template: {}", e);
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Error registering template",
            ));
        }
    }
    let generated_fish = std::fs::read_to_string(&generated_path).unwrap();

    handlebars.register_escape_fn(handlebars::no_escape);
    let version = clap::crate_version!();
    let data = json!({ "generated_completes": generated_fish, "version": version });
    let rendered = handlebars.render(template, &data).unwrap();
    if let Some(file_name) = generated_path.file_name() {
        if let Some(file_name) = file_name.to_str() {
            let out_path = format!("{}/{}", outdir, file_name);
            match std::fs::write(&out_path, rendered) {
                Ok(_) => {}
                Err(e) => {
                    let error_msg =
                        format!("Error writing completion to file `{}`: {}", &out_path, e);
                    return Err(Error::new(std::io::ErrorKind::Other, error_msg));
                }
            }
        }
    };
    Ok(())
}

#[cfg(feature = "gh-release")]
pub fn build_templated_completions(
    cmd: &mut clap::Command,
    release_dir: &str,
) -> Result<(), Error> {
    for bin in &["ya", "yadayada"] {
        build_fish_completion(cmd, release_dir, bin)?;
        build_bash_completion(cmd, release_dir, bin)?;
        build_elvish_completion(cmd, release_dir, bin)?;
        build_zsh_completion(cmd, release_dir, bin)?;
        build_powershell_completion(cmd, release_dir, bin)?;
    }
    Ok(())
}
