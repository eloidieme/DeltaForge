use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use serde::Serialize;

use crate::cli::DoctorArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::pack::{PackSearchOptions, discover_packs_with_options, validate_pack};

pub fn run(args: DoctorArgs, options: &GlobalOptions) -> Result<()> {
    let cargo = tool_version("cargo");
    let git = tool_version("git");
    let discovery = discover_packs_with_options(&PackSearchOptions {
        packs_dir: options.packs_dir.clone(),
    })?;
    let mut pack_results = discovery
        .packs
        .iter()
        .map(|pack| {
            let problems = validate_pack(pack);
            DoctorPack {
                id: pack.manifest.id.clone(),
                valid: problems.is_empty(),
                problems,
            }
        })
        .collect::<Vec<_>>();
    for problem in &discovery.problems {
        pack_results.push(DoctorPack {
            id: problem.path.display().to_string(),
            valid: false,
            problems: vec![problem.error.clone()],
        });
    }
    let (project, project_error) = match ProjectContext::load(options) {
        Ok(context) => (
            Some(DoctorProject {
                root: context.root.display().to_string(),
                project: context.state.project,
                language: context.state.language,
                current_stage: context.state.current_stage,
            }),
            None,
        ),
        Err(error) if project_state_exists(options) => (None, Some(format!("{error:#}"))),
        Err(_) => (None, None),
    };

    let report = DoctorReport {
        cargo,
        git,
        pack_count: pack_results.len(),
        packs: pack_results,
        project,
        project_error,
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("DeltaForge doctor");
        println!();
        print_tool("cargo", &report.cargo);
        print_tool("git", &report.git);
        println!("packs: {}", report.pack_count);
        for pack in &report.packs {
            let marker = if pack.valid { "ok" } else { "invalid" };
            println!("  {}: {}", pack.id, marker);
            for problem in &pack.problems {
                println!("    - {problem}");
            }
        }
        if let Some(project) = &report.project {
            println!("project: {}", project.root);
            println!("  pack: {}", project.project);
            println!("  language: {}", project.language);
            println!("  current stage: {}", project.current_stage);
        } else {
            println!("project: none detected");
            if let Some(error) = &report.project_error {
                println!("  error: {error}");
            }
        }
    }

    Ok(())
}

fn project_state_exists(options: &GlobalOptions) -> bool {
    let start = options
        .project_dir
        .clone()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    start
        .ancestors()
        .any(|path| path.join(".deltaforge/state.json").is_file())
}

fn tool_version(tool: &str) -> Option<String> {
    let output = Command::new(tool).arg("--version").output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn print_tool(name: &str, value: &Option<String>) {
    match value {
        Some(version) => println!("{name}: {version}"),
        None => println!("{name}: not found"),
    }
}

#[derive(Debug, Serialize)]
struct DoctorReport {
    cargo: Option<String>,
    git: Option<String>,
    pack_count: usize,
    packs: Vec<DoctorPack>,
    project: Option<DoctorProject>,
    project_error: Option<String>,
}

#[derive(Debug, Serialize)]
struct DoctorPack {
    id: String,
    valid: bool,
    problems: Vec<String>,
}

#[derive(Debug, Serialize)]
struct DoctorProject {
    root: String,
    project: String,
    language: String,
    current_stage: String,
}
