use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;

use crate::cli::DoctorArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::pack::{PackSearchOptions, discover_packs_with_options, validate_pack};

pub fn run(args: DoctorArgs, options: &GlobalOptions) -> Result<()> {
    let discovery = discover_packs_with_options(&PackSearchOptions {
        packs_dir: options.packs_dir.clone(),
    })?;
    // Tools come from what the discovered packs declare, so adding a language
    // does not require changing this command. Git is checked separately:
    // DeltaForge itself uses it for stage snapshots.
    let mut tools: Vec<crate::creation::ToolStatus> = Vec::new();
    for pack in &discovery.packs {
        for language in pack.manifest.languages.values() {
            for tool in crate::creation::language_tools(language) {
                if !tools.iter().any(|seen| seen.program == tool.program) {
                    tools.push(tool);
                }
            }
        }
    }
    if !tools.iter().any(|tool| tool.program == "git") {
        let version = crate::creation::tool_version("git");
        tools.push(crate::creation::ToolStatus {
            program: "git".to_string(),
            label: "Git".to_string(),
            found: version.is_some(),
            version,
            install_url: Some("https://git-scm.com/downloads".to_string()),
            required: false,
        });
    }
    tools.sort_by(|left, right| left.program.cmp(&right.program));
    let workspace = crate::creation::default_workspace()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|error| format!("unavailable: {error:#}"));
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
        tools,
        workspace,
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
        for tool in &report.tools {
            match &tool.version {
                Some(version) => println!("{}: {version}", tool.program),
                None if tool.required => println!("{}: not found (required)", tool.program),
                None => println!("{}: not found", tool.program),
            }
        }
        println!("new projects: {}", report.workspace);
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

#[derive(Debug, Serialize)]
struct DoctorReport {
    tools: Vec<crate::creation::ToolStatus>,
    /// Where the browser creation flow puts new projects by default.
    workspace: String,
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
