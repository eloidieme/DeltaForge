use anyhow::{Context, Result, bail};

use crate::cli::TestArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::runner;
use crate::state::LastFailedTest;

pub fn run(args: TestArgs, options: &GlobalOptions) -> Result<()> {
    let mut context = ProjectContext::load(options)?;
    let stages = if args.all {
        context.pack.manifest.stages.clone()
    } else {
        let stage_id = args
            .stage
            .as_deref()
            .unwrap_or(&context.state.current_stage);
        vec![
            context
                .pack
                .manifest
                .stage(stage_id)
                .with_context(|| format!("pack does not contain stage {stage_id}"))?
                .clone(),
        ]
    };

    let mut summaries = Vec::new();
    let mut newly_completed_current = false;
    for stage in &stages {
        let summary = runner::run_stage_tests(&context, stage, &args)?;
        if !args.list_tests && summary.completion_eligible {
            let was_completed = context.state.is_completed(&stage.id);
            context.state.record_completion_proof(
                &stage.id,
                context.pack_digest()?,
                context.stage_behavioral_digest(&stage.id)?,
                context.project_digest()?,
                summary.total_defined,
            )?;
            newly_completed_current |= !was_completed && stage.id == context.state.current_stage;
        }
        if !args.list_tests {
            context.state.record_test_run(
                stage.id.clone(),
                summary.passed,
                summary.failed,
                summary
                    .results
                    .iter()
                    .filter(|result| !result.passed)
                    .map(|result| LastFailedTest {
                        name: result.name.clone(),
                        failures: result.failures.clone(),
                    })
                    .collect(),
            )?;
        }
        summaries.push(summary);
    }

    if !args.list_tests {
        context.save_state()?;
    }

    if summaries.iter().any(|summary| !summary.is_success()) {
        if args.json {
            println!("{}", serde_json::to_string_pretty(&summaries)?);
        }
        bail!("tests failed");
    }

    if newly_completed_current && context.config.git.auto_commit {
        super::commit::run_automatic(options, args.json)?;
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&summaries)?);
    }

    Ok(())
}
