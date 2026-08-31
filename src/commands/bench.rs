//! Terminal rendering for `deltaforge bench`.
//!
//! Measurement, gate evaluation, and history live in [`crate::benchmarks`];
//! the run lease, journal, and event stream in
//! [`crate::application::run_benchmarks`]. This module only turns an outcome
//! into text.

use anyhow::{Result, bail};

use crate::application::{self, BenchmarkRunRequest, RunTrigger};
use crate::benchmarks::{
    BenchmarkComparison, BenchmarkPoint, BenchmarkRecord, ComparisonOutcome, EvaluatedGate,
    MetricComparison, history_path, metric_name, thread_speedup,
};
use crate::cli::BenchArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::pack::GateBound;

pub fn run(args: BenchArgs, options: &GlobalOptions) -> Result<()> {
    if args.iterations == Some(0) {
        bail!("benchmark iterations must be greater than 0");
    }
    let request = BenchmarkRunRequest {
        stage: args.stage.clone(),
        all: args.all,
        iterations: args.iterations,
        warmup: args.warmup,
        save: args.save,
        compare: args.compare,
        trigger: RunTrigger::Cli,
    };
    let mut sink = application::NullEventSink;
    let outcome = application::run_benchmarks(options, request, &mut sink)?;
    let context = ProjectContext::load(options)?;

    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&records_json_with_gates(
                &context,
                &outcome.records,
                outcome.comparisons.as_deref(),
                &outcome.gates
            )?)?
        );
    } else if outcome.records.is_empty() {
        println!("No benchmarks defined for selected stage(s).");
    } else {
        for (index, record) in outcome.records.iter().enumerate() {
            println!("{} / {}", record.stage, record.benchmark);
            print!("{}", render_benchmark_human(record));
            if let Some(comparisons) = &outcome.comparisons {
                print!("{}", render_comparison_human(&comparisons[index]));
            }
        }
        for report in &outcome.gates {
            print!("{}", render_gate_human(&context, report));
        }
        if outcome.saved {
            println!(
                "Saved benchmark history: {}",
                history_path(&context).display()
            );
        }
    }

    if let Some(error) = &outcome.execution_error {
        bail!("{error}");
    }
    if outcome.failed_points() > 0 {
        bail!("one or more benchmarks failed");
    }
    Ok(())
}

fn render_gate_human(context: &ProjectContext, report: &application::StageGateReport) -> String {
    let correctness = if context.verify_completion_proof(&report.stage_id).is_ok() {
        "passed"
    } else {
        "not yet"
    };
    let mut out = format!(
        "Correctness: {correctness}\nPerformance: {}\n",
        report.status.label().replace('_', " ")
    );
    for gate in &report.gates {
        out.push_str(&format!(
            "\nGate: {}\n  required: {} {} {}\n",
            gate.name,
            metric_name(gate.metric),
            match gate.bound {
                GateBound::Min(_) => ">=",
                GateBound::Max(_) => "<=",
            },
            match gate.bound {
                GateBound::Min(value) | GateBound::Max(value) => value,
            }
        ));
        match gate.measured {
            Some(value) => out.push_str(&format!("  measured: {value}\n")),
            None => out.push_str("  measured: not available\n"),
        }
        if !gate.passed && !gate.advice.is_empty() {
            out.push_str("\nLikely areas to investigate:\n");
            for advice in &gate.advice {
                out.push_str(&format!("- {advice}\n"));
            }
        }
    }
    out
}

/// JSON output for `bench --json`: the records, each augmented with a
/// `derived` object (e.g. `{"speedup_1_to_8": 3.4}`) when a speedup applies.
/// Derived metrics are attached only here, never written to history.
fn records_json(
    records: &[BenchmarkRecord],
    comparisons: Option<&[BenchmarkComparison]>,
) -> Result<serde_json::Value> {
    let mut value = serde_json::to_value(records)?;
    if let serde_json::Value::Array(items) = &mut value {
        for (index, (item, record)) in items.iter_mut().zip(records).enumerate() {
            if let Some(speedup) = thread_speedup(&record.points) {
                item["derived"] = serde_json::json!({ speedup.key: speedup.value });
            }
            if let Some(comparisons) = comparisons {
                item["comparison"] = serde_json::to_value(&comparisons[index])?;
            }
        }
    }
    Ok(value)
}

