use anyhow::Result;
use serde::Serialize;

use crate::cli::StatusArgs;
use crate::context::{GlobalOptions, ProjectContext};

pub fn run(args: StatusArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;

    if args.json {
        let report = StatusReport {
            project: context.state.project.clone(),
            language: context.state.language.clone(),
            current_stage: context.state.current_stage.clone(),
            stages: context
                .pack
                .manifest
                .stages
                .iter()
                .map(|stage| StatusStage {
                    id: stage.id.clone(),
                    title: stage.title.clone(),
                    status: stage_status(&context, &stage.id),
                })
                .collect(),
        };
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("Project: {}", context.pack.manifest.name);
    println!("Language: {}", context.state.language);
    println!("Current stage: {}", context.state.current_stage);
    println!();

    println!("Stages:");
    for stage in &context.pack.manifest.stages {
        let marker = if context.state.is_completed(&stage.id) {
            "✓"
        } else if stage.id == context.state.current_stage {
            "→"
        } else {
            "○"
        };
        println!("  {marker} {} - {}", stage.id, stage.title);
    }

    Ok(())
}

fn stage_status(context: &ProjectContext, stage_id: &str) -> &'static str {
    if context.state.is_completed(stage_id) {
        "complete"
    } else if stage_id == context.state.current_stage {
        "current"
    } else {
        "locked"
    }
}

#[derive(Debug, Serialize)]
struct StatusReport {
    project: String,
    language: String,
    current_stage: String,
    stages: Vec<StatusStage>,
}

#[derive(Debug, Serialize)]
struct StatusStage {
    id: String,
    title: String,
    status: &'static str,
}
