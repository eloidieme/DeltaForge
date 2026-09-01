use anyhow::{Context, Result};
use serde::Serialize;

use crate::cli::ExplainFailureArgs;
use crate::context::{GlobalOptions, ProjectContext};

pub fn run(args: ExplainFailureArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let stage_id = args
        .stage
        .as_deref()
        .unwrap_or(&context.state.current_stage);
    let run = context
        .state
        .last_test_runs
        .get(stage_id)
        .with_context(|| {
            format!("no test run recorded for stage {stage_id}; run `deltaforge test` first")
        })?;
    let stage = context
        .pack
        .manifest
        .stage(stage_id)
        .with_context(|| format!("pack does not contain stage {stage_id}"))?;
    // The recorded diagnosis describes the project as it was at the last run,
    // not necessarily as it is now: an edit since then can already have fixed
    // (or introduced) the failure this would otherwise confidently restate.
    let stale = run.project_digest != context.project_digest()?;

    let explanation = FailureExplanation {
        stage_id: stage.id.clone(),
        stage_title: stage.title.clone(),
        passed: run.passed,
        failed: run.failed,
        timestamp: run.timestamp.clone(),
        stale,
        failed_tests: run.failed_tests.clone(),
        suggestions: suggestions(run),
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&explanation)?);
    } else {
        println!(
            "Stage {}: {}",
            explanation.stage_id, explanation.stage_title
        );
        if explanation.stale {
            println!("This result is from before your last edit — run `deltaforge test` again.");
        }
        println!(
            "Last run: {} passed, {} failed at {}",
            explanation.passed, explanation.failed, explanation.timestamp
        );
        println!();
        if explanation.failed == 0 {
            println!("No failures recorded for this stage.");
        } else {
            let primary = explanation.failed_tests.iter().min_by_key(|failure| {
                failure
                    .diagnosis
                    .as_ref()
                    .map_or(u32::MAX, |diagnosis| diagnosis.priority)
            });
            if let Some(failed_test) = primary {
                println!("Fix this first: {}", failed_test.name);
                if let Some(diagnosis) = &failed_test.diagnosis {
                    println!("{}", diagnosis.headline);
                    println!();
                    println!("{}", diagnosis.summary);
                    println!("Requirement: {}", diagnosis.contract);
                    if let Some(expected) = &diagnosis.expected {
                        println!("Expected: {expected}");
                    }
                    if let Some(actual) = &diagnosis.actual {
                        println!("Actual: {actual}");
                    }
                    let fixture = diagnosis
                        .fixture
                        .iter()
                        .chain(diagnosis.fixture_entries.iter())
                        .cloned()
                        .collect::<Vec<_>>();
                    if !fixture.is_empty() {
                        println!("Fixture: {}", fixture.join(" · "));
                    }
                } else {
                    for failure in &failed_test.failures {
                        println!("  - {failure}");
                    }
                }
            }
            let other_count = explanation.failed_tests.len().saturating_sub(1);
            if other_count > 0 {
                println!();
                println!("{other_count} other failing check(s):");
                for failed_test in &explanation.failed_tests {
                    if primary.is_some_and(|primary| std::ptr::eq(primary, failed_test)) {
                        continue;
                    }
                    let label = failed_test
                        .diagnosis
                        .as_ref()
                        .map_or(failed_test.name.as_str(), |diagnosis| {
                            diagnosis.headline.as_str()
                        });
                    println!("  - {label}");
                }
            }
            println!();
            println!("Suggested next steps:");
            for suggestion in &explanation.suggestions {
                println!("  - {suggestion}");
            }
        }
    }

    Ok(())
}

fn suggestions(run: &crate::state::LastTestRunSummary) -> Vec<String> {
    if run.failed == 0 {
        return vec!["Run `deltaforge next` to continue.".to_string()];
    }

    let mut suggestions = Vec::new();
    for failed_test in &run.failed_tests {
        for failure in &failed_test.failures {
            if failure.contains("stdout to contain") {
                suggestions.push("Compare your program output with the expected lines and check ordering/path formatting.".to_string());
            } else if failure.contains("stdout exactly") {
                suggestions.push("Check extra whitespace, trailing lines, and whether the command should print nothing.".to_string());
            } else if failure.contains("exit code") {
                suggestions.push("Make sure the command returns success for valid inputs and failure only for real errors.".to_string());
            } else if failure.contains("file to exist") {
                suggestions.push(
                    "Verify the command writes output files under the provided temp/output path."
                        .to_string(),
                );
            } else if failure.contains("timed out") {
                suggestions.push(
                    "Look for infinite loops or unexpectedly slow scans before optimizing details."
                        .to_string(),
                );
            }
        }
    }

    suggestions.sort();
    suggestions.dedup();
    if suggestions.is_empty() {
        suggestions.push(
            "Run `deltaforge test --verbose` to inspect stdout and stderr for the failing command."
                .to_string(),
        );
    }
    suggestions
}

#[derive(Debug, Serialize)]
struct FailureExplanation {
    stage_id: String,
    stage_title: String,
    passed: usize,
    failed: usize,
    timestamp: String,
    /// True when the project has changed since this run, so the diagnosis
    /// below may no longer describe the current source.
    stale: bool,
    failed_tests: Vec<crate::state::LastFailedTest>,
    suggestions: Vec<String>,
}
