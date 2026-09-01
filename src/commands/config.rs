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
                println!(
                    "Config: {}",
                    crate::fs_util::display_path(&context.config_path)
                );
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
                        path: crate::fs_util::display_path(&context.config_path),
                    })?
                );
            } else {
                println!("✓ config is valid");
                println!(
                    "Path: {}",
                    crate::fs_util::display_path(&context.config_path)
                );
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
