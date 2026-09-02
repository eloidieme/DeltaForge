use anyhow::{Result, bail};

use crate::application::{self, RunEvent, RunTrigger, TestRunRequest};
use crate::cli::TestArgs;
use crate::context::{GlobalOptions, ProjectContext};

pub fn run(args: TestArgs, options: &GlobalOptions) -> Result<()> {
    let json = args.json;
    let verbose = args.verbose;
    let list_tests = args.list_tests;
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
                if !verbose {
                    // The redacted form (fixture/temp paths replaced with
                    // `{fixture_path}`) so this block matches the `failures`
                    // lines above it instead of contradicting them; `--verbose`
                    // shows the raw streams via `print_streams` instead.
                    let shown = result.report_stdout.as_deref().unwrap_or(&result.stdout);
                    if !shown.is_empty() {
                        println!("  actual stdout:");
                        for line in shown.lines().take(20) {
                            println!("    {line}");
                        }
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

    if outcome.newly_completed_current && !tests_failed {
        let context = ProjectContext::load(options)?;
        if context.config.git.auto_commit
            && let Err(error) = super::commit::run_automatic(options, json)
        {
            // Best-effort: the checks genuinely passed, so the exit code must
            // say so. A failed automatic snapshot (a pre-commit hook, a
            // missing Git identity, a full disk) is reported but does not
            // turn a green run into a failing one; `deltaforge commit` is
            // always available to retry it by hand.
            eprintln!("warning: automatic snapshot after passing this step failed: {error:#}");
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
