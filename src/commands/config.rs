use anyhow::Result;
use serde::Serialize;

use crate::cli::ConfigCommand;
use crate::context::{GlobalOptions, ProjectContext};

pub fn run(command: ConfigCommand, options: &GlobalOptions) -> Result<()> {
    match command {
        ConfigCommand::Show(args) => {
            let context = ProjectContext::load(options)?;
            if args.json {
                println!("{}", serde_json::to_string_pretty(&context.config)?);
            } else {
                println!("Config: {}", context.config_path.display());
                println!("{}", toml::to_string_pretty(&context.config)?);
            }
        }
        ConfigCommand::Validate(args) => {
            let context = ProjectContext::load(options)?;
            if args.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&ConfigValidation {
                        valid: true,
                        path: context.config_path.display().to_string(),
                    })?
                );
            } else {
                println!("✓ config is valid");
                println!("Path: {}", context.config_path.display());
            }
        }
    }
    Ok(())
}

#[derive(Debug, Serialize)]
struct ConfigValidation {
    valid: bool,
    path: String,
}
