use anyhow::{Context, Result, bail};

use crate::cli::TestArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::learning_web::{
    InitialView, generate_learning_page, open_learning_page, should_use_browser,
};
use crate::runner;
use crate::state::LastFailedTest;
use crate::test_web::generate_test_report;
use crate::viewer;

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

    let tests_failed = summaries.iter().any(|summary| !summary.is_success());
    let browser_disabled = std::env::var_os("DELTAFORGE_NO_BROWSER").is_some();
    let report_capable = !args.json && !args.list_tests && !args.terminal;
    if report_capable {
        // Regenerate on every run, not only failing ones, so a connected
        // live viewer tab always shows the latest result.
        let initial_stage = summaries
            .iter()
            .find(|summary| summary.failed > 0)
            .or_else(|| summaries.first())
            .map(|summary| summary.stage_id.as_str())
            .unwrap_or(&context.state.current_stage);
        let overview = super::overview::read_pack_overview(&context.pack);
        generate_learning_page(&context, &overview, InitialView::Stage(initial_stage))?;
        let report = generate_test_report(&context, &summaries)?;
        let ui_dir = context.root.join(".deltaforge/ui");
        let should_open =
            !browser_disabled && (args.open || (tests_failed && should_use_browser(false)));
        if should_open {
            match viewer::open_live(&ui_dir, "test-report.html") {
                Ok(viewer::LiveOpen::OpenedTab(url)) => {
                    println!("Opened the test report in your browser: {url}");
                }
                Ok(viewer::LiveOpen::Updated(url)) => {
                    println!("Live test report updated: {url}");
                }
                Err(_) => match open_learning_page(&report) {
                    Ok(()) => println!("Opened the test report in your browser."),
                    Err(error) => {
                        eprintln!("warning: {error:#}; open the generated report manually");
                        println!("Test report: {}", report.display());
                    }
                },
            }
        } else {
            let _ = viewer::bump_version(&ui_dir, Some("test-report.html"));
            if args.open || (tests_failed && !browser_disabled) {
                println!("Test report: {}", report.display());
            } else if let Some(status) = viewer::live_status(&ui_dir)
                && status.clients > 0
            {
                println!("Live test report updated: {}test-report.html", status.url);
            }
        }
    }

    if tests_failed {
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
