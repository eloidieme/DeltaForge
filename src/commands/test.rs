use anyhow::{Result, bail};

use crate::application::{self, RunEvent, RunTrigger, TestRunRequest};
use crate::cli::TestArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::learning_web::{
    InitialView, generate_learning_page, open_learning_page, should_use_browser,
};
use crate::test_web::generate_test_report;
use crate::viewer;

pub fn run(args: TestArgs, options: &GlobalOptions) -> Result<()> {
    let json = args.json;
    let verbose = args.verbose;
    let list_tests = args.list_tests;
    let open = args.open;
    let terminal = args.terminal;
    let all = args.all;

    let mut sink = |event: RunEvent| {
        if json {
            return;
        }
        match event {
            RunEvent::JobStarted { stage_ids, .. } => {
                println!("Checking {}", stage_ids.join(", "));
                println!();
            }
            RunEvent::BuildStarted { command } if verbose => {
                println!("Build: {}", command.join(" "));
            }
            RunEvent::BuildOutput { stream, text } if verbose => {
                println!("{stream}:\n{text}");
            }
            RunEvent::TestPassed { result, .. } => {
                println!("PASS  {}", result.name);
                if verbose {
                    print_streams(&result);
                }
                if let Some(path) = &result.kept_temp_dir {
                    println!("Kept temp dir: {}", path.display());
                }
            }
            RunEvent::TestFailed { result, .. } => {
                println!("FAIL  {}", result.name);
                for failure in &result.failures {
                    println!("  {failure}");
                }
                if !verbose && !result.stdout.is_empty() {
                    println!("  actual stdout:");
                    for line in result.stdout.lines().take(20) {
                        println!("    {line}");
                    }
                }
                if verbose {
                    print_streams(&result);
                }
                if let Some(path) = &result.kept_temp_dir {
                    println!("Kept temp dir: {}", path.display());
                }
            }
            RunEvent::RunCompleted {
                passed_tests,
                failed_tests,
                ..
            } => {
                println!();
                println!("{passed_tests} passed, {failed_tests} failed");
            }
            _ => {}
        }
    };

    let outcome = application::run_tests(
        options,
        TestRunRequest {
            stage: args.stage,
            all,
            filter: args.filter,
            list_tests,
            fail_fast: args.fail_fast,
            no_build: args.no_build,
            keep_temp: args.keep_temp,
            capture_details: !json && !terminal,
            trigger: RunTrigger::Cli,
        },
        &mut sink,
    )?;

    if list_tests && !json {
        for summary in &outcome.summaries {
            println!("{}:", summary.stage_id);
            for result in &summary.results {
                println!("  {}", result.name);
            }
        }
    }

    let tests_failed = !outcome.is_success();
    let browser_disabled = std::env::var_os("DELTAFORGE_NO_BROWSER").is_some();
    let report_capable = !json && !list_tests && !terminal;
    if report_capable {
        let context = ProjectContext::load(options)?;
        let initial_stage = outcome
            .summaries
            .iter()
            .find(|summary| summary.failed > 0)
            .or_else(|| outcome.summaries.first())
            .map(|summary| summary.stage_id.as_str())
            .unwrap_or(&context.state.current_stage);
        let overview = super::overview::read_pack_overview(&context.pack);
        generate_learning_page(&context, &overview, InitialView::Stage(initial_stage))?;
        let report = generate_test_report(&context, &outcome.summaries)?;
        let ui_dir = context.root.join(".deltaforge/ui");
        let should_open =
            !browser_disabled && (open || (tests_failed && should_use_browser(false)));
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
            if open || (tests_failed && !browser_disabled) {
                println!("Test report: {}", report.display());
            } else if let Some(status) = viewer::live_status(&ui_dir)
                && status.clients > 0
            {
                println!("Live test report updated: {}test-report.html", status.url);
            }
        }
    }

    if outcome.newly_completed_current && !tests_failed {
        let context = ProjectContext::load(options)?;
        if context.config.git.auto_commit {
            super::commit::run_automatic(options, json)?;
        }
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&outcome)?);
    }

    if let Some(error) = outcome.execution_error {
        bail!("{error}");
    }
    if tests_failed {
        bail!("tests failed");
    }

    Ok(())
}

fn print_streams(result: &crate::runner::TestResult) {
    if !result.stdout.is_empty() {
        println!("stdout:\n{}", result.stdout);
    }
    if !result.stderr.is_empty() {
        println!("stderr:\n{}", result.stderr);
    }
}
