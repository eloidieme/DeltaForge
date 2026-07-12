use anyhow::Result;
use serde::Serialize;

use crate::cli::StatusArgs;
use crate::context::{GlobalOptions, ProjectContext};

pub fn run(args: StatusArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;

    let stages = context
        .pack
        .manifest
        .stages
        .iter()
        .map(|stage| {
            Ok(StatusStage {
                id: stage.id.clone(),
                title: stage.title.clone(),
                status: stage_status(&context, &stage.id)?,
                performance: context.gate_status(&stage.id)?.map(|status| status.label()),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    if args.json {
        let report = StatusReport {
            project: context.state.project.clone(),
            language: context.state.language.clone(),
            current_stage: context.state.current_stage.clone(),
            stages,
        };
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("Project: {}", context.pack.manifest.name);
    println!("Language: {}", context.state.language);
    println!("Current stage: {}", context.state.current_stage);
    println!();

    println!("Stages:");
    let mut any_stale = false;
    for stage in &stages {
        let marker = match stage.status {
            "complete" => "✓",
            "needs_revalidation" => {
                any_stale = true;
                "!"
            }
            "current" => "→",
            _ => "○",
        };
        let performance = stage
            .performance
            .map(|value| format!(" - performance: {}", value.replace('_', " ")))
            .unwrap_or_default();
        println!("  {marker} {} - {}{performance}", stage.id, stage.title);
    }
    if any_stale {
        println!();
        println!("Stages marked ! passed against an older version of this pack.");
        println!("Run `deltaforge test --stage <id>` to revalidate them.");
    }

    Ok(())
}

fn stage_status(context: &ProjectContext, stage_id: &str) -> Result<&'static str> {
    if context.state.is_completed(stage_id) {
        if context.stage_needs_revalidation(stage_id)? {
            Ok("needs_revalidation")
        } else {
            Ok("complete")
        }
    } else if stage_id == context.state.current_stage {
        Ok("current")
    } else {
        Ok("locked")
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
    performance: Option<&'static str>,
}
