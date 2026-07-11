use std::fs;

use anyhow::Result;
use serde::Serialize;

use crate::cli::{ReportArgs, ReportFormat};
use crate::commands::bench::{history_path, read_history};
use crate::context::{GlobalOptions, ProjectContext};
use crate::fs_util::atomic_write;

pub fn run(args: ReportArgs, options: &GlobalOptions) -> Result<()> {
    let context = ProjectContext::load(options)?;
    let history = read_history(&history_path(&context))?;
    let design_notes = read_design_notes(&context)?;

    let output = match args.format {
        ReportFormat::Markdown => render_markdown(&context, &history, &design_notes),
        ReportFormat::Html => render_html(&context, &history, &design_notes),
        ReportFormat::Json => {
            serde_json::to_string_pretty(&render_json(&context, &history, &design_notes))?
        }
    };

    atomic_write(&args.output, output)?;
    println!("Wrote report: {}", args.output.display());
    Ok(())
}

fn render_markdown(
    context: &ProjectContext,
    history: &[crate::commands::bench::BenchmarkRecord],
    design_notes: &[(String, String)],
) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {} Report\n\n", context.pack.manifest.name));
    out.push_str("## Project Metadata\n\n");
    out.push_str(&format!("- Project: `{}`\n", context.state.project));
    out.push_str(&format!("- Language: `{}`\n", context.state.language));
    out.push_str(&format!(
        "- Current stage: `{}`\n",
        context.state.current_stage
    ));
    out.push_str(&format!("- Created: `{}`\n", context.state.created_at));
    out.push_str(&format!("- Updated: `{}`\n\n", context.state.updated_at));

    out.push_str("## Stage Progress\n\n");
    for stage in &context.pack.manifest.stages {
        let status = if context.state.is_completed(&stage.id) {
            "complete"
        } else if stage.id == context.state.current_stage {
            "current"
        } else {
            "locked"
        };
        let completed_at = context
            .state
            .completed_stage_timestamps
            .get(&stage.id)
            .map_or("", String::as_str);
        out.push_str(&format!(
            "- `{}` - {}: {} {}\n",
            stage.id, stage.title, status, completed_at
        ));
    }

    out.push_str("\n## Latest Test Results\n\n");
    if context.state.last_test_runs.is_empty() {
        out.push_str("No test runs recorded yet.\n");
    } else {
        for run in context.state.last_test_runs.values() {
            out.push_str(&format!(
                "- `{}`: {} passed, {} failed at `{}`\n",
                run.stage_id, run.passed, run.failed, run.timestamp
            ));
        }
    }

    out.push_str("\n## Benchmark History\n\n");
    if history.is_empty() {
        out.push_str("No benchmark history recorded yet.\n");
    } else {
        out.push_str(
            "| Stage | Benchmark | Params | Median ms | P95 ms | Throughput MB/s | Peak MB |\n",
        );
        out.push_str("| --- | --- | --- | ---: | ---: | ---: | ---: |\n");
        for record in history {
            for point in &record.points {
                let label = point.params_label();
                out.push_str(&format!(
                    "| `{}` | {} | {} | {} | {} | {} | {} |\n",
                    record.stage,
                    record.benchmark,
                    if label.is_empty() {
                        "-".to_string()
                    } else {
                        label
                    },
                    format_optional(point.runtime_median_ms),
                    format_optional(point.runtime_p95_ms),
                    format_optional(point.throughput_mb_s),
                    format_optional(point.peak_memory_mb)
                ));
            }
        }
    }

    out.push_str("\n## Design Notes\n\n");
    if design_notes.is_empty() {
        out.push_str("No design notes recorded yet.\n");
    } else {
        for (stage, notes) in design_notes {
            out.push_str(&format!("### `{stage}`\n\n{}\n\n", notes.trim()));
        }
    }

    out.push_str("## Environment Summary\n\n");
    out.push_str(&format!("- OS: `{}`\n", std::env::consts::OS));
    out.push_str(&format!("- Architecture: `{}`\n", std::env::consts::ARCH));
    out
}

fn render_html(
    context: &ProjectContext,
    history: &[crate::commands::bench::BenchmarkRecord],
    design_notes: &[(String, String)],
) -> String {
    let markdown = render_markdown(context, history, design_notes);
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>{} Report</title><style>body{{font-family:system-ui,sans-serif;line-height:1.5;max-width:900px;margin:40px auto;padding:0 20px}}pre{{white-space:pre-wrap}}</style></head><body><pre>{}</pre></body></html>",
        html_escape(&context.pack.manifest.name),
        html_escape(&markdown)
    )
}

fn render_json(
    context: &ProjectContext,
    history: &[crate::commands::bench::BenchmarkRecord],
    design_notes: &[(String, String)],
) -> JsonReport {
    JsonReport {
        project: context.state.project.clone(),
        pack_name: context.pack.manifest.name.clone(),
        language: context.state.language.clone(),
        current_stage: context.state.current_stage.clone(),
        completed_stages: context.state.completed_stages.clone(),
        latest_test_runs: context.state.last_test_runs.values().cloned().collect(),
        benchmark_history: history.to_vec(),
        design_notes: design_notes
            .iter()
            .map(|(stage, notes)| JsonDesignNote {
                stage: stage.clone(),
                notes: notes.clone(),
            })
            .collect(),
        environment: JsonEnvironment {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        },
    }
}

#[derive(Debug, Serialize)]
struct JsonReport {
    project: String,
    pack_name: String,
    language: String,
    current_stage: String,
    completed_stages: Vec<String>,
    latest_test_runs: Vec<crate::state::LastTestRunSummary>,
    benchmark_history: Vec<crate::commands::bench::BenchmarkRecord>,
    design_notes: Vec<JsonDesignNote>,
    environment: JsonEnvironment,
}

#[derive(Debug, Serialize)]
struct JsonDesignNote {
    stage: String,
    notes: String,
}

#[derive(Debug, Serialize)]
struct JsonEnvironment {
    os: String,
    arch: String,
}

fn read_design_notes(context: &ProjectContext) -> Result<Vec<(String, String)>> {
    let notes_dir = context.root.join(".deltaforge").join("design_notes");
    if !notes_dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut notes = Vec::new();
    for entry in fs::read_dir(notes_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let stage = entry
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("unknown")
                .to_string();
            notes.push((stage, fs::read_to_string(entry.path())?));
        }
    }
    notes.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(notes)
}

fn format_optional(value: Option<f64>) -> String {
    value.map_or_else(|| "-".to_string(), |value| format!("{value:.2}"))
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
