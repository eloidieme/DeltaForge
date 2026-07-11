use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::Serialize;

use crate::authoring::{
    AddStageRequest, AuthoringReport, CheckReferenceRequest, NewPackRequest, add_stage,
    check_reference, create_pack, diagnose_pack,
};
use crate::cli::PackCommand;
use crate::context::GlobalOptions;
use crate::pack::{LoadedPack, PackSearchOptions, discover_packs_with_options, load_pack};

pub fn run(command: PackCommand, options: &GlobalOptions) -> Result<()> {
    match command {
        PackCommand::List(args) => list(args.json, options),
        PackCommand::Show(args) => show(&args.project, args.json, options),
        PackCommand::New(args) => {
            let report = create_pack(&NewPackRequest {
                id: args.id,
                name: args.name,
                description: args.description,
                dest: args.dest,
                language: args.lang,
                force: args.force,
            })?;
            print_report(&report, args.json)
        }
        PackCommand::AddStage(args) => {
            let report = add_stage(&AddStageRequest {
                pack_dir: args.pack_dir,
                id: args.id,
                title: args.title,
                force: args.force,
            })?;
            print_report(&report, args.json)
        }
        PackCommand::Doctor(args) => {
            let pack = load_pack(&args.project, &pack_options(options))?;
            let report = diagnose_pack(&pack);
            print_report(&report, args.json)
        }
        PackCommand::CheckReference(args) => {
            let report = check_reference(&CheckReferenceRequest {
                project: args.project,
                language: args.lang,
                reference: args.reference,
                packs_dir: options.packs_dir.clone(),
            })?;
            print_report(&report, args.json)?;
            if report.is_ok() {
                Ok(())
            } else {
                bail!("reference validation failed")
            }
        }
        PackCommand::Install(args) => install(&args.project, &args.dest, args.force, options),
    }
}

fn list(json: bool, options: &GlobalOptions) -> Result<()> {
    let packs = discover_packs_with_options(&pack_options(options))?;
    if json {
        let summaries = packs.iter().map(PackSummary::from).collect::<Vec<_>>();
        println!("{}", serde_json::to_string_pretty(&summaries)?);
        return Ok(());
    }

    if packs.is_empty() {
        println!("No project packs found.");
        return Ok(());
    }

    println!("Available project packs:");
    println!();
    for pack in packs {
        println!("{}", pack.manifest.id);
        println!("  {}", pack.manifest.description);
        println!("  Version: {}", pack.manifest.version);
        println!(
            "  Languages: {}",
            pack.manifest
                .languages
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!("  Stages: {}", pack.manifest.stages.len());
        println!("  Source: {}", pack.root.display());
        println!();
    }
    Ok(())
}

fn show(project: &str, json: bool, options: &GlobalOptions) -> Result<()> {
    let pack = load_pack(project, &pack_options(options))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&pack.manifest)?);
        return Ok(());
    }

    println!("{} ({})", pack.manifest.name, pack.manifest.id);
    println!("{}", pack.manifest.description);
    println!("Version: {}", pack.manifest.version);
    println!("Source: {}", pack.root.display());
    println!();
    println!("Stages:");
    for stage in &pack.manifest.stages {
        println!("  {} - {}", stage.id, stage.title);
    }
    Ok(())
}

fn install(
    project: &str,
    destination_root: &Path,
    force: bool,
    options: &GlobalOptions,
) -> Result<()> {
    let pack = load_pack(project, &pack_options(options))?;
    let destination = destination_root.join(&pack.manifest.id);
    if destination.exists() {
        if !force {
            bail!(
                "pack destination already exists: {}\nUse --force to replace it.",
                destination.display()
            );
        }
        fs::remove_dir_all(&destination).with_context(|| {
            format!(
                "failed to remove existing pack directory {}",
                destination.display()
            )
        })?;
    }
    fs::create_dir_all(destination_root).with_context(|| {
        format!(
            "failed to create destination packs directory {}",
            destination_root.display()
        )
    })?;
    copy_dir_recursive(&pack.root, &destination)?;
    println!(
        "Installed pack {} to {}",
        pack.manifest.id,
        destination.display()
    );
    Ok(())
}

fn pack_options(options: &GlobalOptions) -> PackSearchOptions {
    PackSearchOptions {
        packs_dir: options.packs_dir.clone(),
    }
}

fn print_report(report: &AuthoringReport, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(report)?);
        return Ok(());
    }

    println!("status: {}", report.status);
    if let Some(pack) = &report.pack {
        println!("pack: {pack}");
    }
    if let Some(path) = &report.path {
        println!("path: {path}");
    }
    if !report.problems.is_empty() {
        println!();
        println!("problems:");
        for problem in &report.problems {
            println!("  - {problem}");
        }
    }
    if !report.next_actions.is_empty() {
        println!();
        println!("next actions:");
        for action in &report.next_actions {
            println!("  - {action}");
        }
    }
    Ok(())
}

#[derive(Debug, Serialize)]
struct PackSummary {
    id: String,
    name: String,
    version: String,
    description: String,
    languages: Vec<String>,
    stages: usize,
    source: String,
}

impl From<&LoadedPack> for PackSummary {
    fn from(pack: &LoadedPack) -> Self {
        Self {
            id: pack.manifest.id.clone(),
            name: pack.manifest.name.clone(),
            version: pack.manifest.version.clone(),
            description: pack.manifest.description.clone(),
            languages: pack.manifest.languages.keys().cloned().collect(),
            stages: pack.manifest.stages.len(),
            source: pack.root.display().to_string(),
        }
    }
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    if !source.is_dir() {
        bail!("source pack directory does not exist: {}", source.display());
    }
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create directory {}", destination.display()))?;
    for entry in fs::read_dir(source)
        .with_context(|| format!("failed to read directory {}", source.display()))?
    {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type()?;
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
