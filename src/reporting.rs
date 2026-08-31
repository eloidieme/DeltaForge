//! The engineering record a learner can export.
//!
//! Every line this module emits traces to something DeltaForge actually
//! recorded: a completion proof, a saved measurement, a gate result, a commit,
//! or a note the learner wrote. Nothing here offers generic advice, and no
//! claim appears without the evidence behind it.

use std::collections::BTreeMap;
use std::process::Command;

use anyhow::Result;
use serde::Serialize;

use crate::benchmarks::{BenchmarkRecord, history_path, read_history};
use crate::context::{GateStatus, ProjectContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ReportFormat {
    Markdown,
    Html,
    Json,
}

impl ReportFormat {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Markdown => "md",
            Self::Html => "html",
            Self::Json => "json",
        }
    }

    /// File name the browser export writes into the project. These names are
    /// excluded from the project digest by `ProjectContext::project_digest`;
    /// changing one means changing both.
    pub fn export_file_name(self) -> String {
        format!("deltaforge-report.{}", self.extension())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectReport {
    pub project: String,
    pub pack_name: String,
    pub pack_version: String,
    pub language: String,
    pub created_at: String,
    pub updated_at: String,
    pub steps_total: usize,
    pub steps_complete: usize,
    /// Behavioral checks passing across all completed steps, summed from the
    /// completion proofs rather than from the last run.
    pub checks_proven: usize,
    pub steps: Vec<StepRecord>,
    pub measurements: Vec<MeasurementRecord>,
    pub gates: Vec<GateRecordView>,
    pub snapshots: Vec<SnapshotRecord>,
    pub notes: Vec<NoteRecord>,
    pub environment: Environment,
}

#[derive(Debug, Clone, Serialize)]
pub struct StepRecord {
    pub id: String,
    pub title: String,
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Checks the completion proof recorded for this step. `None` means the
    /// step passed before proofs existed, or has not passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gate: Option<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MeasurementRecord {
    pub stage_id: String,
    pub benchmark: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    pub machine: BTreeMap<String, String>,
    pub points: Vec<MeasurementPoint>,
    /// Speedup derived from a `threads` matrix, when the benchmark has one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speedup: Option<MeasuredSpeedup>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MeasuredSpeedup {
    pub key: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MeasurementPoint {
    pub params_label: String,
    pub success: bool,
    pub iterations: u64,
    pub runtime_median_ms: Option<f64>,
    pub runtime_p95_ms: Option<f64>,
    pub throughput_mb_s: Option<f64>,
    pub peak_memory_mb: Option<f64>,
    /// Change in median runtime against the first saved run of the same point,
    /// which is what "faster than when I started" means here.
    pub percent_change_from_first: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GateRecordView {
    pub stage_id: String,
    pub name: String,
    pub metric: String,
    pub requirement: String,
    pub measured: f64,
    pub passed: bool,
    pub recorded_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotRecord {
    pub tag: String,
    pub commit: String,
    pub subject: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteRecord {
    pub stage_id: String,
    pub kind: &'static str,
    pub text: String,
    pub recorded_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Environment {
    pub os: String,
    pub arch: String,
}

/// Assemble the record from project state, saved measurements, and the Git
/// history. Reads only; never mutates the project.
pub fn build(context: &ProjectContext) -> Result<ProjectReport> {
    let history = read_history(&history_path(context)).unwrap_or_default();

    let steps = context
        .pack
        .manifest
        .stages
        .iter()
        .map(|stage| {
            let proof = context.state.completion_proofs.get(&stage.id);
            StepRecord {
                id: stage.id.clone(),
                title: stage.title.clone(),
                status: if context.state.is_completed(&stage.id) {
                    "complete"
                } else if stage.id == context.state.current_stage {
                    "current"
                } else {
                    "not started"
                },
                completed_at: context
                    .state
                    .completed_stage_timestamps
                    .get(&stage.id)
                    .cloned(),
                checks: proof.map(|proof| proof.test_count),
                gate: context
                    .gate_status(&stage.id)
                    .unwrap_or(None)
                    .map(GateStatus::label),
            }
        })
        .collect::<Vec<_>>();

    let checks_proven = steps
        .iter()
        .filter(|step| step.status == "complete")
        .filter_map(|step| step.checks)
        .sum();

    let measurements = latest_measurements(&history);

    let mut gates = Vec::new();
    for (stage_id, record) in &context.state.gate_results {
        for result in &record.results {
            gates.push(GateRecordView {
                stage_id: stage_id.clone(),
                name: result.name.clone(),
                metric: crate::benchmarks::metric_name(result.metric).to_string(),
                requirement: match result.bound {
                    crate::pack::GateBound::Min(value) => format!("at least {value}"),
                    crate::pack::GateBound::Max(value) => format!("at most {value}"),
                },
                measured: result.measured,
                passed: result.passed,
                recorded_at: record.timestamp.clone(),
            });
        }
    }

    let mut notes = Vec::new();
    for (stage_id, note) in &context.state.predictions {
        if !note.skipped {
            notes.push(NoteRecord {
                stage_id: stage_id.clone(),
                kind: "prediction",
                text: note.text.clone(),
                recorded_at: note.recorded_at.clone(),
            });
        }
    }
    for (stage_id, note) in &context.state.reflections {
        if !note.skipped {
            notes.push(NoteRecord {
                stage_id: stage_id.clone(),
                kind: "reflection",
                text: note.text.clone(),
                recorded_at: note.recorded_at.clone(),
            });
        }
    }
    for (stage_id, text) in read_design_notes(context) {
        notes.push(NoteRecord {
            stage_id,
            kind: "design note",
            text,
            recorded_at: String::new(),
        });
    }
    notes.sort_by(|left, right| left.stage_id.cmp(&right.stage_id));

    Ok(ProjectReport {
        project: context.state.project.clone(),
        pack_name: context.pack.manifest.name.clone(),
        pack_version: context.state.pack_version.clone(),
        language: context.state.language.clone(),
        created_at: context.state.created_at.clone(),
        updated_at: context.state.updated_at.clone(),
        steps_total: steps.len(),
        steps_complete: steps.iter().filter(|s| s.status == "complete").count(),
        checks_proven,
        steps,
        measurements,
        gates,
        snapshots: read_snapshots(context),
        notes,
        environment: Environment {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        },
    })
}

/// One entry per (stage, benchmark): the most recent saved run, with each
/// point compared against the earliest saved run of the same point.
fn latest_measurements(history: &[BenchmarkRecord]) -> Vec<MeasurementRecord> {
    let mut keys: Vec<(&str, &str)> = history
        .iter()
        .map(|record| (record.stage.as_str(), record.benchmark.as_str()))
        .collect();
    keys.sort_unstable();
    keys.dedup();

    keys.into_iter()
        .filter_map(|(stage, benchmark)| {
            let mut runs = history
                .iter()
                .filter(|record| record.stage == stage && record.benchmark == benchmark);
            let first = runs.next()?;
            let latest = history
                .iter()
                .rev()
                .find(|record| record.stage == stage && record.benchmark == benchmark)?;
            Some(MeasurementRecord {
                stage_id: stage.to_string(),
                benchmark: benchmark.to_string(),
                timestamp: latest.timestamp.clone(),
                git_commit: latest.git_commit.clone(),
                machine: latest.machine.clone(),
                speedup: crate::benchmarks::thread_speedup(&latest.points).map(|speedup| {
                    MeasuredSpeedup {
                        key: speedup.key,
                        value: speedup.value,
                    }
                }),
                points: latest
                    .points
                    .iter()
                    .map(|point| {
                        let baseline = first
                            .points
                            .iter()
                            .find(|candidate| candidate.params == point.params)
                            .and_then(|candidate| candidate.runtime_median_ms);
                        MeasurementPoint {
                            params_label: point.params_label(),
                            success: point.success,
                            iterations: point.iterations,
                            runtime_median_ms: point.runtime_median_ms,
                            runtime_p95_ms: point.runtime_p95_ms,
                            throughput_mb_s: point.throughput_mb_s,
                            peak_memory_mb: point.peak_memory_mb,
                            percent_change_from_first: match (baseline, point.runtime_median_ms) {
                                (Some(before), Some(now))
                                    if before.is_finite() && before != 0.0 && now.is_finite() =>
                                {
                                    Some((now - before) / before * 100.0)
                                }
                                _ => None,
                            },
                        }
                    })
                    .collect(),
            })
        })
        .collect()
}

/// Stage tags DeltaForge created, newest last, with the commit each names.
fn read_snapshots(context: &ProjectContext) -> Vec<SnapshotRecord> {
    let output = Command::new("git")
        .args([
            "for-each-ref",
            "--sort=creatordate",
            "--format=%(refname:short)%09%(objectname:short)%09%(creatordate:short)%09%(contents:subject)",
            "refs/tags/deltaforge-*",
        ])
        .current_dir(&context.root)
        .output();
    let Ok(output) = output else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut fields = line.split('\t');
            Some(SnapshotRecord {
                tag: fields.next()?.to_string(),
                commit: fields.next()?.to_string(),
                date: fields.next()?.to_string(),
                subject: fields.next().unwrap_or_default().to_string(),
            })
        })
        .collect()
}

fn read_design_notes(context: &ProjectContext) -> Vec<(String, String)> {
    let notes_dir = context.root.join(".deltaforge").join("design_notes");
    let Ok(entries) = std::fs::read_dir(notes_dir) else {
        return Vec::new();
    };
    let mut notes = Vec::new();
    for entry in entries.flatten() {
        if entry.file_type().is_ok_and(|kind| kind.is_file())
            && let Ok(text) = std::fs::read_to_string(entry.path())
            && !text.trim().is_empty()
        {
            let stage = entry
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("unknown")
                .to_string();
            notes.push((stage, text.trim().to_string()));
        }
    }
    notes.sort();
    notes
}

pub fn render(report: &ProjectReport, format: ReportFormat) -> Result<String> {
    Ok(match format {
        ReportFormat::Markdown => render_markdown(report),
        ReportFormat::Html => render_html(report),
        ReportFormat::Json => serde_json::to_string_pretty(report)?,
    })
}

pub fn render_markdown(report: &ProjectReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {} engineering record\n\n", report.pack_name));
    out.push_str(&format!(
        "Built in {} against {} {}. Started {}, last worked on {}.\n\n",
        report.language,
        report.pack_name,
        report.pack_version,
        report.created_at,
        report.updated_at
    ));

    out.push_str("## What is proven\n\n");
    out.push_str(&format!(
        "- {} of {} steps complete.\n",
        report.steps_complete, report.steps_total
    ));
    if report.checks_proven > 0 {
        out.push_str(&format!(
            "- {} behavioral checks passing against the source as it stands.\n",
            report.checks_proven
        ));
    }
    let gates_passed = report.gates.iter().filter(|gate| gate.passed).count();
    if !report.gates.is_empty() {
        out.push_str(&format!(
            "- {} of {} performance targets met.\n",
            gates_passed,
            report.gates.len()
        ));
    }
    if !report.snapshots.is_empty() {
        out.push_str(&format!(
            "- {} step snapshots recorded in Git history.\n",
            report.snapshots.len()
        ));
    }
    out.push('\n');

    out.push_str("## Steps\n\n");
    out.push_str("| Step | Status | Completed | Checks | Performance |\n");
    out.push_str("| --- | --- | --- | ---: | --- |\n");
    for step in &report.steps {
        out.push_str(&format!(
            "| `{}` {} | {} | {} | {} | {} |\n",
            step.id,
            step.title,
            step.status,
            step.completed_at.as_deref().unwrap_or("-"),
            step.checks
                .map_or_else(|| "-".to_string(), |n| n.to_string()),
            step.gate.map_or("-", |gate| gate).replace('_', " "),
        ));
    }
    out.push('\n');

    out.push_str("## Measurements\n\n");
    if report.measurements.is_empty() {
        out.push_str("No benchmark run has been saved yet.\n\n");
    } else {
        for measurement in &report.measurements {
            out.push_str(&format!(
                "### `{}` / {}\n\n",
                measurement.stage_id, measurement.benchmark
            ));
            out.push_str(&format!(
                "Measured {} on {} {}",
                measurement.timestamp,
                measurement.machine.get("os").map_or("-", String::as_str),
                measurement.machine.get("arch").map_or("-", String::as_str),
            ));
            if let Some(commit) = &measurement.git_commit {
                out.push_str(&format!(
                    ", at commit `{}`",
                    &commit[..commit.len().min(12)]
                ));
            }
            out.push_str(".\n\n");
            out.push_str("| Parameters | Iterations | Median ms | P95 ms | MB/s | Peak MB | Change since first run |\n");
            out.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
            for point in &measurement.points {
                out.push_str(&format!(
                    "| {} | {} | {} | {} | {} | {} | {} |\n",
                    if point.params_label.is_empty() {
                        "-"
                    } else {
                        &point.params_label
                    },
                    point.iterations,
                    optional(point.runtime_median_ms),
                    optional(point.runtime_p95_ms),
                    optional(point.throughput_mb_s),
                    optional(point.peak_memory_mb),
                    point
                        .percent_change_from_first
                        .map_or_else(|| "-".to_string(), |change| format!("{change:+.1}%")),
                ));
            }
            if let Some(speedup) = &measurement.speedup {
                out.push_str(&format!(
                    "\nDerived {}: {:.2}x.\n",
                    speedup.key.replace('_', " "),
                    speedup.value
                ));
            }
            out.push('\n');
        }
    }

    if !report.gates.is_empty() {
        out.push_str("## Performance targets\n\n");
        out.push_str("| Step | Target | Required | Measured | Met | Recorded |\n");
        out.push_str("| --- | --- | --- | ---: | --- | --- |\n");
        for gate in &report.gates {
            out.push_str(&format!(
                "| `{}` | {} | {} {} | {:.2} | {} | {} |\n",
                gate.stage_id,
                gate.name,
                gate.metric,
                gate.requirement,
                gate.measured,
                if gate.passed { "yes" } else { "not yet" },
                gate.recorded_at,
            ));
        }
        out.push('\n');
    }

    if !report.snapshots.is_empty() {
        out.push_str("## Snapshots\n\n");
        for snapshot in &report.snapshots {
            out.push_str(&format!(
                "- `{}` `{}` {} — {}\n",
                snapshot.commit, snapshot.tag, snapshot.date, snapshot.subject
            ));
        }
        out.push('\n');
    }

    if !report.notes.is_empty() {
        out.push_str("## Notes\n\n");
        for note in &report.notes {
            out.push_str(&format!("### `{}` — {}\n\n", note.stage_id, note.kind));
            out.push_str(note.text.trim());
            out.push_str("\n\n");
        }
    }

    out.push_str("## Environment\n\n");
    out.push_str(&format!(
        "- {} on {}\n",
        report.environment.os, report.environment.arch
    ));
    out
}

/// A short narrative built from the same evidence, for a portfolio or a
/// README. Every sentence names a number the project actually recorded.
pub fn render_summary(report: &ProjectReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", report.pack_name));
    out.push_str(&format!(
        "A {} implementation of {}, built one behavior at a time.\n\n",
        report.language, report.pack_name
    ));

    out.push_str("## What it does\n\n");
    let completed = report
        .steps
        .iter()
        .filter(|step| step.status == "complete")
        .collect::<Vec<_>>();
    if completed.is_empty() {
        out.push_str("No step has passed its checks yet.\n\n");
    } else {
        for step in &completed {
            out.push_str(&format!("- {}\n", step.title));
        }
        out.push('\n');
    }

    out.push_str("## Evidence\n\n");
    out.push_str(&format!(
        "- {} of {} steps complete.\n",
        report.steps_complete, report.steps_total
    ));
    if report.checks_proven > 0 {
        out.push_str(&format!(
            "- {} behavioral checks pass against the current source.\n",
            report.checks_proven
        ));
    }
    for gate in report.gates.iter().filter(|gate| gate.passed) {
        out.push_str(&format!(
            "- {} met on `{}`: {} {}, measured {:.2}.\n",
            gate.name, gate.stage_id, gate.metric, gate.requirement, gate.measured
        ));
    }
    for measurement in &report.measurements {
        for point in measurement
            .points
            .iter()
            .filter(|point| point.success && point.runtime_median_ms.is_some())
        {
            let label = if point.params_label.is_empty() {
                String::new()
            } else {
                format!(" ({})", point.params_label)
            };
            out.push_str(&format!(
                "- `{}`{}: median {:.2} ms over {} iterations",
                measurement.benchmark,
                label,
                point.runtime_median_ms.unwrap_or_default(),
                point.iterations
            ));
            if let Some(throughput) = point.throughput_mb_s {
                out.push_str(&format!(", {throughput:.2} MB/s"));
            }
            if let Some(change) = point.percent_change_from_first
                && change.abs() >= 1.0
            {
                out.push_str(&format!(
                    ", {:.0}% {} than the first saved run",
                    change.abs(),
                    if change < 0.0 { "faster" } else { "slower" }
                ));
            }
            out.push_str(".\n");
        }
    }
    out.push('\n');

    if let Some(remaining) = report
        .steps
        .iter()
        .find(|step| step.status == "current")
        .filter(|_| report.steps_complete < report.steps_total)
    {
        out.push_str("## In progress\n\n");
        out.push_str(&format!("- {} ({})\n", remaining.title, remaining.id));
    }
    out
}

pub fn render_html(report: &ProjectReport) -> String {
    format!(
        concat!(
            "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">",
            "<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">",
            "<title>{} engineering record</title>",
            "<style>",
            ":root{{color-scheme:light dark}}",
            "body{{font-family:ui-sans-serif,system-ui,-apple-system,\"Segoe UI\",sans-serif;",
            "line-height:1.6;max-width:56rem;margin:0 auto;padding:3rem 1.5rem}}",
            "pre{{white-space:pre-wrap;overflow-x:auto}}",
            "</style></head><body><pre>{}</pre></body></html>"
        ),
        html_escape(&report.pack_name),
        html_escape(&render_markdown(report))
    )
}

fn optional(value: Option<f64>) -> String {
    value.map_or_else(|| "-".to_string(), |value| format!("{value:.2}"))
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_report() -> ProjectReport {
        ProjectReport {
            project: "flashindex".to_string(),
            pack_name: "FlashIndex".to_string(),
            pack_version: "2.0.0".to_string(),
            language: "rust".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-02T00:00:00Z".to_string(),
            steps_total: 2,
            steps_complete: 1,
            checks_proven: 9,
            steps: vec![
                StepRecord {
                    id: "01_scan_files".to_string(),
                    title: "Scan files".to_string(),
                    status: "complete",
                    completed_at: Some("2026-01-02T00:00:00Z".to_string()),
                    checks: Some(9),
                    gate: None,
                },
                StepRecord {
                    id: "02_filter_files".to_string(),
                    title: "Choose searchable files".to_string(),
                    status: "current",
                    completed_at: None,
                    checks: None,
                    gate: None,
                },
            ],
            measurements: Vec::new(),
            gates: Vec::new(),
            snapshots: Vec::new(),
            notes: Vec::new(),
            environment: Environment {
                os: "linux".to_string(),
                arch: "x86_64".to_string(),
            },
        }
    }

    #[test]
    fn a_report_states_only_what_was_recorded() {
        let rendered = render_markdown(&empty_report());
        assert!(rendered.contains("1 of 2 steps complete."));
        assert!(rendered.contains("9 behavioral checks passing"));
        assert!(rendered.contains("No benchmark run has been saved yet."));
        // The old report emitted fixed advice regardless of what happened.
        assert!(!rendered.contains("Profile benchmark hot paths"));
        assert!(!rendered.contains("Future Improvements"));
        // No performance section without a recorded gate.
        assert!(!rendered.contains("## Performance targets"));
    }

    #[test]
    fn a_summary_without_measurements_claims_no_performance() {
        let summary = render_summary(&empty_report());
        assert!(summary.contains("- Scan files"));
        assert!(summary.contains("9 behavioral checks pass"));
        assert!(!summary.contains("median"));
        assert!(summary.contains("## In progress"));
    }

    #[test]
    fn measured_speedup_and_change_are_reported_with_their_evidence() {
        let mut report = empty_report();
        report.measurements.push(MeasurementRecord {
            stage_id: "12_parallel_performance".to_string(),
            benchmark: "index_with_threads".to_string(),
            timestamp: "2026-02-01T00:00:00Z".to_string(),
            git_commit: Some("abcdef1234567890".to_string()),
            machine: BTreeMap::from([
                ("os".to_string(), "linux".to_string()),
                ("arch".to_string(), "x86_64".to_string()),
            ]),
            speedup: Some(MeasuredSpeedup {
                key: "speedup_1_to_8".to_string(),
                value: 2.5,
            }),
            points: vec![MeasurementPoint {
                params_label: "threads=8".to_string(),
                success: true,
                iterations: 4,
                runtime_median_ms: Some(40.0),
                runtime_p95_ms: Some(44.0),
                throughput_mb_s: Some(12.5),
                peak_memory_mb: Some(30.0),
                percent_change_from_first: Some(-60.0),
            }],
        });

        let rendered = render_markdown(&report);
        assert!(rendered.contains("at commit `abcdef123456`"));
        assert!(rendered.contains("-60.0%"));
        assert!(rendered.contains("Derived speedup 1 to 8: 2.50x."));

        let summary = render_summary(&report);
        assert!(summary.contains("median 40.00 ms over 4 iterations"));
        assert!(summary.contains("60% faster than the first saved run"));
    }
}
