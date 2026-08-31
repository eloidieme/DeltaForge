//! `deltaforge init`: the automation form of project creation.
//!
//! The browser creation flow and this command share one engine
//! ([`crate::creation`], driven through [`crate::application::create_project`]),
//! so a project scripted into existence is identical to one created from the
//! catalog. This command keeps its historical freedom to write anywhere the
//! shell can reach: it is invoked by a person at a prompt or by CI, not by a
//! web page.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};

use crate::cli::InitArgs;
use crate::context::GlobalOptions;
use crate::creation;
use crate::pack::{PackSearchOptions, load_pack};

use super::default_project_directory;

pub fn run(args: InitArgs, options: &GlobalOptions) -> Result<()> {
    let pack = load_pack(
        &args.project,
        &PackSearchOptions {
            packs_dir: options.packs_dir.clone(),
        },
    )?;
    let language = pack.manifest.language(&args.lang).with_context(|| {
        format!(
            "pack {} does not support language {}",
            args.project, args.lang
        )
    })?;
    for tool in creation::language_tools(language) {
        if tool.required && !tool.found {
            bail!(
                "{} is required to build this project and was not found on PATH",
                tool.label
            );
        }
    }

    let stage = match args.stage.as_deref() {
        Some(stage_id) => pack
            .manifest
            .stage(stage_id)
            .with_context(|| format!("pack {} does not contain stage {stage_id}", args.project))?,
        None => pack
            .manifest
            .first_stage()
            .with_context(|| format!("pack {} does not define any stages", args.project))?,
    };

    let target = PathBuf::from(default_project_directory(&args));
    if target.exists() {
        bail!("target directory already exists: {}", target.display());
    }

    creation::create(&pack, &args.lang, &target, stage, !args.no_git)?;

    println!("deltaforge init");
    println!("Project: {}", args.project);
    println!("Language: {}", args.lang);
    println!("Target directory: {}", target.display());
    println!("Current stage: {} - {}", stage.id, stage.title);
    println!(
        "Git initialization: {}",
        if args.no_git { "disabled" } else { "enabled" }
    );
    println!();
    println!("Created project.");
    println!("Next:");
    println!("  cd {}", target.display());
    println!("  deltaforge");

    Ok(())
}
