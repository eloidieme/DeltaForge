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
    for stage in &stages {
        let summary = runner::run_stage_tests(&context, stage, &args)?;
        if !args.list_tests && summary.is_success() {
            context.state.mark_completed(&stage.id)?;
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

    if args.json {
        println!("{}", serde_json::to_string_pretty(&summaries)?);
    }

    if summaries.iter().any(|summary| !summary.is_success()) {
        bail!("tests failed");
    }

    Ok(())
}
