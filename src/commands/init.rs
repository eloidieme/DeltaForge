use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::cli::InitArgs;
use crate::config::ProjectConfig;
use crate::context::GlobalOptions;
use crate::pack::{LoadedPack, PackSearchOptions, StageSpec, load_pack};
use crate::state::ProjectState;

use super::default_project_directory;
use super::overview::read_pack_overview;

pub fn run(args: InitArgs, options: &GlobalOptions) -> Result<()> {
    let loaded_pack = load_pack(
        &args.project,
        &PackSearchOptions {
            packs_dir: options.packs_dir.clone(),
        },
    )?;
    let language = loaded_pack.manifest.language(&args.lang).with_context(|| {
        format!(
            "pack {} does not support language {}",
            args.project, args.lang
        )
    })?;

    let current_stage = match args.stage.as_deref() {
        Some(stage_id) => loaded_pack
            .manifest
            .stage(stage_id)
            .with_context(|| format!("pack {} does not contain stage {stage_id}", args.project))?,
        None => loaded_pack
            .manifest
            .first_stage()
            .with_context(|| format!("pack {} does not define any stages", args.project))?,
    };

    let target_directory = PathBuf::from(default_project_directory(&args));
    if target_directory.exists() {
        bail!(
            "target directory already exists: {}",
            target_directory.display()
        );
    }

    let template_root = loaded_pack.root.join(&language.template);
    copy_dir_recursive(&template_root, &target_directory).with_context(|| {
        format!(
            "failed to copy template {} to {}",
            template_root.display(),
            target_directory.display()
        )
    })?;

    write_deltaforge_metadata(&target_directory, &args, current_stage)?;
    write_readme(&target_directory, &loaded_pack, current_stage)?;

    if !args.no_git {
        initialize_git(&target_directory)?;
    }

    println!("deltaforge init");
    println!("Project: {}", args.project);
    println!("Language: {}", args.lang);
    println!("Target directory: {}", target_directory.display());
    println!(
        "Current stage: {} - {}",
        current_stage.id, current_stage.title
    );
    println!(
        "Git initialization: {}",
        if args.no_git { "disabled" } else { "enabled" }
    );
    println!();
    println!("Created project.");
    println!("Next:");
    println!("  cd {}", target_directory.display());
    println!("  deltaforge overview");
    println!("  deltaforge instructions");
    println!("  deltaforge test");

    Ok(())
}

fn write_deltaforge_metadata(
    target_directory: &Path,
    args: &InitArgs,
    current_stage: &StageSpec,
) -> Result<()> {
    let deltaforge_dir = target_directory.join(".deltaforge");
    fs::create_dir_all(&deltaforge_dir).with_context(|| {
        format!(
            "failed to create DeltaForge metadata directory {}",
            deltaforge_dir.display()
        )
    })?;

    let state = ProjectState::new(
        args.project.clone(),
        args.lang.clone(),
        current_stage.id.clone(),
    )?;
    state.write_to(&deltaforge_dir.join("state.json"))?;

    ProjectConfig::default().write_to(&deltaforge_dir.join("config.toml"))?;

    Ok(())
}

fn write_readme(
    target_directory: &Path,
    loaded_pack: &LoadedPack,
    current_stage: &StageSpec,
) -> Result<()> {
    let manifest = &loaded_pack.manifest;
    let overview = read_pack_overview(loaded_pack);
    let roadmap = manifest
        .stages
        .iter()
        .map(|stage| {
            let marker = if stage.id == current_stage.id {
                "→"
            } else {
                "○"
            };
            format!("{marker} `{}` - {}", stage.id, stage.title)
        })
        .collect::<Vec<_>>()
        .join("\n");
    let readme = format!(
        "# {}\n\n{}\n\n{}\n\n## Current Stage\n\n`{}` - {}\n\n## Stage Roadmap\n\n{}\n\n## DeltaForge Commands\n\n```bash\ndeltaforge overview\ndeltaforge instructions\ndeltaforge test\ndeltaforge hint\ndeltaforge status\ndeltaforge next\n```\n",
        manifest.name,
        manifest.description,
        overview.trim(),
        current_stage.id,
        current_stage.title,
        roadmap
    );

    fs::write(target_directory.join("README.md"), readme).with_context(|| {
        format!(
            "failed to write project README {}",
            target_directory.join("README.md").display()
        )
    })?;

    Ok(())
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    if !source.is_dir() {
        bail!("template directory does not exist: {}", source.display());
    }

    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create directory {}", destination.display()))?;

    for entry in fs::read_dir(source)
        .with_context(|| format!("failed to read directory {}", source.display()))?
    {
        let entry = entry
            .with_context(|| format!("failed to read directory entry in {}", source.display()))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to inspect {}", source_path.display()))?;

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        }
    }

    Ok(())
}

fn initialize_git(target_directory: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("init")
        .current_dir(target_directory)
        .output()
        .with_context(|| format!("failed to run git init in {}", target_directory.display()))?;

    if !output.status.success() {
        bail!(
            "git init failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    Ok(())
}