fn records_json_with_gates(
    context: &ProjectContext,
    records: &[BenchmarkRecord],
    comparisons: Option<&[BenchmarkComparison]>,
    reports: &[application::StageGateReport],
) -> Result<serde_json::Value> {
    let mut value = records_json(records, comparisons)?;
    if let serde_json::Value::Array(items) = &mut value {
        for (item, record) in items.iter_mut().zip(records) {
            if let Some(report) = reports
                .iter()
                .find(|report| report.stage_id == record.stage && !report.gates.is_empty())
            {
                let gates: Vec<&EvaluatedGate> = report
                    .gates
                    .iter()
                    .filter(|gate| gate.benchmark == record.benchmark)
                    .collect();
                item["gate_results"] = serde_json::to_value(gates)?;
                item["correctness"] = serde_json::Value::String(
                    if context.verify_completion_proof(&record.stage).is_ok() {
                        "passed"
                    } else {
                        "not_yet"
                    }
                    .to_string(),
                );
                item["performance"] = serde_json::Value::String(report.status.label().to_string());
            }
        }
    }
    Ok(value)
}

fn render_comparison_human(comparison: &BenchmarkComparison) -> String {
    let mut out = String::from("  Comparison with prior saved run:\n");
    for point in &comparison.points {
        let label = if point.params.is_empty() {
            String::new()
        } else {
            format!(
                " [{}]",
                point
                    .params
                    .iter()
                    .map(|(name, value)| format!("{name}={value}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let Some(timestamp) = &point.prior_timestamp else {
            out.push_str(&format!("    {label} no prior saved result\n"));
            continue;
        };
        out.push_str(&format!("    {label} prior: {timestamp}\n"));
        render_metric_comparison(
            &mut out,
            "median",
            point.runtime_median_ms.as_ref(),
            "ms",
            2,
        );
        render_metric_comparison(
            &mut out,
            "throughput",
            point.throughput_mb_s.as_ref(),
            "MB/s",
            2,
        );
        render_metric_comparison(
            &mut out,
            "peak memory",
            point.peak_memory_mb.as_ref(),
            "MB",
            1,
        );
        if !point.machine_differences.is_empty() {
            let differences = point
                .machine_differences
                .iter()
                .map(|(name, values)| format!("{name}: {} -> {}", values.previous, values.current))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "      note: machine differs ({differences}); results may not be directly comparable\n"
            ));
        }
    }
    out
}

fn render_metric_comparison(
    out: &mut String,
    name: &str,
    comparison: Option<&MetricComparison>,
    unit: &str,
    decimals: usize,
) {
    let Some(comparison) = comparison else {
        out.push_str(&format!("      {name}: not available\n"));
        return;
    };
    let percent = comparison.percent_delta.map_or_else(
        || "percentage unavailable".to_string(),
        |value| format!("{value:+.2}%"),
    );
    let outcome = match comparison.outcome {
        ComparisonOutcome::Improved => "improved",
        ComparisonOutcome::Regressed => "regressed",
        ComparisonOutcome::Unchanged => "unchanged",
    };
    out.push_str(&format!(
        "      {name}: {previous:.decimals$} {unit} -> {current:.decimals$} {unit} ({delta:+.decimals$} {unit}, {percent}) — {outcome}\n",
        previous = comparison.previous,
        current = comparison.current,
        delta = comparison.delta,
    ));
}

/// Aligned per-point table (params, median, p95, throughput, peak memory),
/// followed by the derived speedup line and any per-point failures.
fn render_benchmark_human(record: &BenchmarkRecord) -> String {
    let mut out = String::new();

    let successful: Vec<&BenchmarkPoint> =
        record.points.iter().filter(|point| point.success).collect();
    if !successful.is_empty() {
        let has_params = successful.iter().any(|point| !point.params.is_empty());
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(successful.len() + 1);
        let mut header = Vec::new();
        if has_params {
            header.push("params".to_string());
        }
        header.extend(
            ["median", "p95", "throughput", "peak mem"]
                .iter()
                .map(ToString::to_string),
        );
        rows.push(header);
        for point in &successful {
            let mut row = Vec::new();
            if has_params {
                row.push(point.params_label());
            }
            row.push(format_measure(point.runtime_median_ms, "ms", 2));
            row.push(format_measure(point.runtime_p95_ms, "ms", 2));
            row.push(format_measure(point.throughput_mb_s, "MB/s", 2));
            row.push(format_measure(point.peak_memory_mb, "MB", 1));
            rows.push(row);
        }

        let columns = rows[0].len();
        let widths: Vec<usize> = (0..columns)
            .map(|column| rows.iter().map(|row| row[column].len()).max().unwrap_or(0))
            .collect();
        for row in &rows {
            out.push_str("  ");
            for (column, cell) in row.iter().enumerate() {
                if column > 0 {
                    out.push_str("  ");
                }
                let width = widths[column];
                if has_params && column == 0 {
                    out.push_str(&format!("{cell:<width$}"));
                } else {
                    out.push_str(&format!("{cell:>width$}"));
                }
            }
            while out.ends_with(' ') {
                out.pop();
            }
            out.push('\n');
        }
    }

    if let Some(speedup) = thread_speedup(&record.points) {
        out.push_str(&format!("  {}: {:.2}x\n", speedup.key, speedup.value));
    }

    for point in &record.points {
        if !point.success
            && let Some(error) = &point.error
        {
            let label = point.params_label();
            if label.is_empty() {
                out.push_str(&format!("  failed: {error}\n"));
            } else {
                out.push_str(&format!("  [{label}] failed: {error}\n"));
            }
        }
    }
    out
}

fn format_measure(value: Option<f64>, unit: &str, decimals: usize) -> String {
    value.map_or_else(
        || "-".to_string(),
        |value| format!("{value:.decimals$} {unit}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::compare_records;
    use std::collections::BTreeMap;

    fn sample_record() -> BenchmarkRecord {
        BenchmarkRecord {
            project: "flashindex".to_string(),
            language: "rust".to_string(),
            stage: "01_scan_files".to_string(),
            benchmark: "scan_basic_project".to_string(),
            timestamp: "2026-02-01T00:00:00Z".to_string(),
            git_commit: None,
            command: vec!["scan".to_string()],
            points: vec![BenchmarkPoint {
                params: BTreeMap::from([("threads".to_string(), "4".to_string())]),
                success: true,
                iterations: 5,
                warmup: 1,
                runtime_median_ms: Some(10.0),
                runtime_p95_ms: Some(12.0),
                throughput_mb_s: Some(200.0),
                peak_memory_mb: Some(64.5),
                error: None,
            }],
            machine: BTreeMap::new(),
        }
    }

    fn point(params: &[(&str, &str)], median: Option<f64>) -> BenchmarkPoint {
        BenchmarkPoint {
            params: params
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect(),
            success: true,
            iterations: 3,
            warmup: 1,
            runtime_median_ms: median,
            runtime_p95_ms: median,
            throughput_mb_s: Some(100.0),
            peak_memory_mb: Some(32.0),
            error: None,
        }
    }

    #[test]
    fn human_output_renders_aligned_table_and_speedup() {
        let mut record = sample_record();
        record.points = vec![
            point(&[("threads", "1")], Some(800.0)),
            point(&[("threads", "8")], Some(100.0)),
        ];
        let rendered = render_benchmark_human(&record);
        assert_eq!(
            rendered,
            "  params        median        p95   throughput  peak mem\n  \
             threads=1  800.00 ms  800.00 ms  100.00 MB/s   32.0 MB\n  \
             threads=8  100.00 ms  100.00 ms  100.00 MB/s   32.0 MB\n  \
             speedup_1_to_8: 8.00x\n"
        );
    }

    #[test]
    fn human_output_without_params_omits_params_column() {
        let mut record = sample_record();
        record.points = vec![point(&[], Some(10.0))];
        record.points[0].peak_memory_mb = None;
        let rendered = render_benchmark_human(&record);
        assert!(rendered.contains("median"), "{rendered}");
        assert!(!rendered.contains("params"), "{rendered}");
        assert!(rendered.contains("10.00 ms"), "{rendered}");
        assert!(!rendered.contains("speedup"), "{rendered}");
    }

    #[test]
    fn human_output_reports_failed_points_after_table() {
        let mut record = sample_record();
        let mut failed = point(&[("threads", "8")], None);
        failed.success = false;
        failed.error = Some("command failed".to_string());
        record.points = vec![point(&[("threads", "1")], Some(800.0)), failed];
        let rendered = render_benchmark_human(&record);
        assert!(
            rendered.contains("[threads=8] failed: command failed"),
            "{rendered}"
        );
        assert!(!rendered.contains("speedup"), "{rendered}");
    }

    #[test]
    fn json_records_attach_derived_speedup_without_persisting() {
        let mut record = sample_record();
        record.points = vec![
            point(&[("threads", "1")], Some(800.0)),
            point(&[("threads", "8")], Some(100.0)),
        ];
        let json = records_json(std::slice::from_ref(&record), None).unwrap();
        assert_eq!(json[0]["derived"]["speedup_1_to_8"], 8.0);
        // The persisted form stays free of derived metrics.
        let persisted = serde_json::to_value(&record).unwrap();
        assert!(persisted.get("derived").is_none());
    }

    #[test]
    fn comparison_uses_latest_exact_point_and_formats_metric_directions() {
        let mut older = sample_record();
        older.timestamp = "2026-01-01T00:00:00Z".to_string();
        older.points[0].runtime_median_ms = Some(20.0);

        let mut latest = sample_record();
        latest.timestamp = "2026-01-02T00:00:00Z".to_string();
        latest.points[0].runtime_median_ms = Some(10.0);
        latest.points[0].throughput_mb_s = Some(200.0);
        latest.points[0].peak_memory_mb = Some(64.0);
        latest.machine = BTreeMap::from([
            ("os".to_string(), "macos".to_string()),
            ("arch".to_string(), "aarch64".to_string()),
        ]);

        let mut current = sample_record();
        current.timestamp = "2026-01-03T00:00:00Z".to_string();
        current.points[0].runtime_median_ms = Some(8.0);
        current.points[0].throughput_mb_s = Some(250.0);
        current.points[0].peak_memory_mb = Some(70.0);
        current.machine = BTreeMap::from([
            ("os".to_string(), "linux".to_string()),
            ("arch".to_string(), "x86_64".to_string()),
        ]);

        let comparisons = compare_records(std::slice::from_ref(&current), &[older, latest.clone()]);
        let point = &comparisons[0].points[0];
        assert_eq!(
            point.prior_timestamp.as_deref(),
            Some("2026-01-02T00:00:00Z")
        );
        assert_eq!(
            point.runtime_median_ms.as_ref().unwrap().outcome,
            ComparisonOutcome::Improved
        );
        assert_eq!(
            point.throughput_mb_s.as_ref().unwrap().outcome,
            ComparisonOutcome::Improved
        );
        assert_eq!(
            point.peak_memory_mb.as_ref().unwrap().outcome,
            ComparisonOutcome::Regressed
        );
        assert_eq!(point.machine_differences.len(), 2);

        let rendered = render_comparison_human(&comparisons[0]);
        assert!(
            rendered.contains("median: 10.00 ms -> 8.00 ms (-2.00 ms, -20.00%) — improved"),
            "{rendered}"
        );
        assert!(
            rendered.contains(
                "throughput: 200.00 MB/s -> 250.00 MB/s (+50.00 MB/s, +25.00%) — improved"
            ),
            "{rendered}"
        );
        assert!(rendered.contains("peak memory"), "{rendered}");
        assert!(rendered.contains("— regressed"), "{rendered}");
        assert!(rendered.contains("machine differs"), "{rendered}");

        let json = records_json(&[current], Some(&comparisons)).unwrap();
        assert_eq!(
            json[0]["comparison"]["points"][0]["runtime_median_ms"]["outcome"],
            "improved"
        );
        let persisted = serde_json::to_value(&latest).unwrap();
        assert!(persisted.get("comparison").is_none());
    }

    #[test]
    fn comparison_requires_an_exact_parameter_match() {
        let prior = sample_record();
        let mut current = sample_record();
        current.points[0]
            .params
            .insert("threads".into(), "8".into());
        let comparisons = compare_records(&[current], &[prior]);
        let point = &comparisons[0].points[0];
        assert!(point.prior_timestamp.is_none());
        let rendered = render_comparison_human(&comparisons[0]);
        assert!(rendered.contains("[threads=8] no prior saved result"));
    }
}
