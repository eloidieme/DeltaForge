use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use serde::Serialize;

use crate::cli::DoctorArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::pack::{PackSearchOptions, discover_packs_with_options, validate_pack};

pub fn run(args: DoctorArgs, options: &GlobalOptions) -> Result<()> {
    if args.repair {
        let repaired = repair_project(options)?;
        if args.json {
            println!("{}", serde_json::to_string_pretty(&repaired)?);
        } else {
            println!("Restored saved progress from {}", repaired.restored_from);
            println!("Damaged state preserved at {}", repaired.damaged_copy);
        }
        return Ok(());
    }
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

#[derive(Debug, Serialize)]
struct RepairReport {
    status: &'static str,
    restored_from: String,
    damaged_copy: String,
    state_file: String,
}

fn repair_project(options: &GlobalOptions) -> Result<RepairReport> {
    let state_path = locate_state_file(options)?;
    match crate::state::ProjectState::read_from(&state_path) {
        Ok(_) => bail!(
            "saved progress in {} is readable; no repair is needed",
            state_path.display()
        ),
        Err(error) if format!("{error:#}").contains("newer version of DeltaForge") => {
            return Err(error)
                .context("repair refused because this state needs a newer DeltaForge");
        }
        Err(_) => {}
    }

    let previous = crate::state::previous_state_path(&state_path);
    if !previous.is_file() {
        bail!(
            "cannot repair saved progress: {} does not exist; restore the project from version control or another backup",
            previous.display()
        );
    }
    let restored = crate::state::ProjectState::read_from(&previous).with_context(|| {
        format!(
            "cannot repair saved progress because the previous state {} is not readable",
            previous.display()
        )
    })?;
    let serialized = serde_json::to_string_pretty(&restored)
        .context("failed to serialize the recovered project state")?;
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let damaged = state_path.with_file_name(format!("state.json.damaged-{suffix}"));
    fs::rename(&state_path, &damaged).with_context(|| {
        format!(
            "failed to preserve damaged state {} as {}",
            state_path.display(),
            damaged.display()
        )
    })?;
    if let Err(error) = crate::fs_util::atomic_write(&state_path, serialized) {
        let _ = fs::rename(&damaged, &state_path);
        return Err(error).context("failed to install the recovered project state");
    }
    Ok(RepairReport {
        status: "repaired",
        restored_from: previous.display().to_string(),
        damaged_copy: damaged.display().to_string(),
        state_file: state_path.display().to_string(),
    })
}

fn locate_state_file(options: &GlobalOptions) -> Result<PathBuf> {
    let start = options
        .project_dir
        .clone()
        .or_else(|| std::env::current_dir().ok())
        .context("could not locate the current directory")?;
    let start = start.canonicalize().unwrap_or(start);
    for candidate in start.ancestors() {
        let state = candidate.join(".deltaforge").join("state.json");
        if state.is_file() || crate::state::previous_state_path(&state).is_file() {
            return Ok(state);
        }
    }
    bail!(
        "no DeltaForge project state was found from {}; run repair from inside the project",
        start.display()
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repair_restores_the_previous_state_and_preserves_the_damaged_file() {
        let root = crate::fs_util::create_private_scratch_dir("deltaforge-doctor-repair").unwrap();
        let metadata = root.join(".deltaforge");
        fs::create_dir_all(&metadata).unwrap();
        let state_path = metadata.join("state.json");
        let mut state = crate::state::ProjectState::new(
            "flashindex".to_string(),
            "rust".to_string(),
            "01_scan_files".to_string(),
        )
        .unwrap();
        state.write_to(&state_path).unwrap();
        state.current_stage = "02_filter_files".to_string();
        state.write_to(&state_path).unwrap();
        fs::write(&state_path, "{ truncated").unwrap();

        let report = repair_project(&GlobalOptions {
            project_dir: Some(root.clone()),
            packs_dir: None,
        })
        .unwrap();

        assert_eq!(report.status, "repaired");
        assert!(std::path::Path::new(&report.damaged_copy).is_file());
        assert_eq!(
            crate::state::ProjectState::read_from(&state_path)
                .unwrap()
                .current_stage,
            "01_scan_files"
        );
        fs::remove_dir_all(root).unwrap();
    }
}
