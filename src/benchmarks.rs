//! Benchmark execution, gate evaluation, and benchmark history.
//!
//! This module is to `deltaforge bench` what [`crate::runner`] is to
//! `deltaforge test`: it measures and evaluates, emitting [`RunEvent`]s as it
//! goes, and never prints. Rendering lives in [`crate::commands::bench`], and
//! the run lease, journal, and cancellation path are owned by
//! [`crate::application::run_benchmarks`].

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::application::{EventSink, RunEvent};
use crate::context::ProjectContext;
use crate::fs_util::atomic_write;
use crate::pack::{
    BenchmarkSpec, CommandSpec, GateBound, PerformanceGate, PerformanceMetric, StageBenchmarks,
    StageSpec, benchmark_matrix_problems, benchmark_scalar_to_string,
};
use crate::process;
use crate::state::{GateRecord, RecordedGateResult};

/// Execution knobs for one benchmark run. `iterations` and `warmup` override
/// the per-benchmark and project defaults when present; `cancellation_path`
/// is the same sentinel file the test runner watches, so one cancel request
/// stops whichever kind of job is holding the run lease.
#[derive(Debug, Clone, Default)]
pub struct BenchmarkOptions {
    pub iterations: Option<u64>,
    pub warmup: Option<u64>,
    pub cancellation_path: Option<PathBuf>,
}

impl BenchmarkOptions {
    fn is_cancelled(&self) -> bool {
        self.cancellation_path
            .as_ref()
            .is_some_and(|path| path.exists())
    }
}

pub const HISTORY_SCHEMA_VERSION: u64 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkRecord {
    pub project: String,
    pub language: String,
    pub stage: String,
    pub benchmark: String,
    pub timestamp: String,
    pub git_commit: Option<String>,
    pub command: Vec<String>,
    pub points: Vec<BenchmarkPoint>,
    pub machine: BTreeMap<String, String>,
}

/// One measured matrix point. Benchmarks without a parameter matrix produce
/// exactly one point with empty `params`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkPoint {
    #[serde(default)]
    pub params: BTreeMap<String, String>,
    pub success: bool,
    pub iterations: u64,
    pub warmup: u64,
    pub runtime_median_ms: Option<f64>,
    pub runtime_p95_ms: Option<f64>,
    pub throughput_mb_s: Option<f64>,
    #[serde(default)]
    pub peak_memory_mb: Option<f64>,
    pub error: Option<String>,
}

impl BenchmarkPoint {
    /// Human label for the point's parameters, e.g. `threads=4`; empty for
    /// non-matrix benchmarks.
    pub fn params_label(&self) -> String {
        self.params
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Derived speedup across a `threads` matrix parameter. Computed at display
/// time from the points and never persisted (the history schema stays fixed).
pub struct ThreadSpeedup {
    /// e.g. `speedup_1_to_8`
    pub key: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOutcome {
    Improved,
    Regressed,
    Unchanged,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetricComparison {
    pub previous: f64,
    pub current: f64,
    pub delta: f64,
    pub percent_delta: Option<f64>,
    pub outcome: ComparisonOutcome,
}

#[derive(Debug, Clone, Serialize)]
pub struct MachineDifference {
    pub previous: String,
    pub current: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PointComparison {
    pub params: BTreeMap<String, String>,
    pub prior_timestamp: Option<String>,
    pub runtime_median_ms: Option<MetricComparison>,
    pub throughput_mb_s: Option<MetricComparison>,
    pub peak_memory_mb: Option<MetricComparison>,
    pub machine_differences: BTreeMap<String, MachineDifference>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkComparison {
    pub points: Vec<PointComparison>,
}

/// Speedup = median(min threads) / median(max threads), for benchmarks whose
/// points vary over a numeric `threads` parameter. `None` when there are
/// fewer than two distinct thread counts, when `threads` is not the only
/// varying parameter (multiple points per extreme), when a thread value is
/// non-numeric, or when either extreme's median is missing.
pub fn thread_speedup(points: &[BenchmarkPoint]) -> Option<ThreadSpeedup> {
    let mut by_threads: BTreeMap<u64, Vec<&BenchmarkPoint>> = BTreeMap::new();
    for point in points {
        if let Some(threads) = point.params.get("threads") {
            by_threads
                .entry(threads.parse().ok()?)
                .or_default()
                .push(point);
        }
    }
    if by_threads.len() < 2 {
        return None;
    }
    let (min_threads, min_points) = by_threads.first_key_value()?;
    let (max_threads, max_points) = by_threads.last_key_value()?;
    let [min_point] = min_points.as_slice() else {
        return None;
    };
    let [max_point] = max_points.as_slice() else {
        return None;
    };
    let min_median = min_point.runtime_median_ms?;
    let max_median = max_point.runtime_median_ms?;
    if !min_point.success
        || !max_point.success
        || !min_median.is_finite()
        || !max_median.is_finite()
        || max_median <= 0.0
    {
        return None;
    }
    Some(ThreadSpeedup {
        key: format!("speedup_{min_threads}_to_{max_threads}"),
        value: min_median / max_median,
    })
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct VersionedHistory {
    schema_version: u64,
    runs: Vec<BenchmarkRecord>,
}

/// History records written before the file was versioned (a bare JSON array
/// with one flat `results` object per record).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyBenchmarkRecord {
    project: String,
    language: String,
    stage: String,
    benchmark: String,
    timestamp: String,
    git_commit: Option<String>,
    command: Vec<String>,
    results: LegacyBenchmarkResult,
    machine: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyBenchmarkResult {
    success: bool,
    iterations: u64,
    warmup: u64,
    runtime_median_ms: Option<f64>,
    runtime_p95_ms: Option<f64>,
    throughput_mb_s: Option<f64>,
    error: Option<String>,
}

impl From<LegacyBenchmarkRecord> for BenchmarkRecord {
    fn from(legacy: LegacyBenchmarkRecord) -> Self {
        BenchmarkRecord {
            project: legacy.project,
            language: legacy.language,
            stage: legacy.stage,
            benchmark: legacy.benchmark,
            timestamp: legacy.timestamp,
            git_commit: legacy.git_commit,
            command: legacy.command,
            points: vec![BenchmarkPoint {
                params: BTreeMap::new(),
                success: legacy.results.success,
                iterations: legacy.results.iterations,
                warmup: legacy.results.warmup,
                runtime_median_ms: legacy.results.runtime_median_ms,
                runtime_p95_ms: legacy.results.runtime_p95_ms,
                throughput_mb_s: legacy.results.throughput_mb_s,
                peak_memory_mb: None,
                error: legacy.results.error,
            }],
            machine: legacy.machine,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EvaluatedGate {
    pub name: String,
    pub benchmark: String,
    pub metric: PerformanceMetric,
    pub params: BTreeMap<String, String>,
    pub bound: GateBound,
    pub measured: Option<f64>,
    pub passed: bool,
    #[serde(skip_serializing)]
    pub advice: Vec<String>,
}

pub struct StageGateEvaluation {
    pub stage: String,
    pub gates: Vec<EvaluatedGate>,
    pub complete: bool,
}

impl StageGateEvaluation {
    /// This run's own gate outcome, independent of anything previously
    /// recorded. A run that could not measure every gate reports
    /// `NotMeasured` even when a prior complete measurement is still stored.
    pub fn status(&self) -> crate::context::GateStatus {
        use crate::context::GateStatus;
        if !self.complete || self.gates.iter().any(|gate| gate.measured.is_none()) {
            GateStatus::NotMeasured
        } else if self.gates.iter().all(|gate| gate.passed) {
            GateStatus::Passed
        } else {
            GateStatus::NotYet
        }
    }

    pub fn gates_for_benchmark(&self, benchmark: &str) -> Vec<&EvaluatedGate> {
        self.gates
            .iter()
            .filter(|gate| gate.benchmark == benchmark)
            .collect()
    }

    pub fn record(&self, context: &ProjectContext) -> Result<Option<GateRecord>> {
        if self.gates.is_empty()
            || !self.complete
            || self.gates.iter().any(|gate| gate.measured.is_none())
        {
            return Ok(None);
        }
        let results = self
            .gates
            .iter()
            .map(|gate| RecordedGateResult {
                name: gate.name.clone(),
                benchmark: gate.benchmark.clone(),
                metric: gate.metric,
                params: gate.params.clone(),
                bound: gate.bound.clone(),
                measured: gate.measured.expect("checked"),
                passed: gate.passed,
            })
            .collect();
        Ok(Some(GateRecord {
            timestamp: current_timestamp()?,
            project_digest: context.project_digest()?,
            behavioral_digest: context.stage_behavioral_digest(&self.stage)?,
            results,
        }))
    }
}

pub fn evaluate_stage_gates(
    context: &ProjectContext,
    stage: &StageSpec,
    records: &[BenchmarkRecord],
) -> Result<StageGateEvaluation> {
    let source = context.pack.benchmarks_path(stage);
    if !source.is_file() {
        return Ok(StageGateEvaluation {
            stage: stage.id.clone(),
            gates: Vec::new(),
            complete: false,
        });
    }
    let parsed: StageBenchmarks = serde_yaml::from_str(&fs::read_to_string(&source)?)
        .with_context(|| format!("failed to parse benchmarks file {}", source.display()))?;
    let stage_records: Vec<&BenchmarkRecord> = records
        .iter()
        .filter(|record| record.stage == stage.id)
        .collect();
    let complete = stage_records.len() == parsed.benchmarks.len()
        && stage_records
            .iter()
            .all(|record| record.points.iter().all(|point| point.success));
    let mut gates: Vec<EvaluatedGate> = parsed
        .performance_gates
        .iter()
        .filter_map(|gate| evaluate_gate(stage, gate, &stage_records))
        .collect();
    let every_gate_evaluated = gates.len() == parsed.performance_gates.len();
    let complete = complete && every_gate_evaluated;
    suppress_incomplete_passes(&mut gates, complete);
    Ok(StageGateEvaluation {
        stage: stage.id.clone(),
        gates,
        complete,
    })
}

fn suppress_incomplete_passes(gates: &mut [EvaluatedGate], complete: bool) {
    if !complete {
        // A selected point can meet its bound while another point or benchmark
        // fails. Preserve the measurement for diagnostics, but never report an
        // individual pass from an incomplete stage run.
        for gate in gates {
            gate.passed = false;
        }
    }
}

fn evaluate_gate(
    stage: &StageSpec,
    gate: &PerformanceGate,
    records: &[&BenchmarkRecord],
) -> Option<EvaluatedGate> {
    let bound = gate.bound()?;
    let matching: Vec<_> = records
        .iter()
        .copied()
        .filter(|record| record.stage == stage.id && record.benchmark == gate.benchmark)
        .collect();
    let measured = if matching.len() == 1 {
        let record = matching[0];
        if gate.metric == PerformanceMetric::Speedup {
            if gate.params.is_empty() {
                thread_speedup(&record.points)
                    .map(|speedup| speedup.value)
                    .filter(|value| value.is_finite())
            } else {
                None
            }
        } else {
            let points: Vec<_> = record
                .points
                .iter()
                .filter(|point| point.params == gate.params)
                .collect();
            if let [point] = points.as_slice() {
                if point.success {
                    let value = match gate.metric {
                        PerformanceMetric::RuntimeMedianMs => point.runtime_median_ms,
                        PerformanceMetric::RuntimeP95Ms => point.runtime_p95_ms,
                        PerformanceMetric::ThroughputMbS => point.throughput_mb_s,
                        PerformanceMetric::PeakMemoryMb => point.peak_memory_mb,
                        PerformanceMetric::Speedup => None,
                    };
                    value.filter(|value| value.is_finite())
                } else {
                    None
                }
            } else {
                None
            }
        }
    } else {
        None
    };
    let passed = measured.is_some_and(|value| match bound {
        GateBound::Min(min) => value >= min,
        GateBound::Max(max) => value <= max,
    });
    Some(EvaluatedGate {
        name: gate.name.clone(),
        benchmark: gate.benchmark.clone(),
        metric: gate.metric,
        params: gate.params.clone(),
        bound,
        measured,
        passed,
        advice: gate.advice.clone(),
    })
}

pub fn metric_name(metric: PerformanceMetric) -> &'static str {
    match metric {
        PerformanceMetric::RuntimeMedianMs => "runtime_median_ms",
        PerformanceMetric::RuntimeP95Ms => "runtime_p95_ms",
        PerformanceMetric::ThroughputMbS => "throughput_mb_s",
        PerformanceMetric::PeakMemoryMb => "peak_memory_mb",
        PerformanceMetric::Speedup => "speedup",
    }
}

/// Compare each current point with the latest prior saved point carrying the
/// same project/language/stage/benchmark identity and exact parameter map.
pub fn compare_records(
    current: &[BenchmarkRecord],
    history: &[BenchmarkRecord],
) -> Vec<BenchmarkComparison> {
    current
        .iter()
        .map(|record| BenchmarkComparison {
            points: record
                .points
                .iter()
                .map(|point| compare_point(record, point, history))
                .collect(),
        })
        .collect()
}

fn compare_point(
    current_record: &BenchmarkRecord,
    current_point: &BenchmarkPoint,
    history: &[BenchmarkRecord],
) -> PointComparison {
    let prior = history.iter().rev().find_map(|record| {
        same_benchmark(current_record, record)
            .then(|| {
                record
                    .points
                    .iter()
                    .find(|point| point.params == current_point.params)
                    .map(|point| (record, point))
            })
            .flatten()
    });

    let Some((prior_record, prior_point)) = prior else {
        return PointComparison {
            params: current_point.params.clone(),
            prior_timestamp: None,
            runtime_median_ms: None,
            throughput_mb_s: None,
            peak_memory_mb: None,
            machine_differences: BTreeMap::new(),
        };
    };

    PointComparison {
        params: current_point.params.clone(),
        prior_timestamp: Some(prior_record.timestamp.clone()),
        runtime_median_ms: metric_comparison(
            prior_point.runtime_median_ms,
            current_point.runtime_median_ms,
            false,
        ),
        throughput_mb_s: metric_comparison(
            prior_point.throughput_mb_s,
            current_point.throughput_mb_s,
            true,
        ),
        peak_memory_mb: metric_comparison(
            prior_point.peak_memory_mb,
            current_point.peak_memory_mb,
            false,
        ),
        machine_differences: machine_differences(&prior_record.machine, &current_record.machine),
    }
}

fn same_benchmark(current: &BenchmarkRecord, prior: &BenchmarkRecord) -> bool {
    current.project == prior.project
        && current.language == prior.language
        && current.stage == prior.stage
        && current.benchmark == prior.benchmark
}

fn metric_comparison(
    previous: Option<f64>,
    current: Option<f64>,
    higher_is_better: bool,
) -> Option<MetricComparison> {
    let (previous, current) = (previous?, current?);
    if !previous.is_finite() || !current.is_finite() {
        return None;
    }
    let delta = current - previous;
    let outcome = if delta == 0.0 {
        ComparisonOutcome::Unchanged
    } else if (delta > 0.0) == higher_is_better {
        ComparisonOutcome::Improved
    } else {
        ComparisonOutcome::Regressed
    };
    Some(MetricComparison {
        previous,
        current,
        delta,
        percent_delta: (previous != 0.0).then(|| delta / previous * 100.0),
        outcome,
    })
}

fn machine_differences(
    previous: &BTreeMap<String, String>,
    current: &BTreeMap<String, String>,
) -> BTreeMap<String, MachineDifference> {
    ["os", "arch"]
        .into_iter()
        .filter_map(|key| {
            let previous = previous.get(key)?;
            let current = current.get(key)?;
            (previous != current).then(|| {
                (
                    key.to_string(),
                    MachineDifference {
                        previous: previous.clone(),
                        current: current.clone(),
                    },
                )
            })
        })
        .collect()
}

/// Build the project, then run every benchmark the stage declares, emitting
/// progress through `sink`. Returns an empty vector for a stage without a
/// benchmarks file, which is not an error: most stages carry none.
pub fn run_stage_benchmarks(
    context: &ProjectContext,
    stage: &StageSpec,
    options: &BenchmarkOptions,
    sink: &mut dyn EventSink,
) -> Result<Vec<BenchmarkRecord>> {
    if options.is_cancelled() {
        bail!("run cancelled");
    }
    let path = context.pack.benchmarks_path(stage);
    if !path.is_file() {
        return Ok(Vec::new());
    }

    let source = fs::read_to_string(&path)
        .with_context(|| format!("failed to read benchmarks file {}", path.display()))?;
    let parsed: StageBenchmarks = serde_yaml::from_str(&source)
        .with_context(|| format!("failed to parse benchmarks file {}", path.display()))?;

    let language = context
        .pack
        .manifest
        .language(&context.state.language)
        .with_context(|| {
            format!(
                "pack {} does not support language {}",
                context.state.project, context.state.language
            )
        })?;

    if let Some(build) = &language.build {
        sink.emit(RunEvent::BuildStarted {
            command: build.command.clone(),
        });
        let output = process::run_command_cancellable_streaming(
            &build.command,
            &context.root,
            context.config.runner.build_timeout_ms,
            None,
            &BTreeMap::new(),
            options.cancellation_path.as_deref(),
            &mut |stream, bytes| {
                sink.emit(RunEvent::BuildOutput {
                    stream,
                    text: String::from_utf8_lossy(bytes).to_string(),
                });
            },
        )?;
        sink.emit(RunEvent::BuildCompleted {
            passed: output.status.success(),
        });
        if !output.status.success() {
            bail!(
                "benchmark build failed\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let bench_command = resolve_bench_command(language.bench_command(), &context.root);

    let total = parsed.benchmarks.len();
    let mut records = Vec::new();
    for (index, benchmark) in parsed.benchmarks.into_iter().enumerate() {
        if options.is_cancelled() {
            bail!("run cancelled");
        }
        sink.emit(RunEvent::BenchmarkStarted {
            stage_id: stage.id.clone(),
            name: benchmark.name.clone(),
            index: index + 1,
            total,
        });
        let record = run_one_benchmark(context, stage, &bench_command, benchmark, options, sink)?;
        sink.emit(RunEvent::BenchmarkCompleted {
            stage_id: stage.id.clone(),
            name: record.benchmark.clone(),
            success: record.points.iter().all(|point| point.success),
        });
        records.push(record);
    }
    Ok(records)
}

/// Normalize a benchmark command's program so it runs correctly regardless of
/// platform and working directory. A relative program such as
/// `./target/release/flashindex` is resolved against the project root (avoiding
/// the parent-vs-child cwd ambiguity when spawning) and gets the platform
/// executable suffix appended on Windows. PATH-resolved programs (e.g. `cargo`)
/// are left untouched.
fn resolve_bench_command(spec: &CommandSpec, root: &Path) -> CommandSpec {
    let mut command = spec.command.clone();
    if let Some(program) = command.first_mut() {
        *program = resolve_program(program, root);
    }
    CommandSpec { command }
}

fn resolve_program(program: &str, root: &Path) -> String {
    let looks_relative = program.starts_with("./")
        || program.starts_with(".\\")
        || program.contains('/')
        || program.contains('\\');
    if !looks_relative {
        return program.to_string();
    }

    let suffix = std::env::consts::EXE_SUFFIX;
    let with_suffix = if suffix.is_empty() || program.ends_with(suffix) {
        program.to_string()
    } else {
        format!("{program}{suffix}")
    };

    let path = PathBuf::from(with_suffix);
    let resolved = if path.is_absolute() {
        path
    } else {
        root.join(path)
    };
    resolved.to_string_lossy().to_string()
}

fn run_one_benchmark(
    context: &ProjectContext,
    stage: &StageSpec,
    run_spec: &CommandSpec,
    benchmark: BenchmarkSpec,
    options: &BenchmarkOptions,
    sink: &mut dyn EventSink,
) -> Result<BenchmarkRecord> {
    if benchmark.command.is_empty() {
        bail!("benchmark {} command is empty", benchmark.name);
    }

    let matrix = normalize_matrix(&benchmark)?;

    if !crate::integrity::is_safe_relative_path(Path::new(&benchmark.fixture)) {
        bail!(
            "benchmark {} fixture path is unsafe: {}",
            benchmark.name,
            benchmark.fixture
        );
    }
    let temp_dir = create_temp_dir(stage, &benchmark.name)?;
    let source_fixture = context.pack.fixture_path(stage, &benchmark.fixture);
    let fixture_path = temp_dir.join("fixture");
    copy_dir_recursive(&source_fixture, &fixture_path)?;
    let fixture_bytes = directory_size(&fixture_path)?;

    // Matrix placeholders stay intact here; they are expanded per point. The
    // record stores this base command since the expanded argv differs per point.
    let mut command = run_spec.command.clone();
    command.extend(
        benchmark
            .command
            .iter()
            .map(|arg| expand_variables(arg, &fixture_path, &temp_dir)),
    );

    let iterations = options
        .iterations
        .or(benchmark.iterations)
        .unwrap_or(context.config.bench.iterations);
    if iterations == 0 {
        bail!(
            "benchmark {} iterations must be greater than 0",
            benchmark.name
        );
    }
    let warmup = options
        .warmup
        .or(benchmark.warmup)
        .unwrap_or(context.config.bench.warmup);
    let timeout_ms = benchmark
        .timeout_ms
        .unwrap_or(context.config.runner.timeout_ms);

    let fixture = BenchmarkFixture {
        bytes: fixture_bytes,
        source: &source_fixture,
        path: &fixture_path,
    };

    let all_points = cartesian_points(&matrix);
    let point_total = all_points.len();
    let mut points = Vec::new();
    for (index, params) in all_points.into_iter().enumerate() {
        if options.is_cancelled() {
            let _ = fs::remove_dir_all(&temp_dir);
            bail!("run cancelled");
        }
        let point_command: Vec<String> = command
            .iter()
            .map(|arg| expand_params(arg, &params))
            .collect();
        let params_label = params_label(&params);
        sink.emit(RunEvent::BenchmarkPointStarted {
            stage_id: stage.id.clone(),
            name: benchmark.name.clone(),
            params_label: params_label.clone(),
            index: index + 1,
            total: point_total,
        });
        let point = run_iterations(
            &point_command,
            &context.root,
            IterationPlan {
                iterations,
                warmup,
                timeout_ms,
                cancellation_path: options.cancellation_path.as_deref(),
            },
            &fixture,
            params,
            &mut |iteration, duration_ms| {
                sink.emit(RunEvent::BenchmarkSampleRecorded {
                    stage_id: stage.id.clone(),
                    name: benchmark.name.clone(),
                    params_label: params_label.clone(),
                    iteration,
                    iterations,
                    duration_ms,
                });
            },
        );
        sink.emit(RunEvent::BenchmarkPointCompleted {
            stage_id: stage.id.clone(),
            name: benchmark.name.clone(),
            params_label,
            success: point.success,
            runtime_median_ms: point.runtime_median_ms,
            throughput_mb_s: point.throughput_mb_s,
            peak_memory_mb: point.peak_memory_mb,
            error: point.error.clone(),
        });
        points.push(point);
    }

    let _ = fs::remove_dir_all(&temp_dir);

    Ok(BenchmarkRecord {
        project: context.state.project.clone(),
        language: context.state.language.clone(),
        stage: stage.id.clone(),
        benchmark: benchmark.name,
        timestamp: current_timestamp()?,
        git_commit: current_git_commit(&context.root),
        command,
        points,
        machine: machine_metadata(),
    })
}

/// Human label for a matrix point's parameters, e.g. `threads=4`; empty for
/// non-matrix benchmarks.
fn params_label(params: &BTreeMap<String, String>) -> String {
    params
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join(", ")
}

struct BenchmarkFixture<'a> {
    bytes: u64,
    source: &'a Path,
    path: &'a Path,
}

/// Validate the benchmark's matrix declaration and stringify its scalar values.
fn normalize_matrix(benchmark: &BenchmarkSpec) -> Result<BTreeMap<String, Vec<String>>> {
    let problems = benchmark_matrix_problems(&benchmark.matrix);
    if !problems.is_empty() {
        bail!(
            "benchmark {} has an invalid matrix: {}",
            benchmark.name,
            problems.join("; ")
        );
    }
    Ok(benchmark
        .matrix
        .iter()
        .map(|(name, values)| {
            let values = values
                .iter()
                .map(|value| benchmark_scalar_to_string(value).expect("values checked scalar"))
                .collect();
            (name.clone(), values)
        })
        .collect())
}

/// Cartesian product of all matrix parameters: parameters in name order,
/// values in listed order (later parameters vary fastest). An empty matrix
/// yields a single point with no params.
fn cartesian_points(matrix: &BTreeMap<String, Vec<String>>) -> Vec<BTreeMap<String, String>> {
    let mut points = vec![BTreeMap::new()];
    for (name, values) in matrix {
        let mut expanded = Vec::with_capacity(points.len() * values.len());
        for point in &points {
            for value in values {
                let mut point = point.clone();
                point.insert(name.clone(), value.clone());
                expanded.push(point);
            }
        }
        points = expanded;
    }
    points
}

fn expand_params(value: &str, params: &BTreeMap<String, String>) -> String {
    let mut expanded = value.to_string();
    for (name, replacement) in params {
        expanded = expanded.replace(&format!("{{{name}}}"), replacement);
    }
    expanded
}

/// Iteration counts and per-command timeout for one matrix point.
#[derive(Debug, Clone, Copy)]
struct IterationPlan<'a> {
    iterations: u64,
    warmup: u64,
    timeout_ms: u64,
    cancellation_path: Option<&'a Path>,
}

fn run_iterations(
    command: &[String],
    cwd: &Path,
    plan: IterationPlan<'_>,
    fixture: &BenchmarkFixture<'_>,
    params: BTreeMap<String, String>,
    observe: &mut dyn FnMut(u64, f64),
) -> BenchmarkPoint {
    let IterationPlan {
        iterations,
        warmup,
        timeout_ms,
        cancellation_path,
    } = plan;
    for _ in 0..warmup {
        if let Err(error) = reset_fixture(fixture.source, fixture.path) {
            return failed_point(iterations, warmup, &params, error);
        }
        if let Err(error) = run_timed_command(command, cwd, timeout_ms, cancellation_path) {
            return failed_point(iterations, warmup, &params, error);
        }
    }

    let mut durations = Vec::new();
    let mut peak_rss_bytes: Option<u64> = None;
    for iteration in 0..iterations {
        if let Err(error) = reset_fixture(fixture.source, fixture.path) {
            return failed_point(iterations, warmup, &params, error);
        }
        match run_timed_command(command, cwd, timeout_ms, cancellation_path) {
            Ok((duration, sample)) => {
                let elapsed_ms = duration.as_secs_f64() * 1000.0;
                durations.push(elapsed_ms);
                observe(iteration + 1, elapsed_ms);
                if let Some(sample) = sample {
                    peak_rss_bytes = Some(peak_rss_bytes.map_or(sample, |peak| peak.max(sample)));
                }
            }
            Err(error) => return failed_point(iterations, warmup, &params, error),
        }
    }

    durations.sort_by(f64::total_cmp);
    let median = percentile(&durations, 0.5);
    let p95 = percentile(&durations, 0.95);
    let throughput = median.and_then(|ms| {
        if ms > 0.0 {
            Some((fixture.bytes as f64 / 1_048_576.0) / (ms / 1000.0))
        } else {
            None
        }
    });

    BenchmarkPoint {
        params,
        success: true,
        iterations,
        warmup,
        runtime_median_ms: median,
        runtime_p95_ms: p95,
        throughput_mb_s: throughput,
        peak_memory_mb: peak_rss_bytes.map(|bytes| bytes as f64 / 1_048_576.0),
        error: None,
    }
}

fn reset_fixture(source: &Path, destination: &Path) -> Result<()> {
    if destination.exists() {
        fs::remove_dir_all(destination).with_context(|| {
            format!(
                "failed to reset benchmark fixture {}",
                destination.display()
            )
        })?;
    }
    copy_dir_recursive(source, destination)
}

fn failed_point(
    iterations: u64,
    warmup: u64,
    params: &BTreeMap<String, String>,
    error: anyhow::Error,
) -> BenchmarkPoint {
    BenchmarkPoint {
        params: params.clone(),
        success: false,
        iterations,
        warmup,
        runtime_median_ms: None,
        runtime_p95_ms: None,
        throughput_mb_s: None,
        peak_memory_mb: None,
        error: Some(format!("{error:#}")),
    }
}

fn percentile(values: &[f64], percentile: f64) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let index = ((values.len() as f64 - 1.0) * percentile).ceil() as usize;
    values.get(index).copied()
}

pub fn append_history(context: &ProjectContext, records: &[BenchmarkRecord]) -> Result<()> {
    let path = history_path(context);
    let mut runs = read_history(&path)?;
    runs.extend_from_slice(records);
    let history = VersionedHistory {
        schema_version: HISTORY_SCHEMA_VERSION,
        runs,
    };
    atomic_write(&path, serde_json::to_string_pretty(&history)?)
}

/// Read benchmark history, converting the legacy bare-array format (written
/// before the file carried a `schema_version`) losslessly to the current
/// per-point shape.
pub fn read_history(path: &Path) -> Result<Vec<BenchmarkRecord>> {
    match fs::read_to_string(path) {
        Ok(source) => parse_history(&source)
            .with_context(|| format!("failed to parse benchmark history {}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error)
            .with_context(|| format!("failed to read benchmark history {}", path.display())),
    }
}

fn parse_history(source: &str) -> Result<Vec<BenchmarkRecord>> {
    let value: serde_json::Value = serde_json::from_str(source)?;
    if value.is_array() {
        let legacy: Vec<LegacyBenchmarkRecord> = serde_json::from_value(value)
            .context("legacy benchmark history has unexpected structure")?;
        return Ok(legacy.into_iter().map(BenchmarkRecord::from).collect());
    }
    let schema_version = value
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        .context("benchmark history is missing schema_version")?;
    if schema_version != HISTORY_SCHEMA_VERSION {
        bail!(
            "benchmark history schema_version {schema_version} is not supported by this \
             deltaforge (expected {HISTORY_SCHEMA_VERSION}); it may have been written by a \
             newer version"
        );
    }
    let history: VersionedHistory = serde_json::from_value(value)?;
    Ok(history.runs)
}

pub fn history_path(context: &ProjectContext) -> PathBuf {
    context
        .root
        .join(".deltaforge")
        .join("benchmark_history.json")
}

fn run_timed_command(
    command: &[String],
    cwd: &Path,
    timeout_ms: u64,
    cancellation_path: Option<&Path>,
) -> Result<(Duration, Option<u64>)> {
    let start = Instant::now();
    let measured = process::run_command_measured(
        command,
        cwd,
        timeout_ms,
        None,
        &BTreeMap::new(),
        cancellation_path,
    )?;
    let elapsed = start.elapsed();
    if !measured.output.status.success() {
        bail!(
            "benchmark command failed: {}\nstdout:\n{}\nstderr:\n{}",
            command.join(" "),
            String::from_utf8_lossy(&measured.output.stdout),
            String::from_utf8_lossy(&measured.output.stderr)
        );
    }
    Ok((elapsed, measured.peak_rss_bytes))
}

fn create_temp_dir(stage: &StageSpec, name: &str) -> Result<PathBuf> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "deltaforge-bench-{}-{}-{}-{}",
        std::process::id(),
        nanos,
        stage.id,
        sanitize_name(name)
    ));
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn expand_variables(value: &str, fixture_path: &Path, temp_dir: &Path) -> String {
    value
        .replace("{fixture_path}", &fixture_path.to_string_lossy())
        .replace("{temp_dir}", &temp_dir.to_string_lossy())
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    if !source.is_dir() {
        bail!("fixture directory does not exist: {}", source.display());
    }
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

fn directory_size(path: &Path) -> Result<u64> {
    let mut size = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            size += directory_size(&entry.path())?;
        } else if file_type.is_file() {
            size += entry.metadata()?.len();
        }
    }
    Ok(size)
}

fn current_git_commit(root: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn machine_metadata() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("os".to_string(), std::env::consts::OS.to_string()),
        ("arch".to_string(), std::env::consts::ARCH.to_string()),
    ])
}

fn current_timestamp() -> Result<String> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("failed to format current timestamp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resetting_fixture_discards_mutation() {
        let root =
            std::env::temp_dir().join(format!("deltaforge-bench-reset-{}", std::process::id()));
        let source = root.join("source");
        let destination = root.join("destination");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("input.txt"), "original").unwrap();
        copy_dir_recursive(&source, &destination).unwrap();
        fs::write(destination.join("input.txt"), "mutated").unwrap();
        fs::write(destination.join("extra.txt"), "extra").unwrap();

        reset_fixture(&source, &destination).unwrap();

        assert_eq!(
            fs::read_to_string(destination.join("input.txt")).unwrap(),
            "original"
        );
        assert!(!destination.join("extra.txt").exists());
        let _ = fs::remove_dir_all(root);
    }

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

    #[test]
    fn legacy_history_fixture_converts_losslessly() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/legacy_benchmark_history.json");
        let history = read_history(&path).unwrap();
        assert_eq!(history.len(), 2);

        let succeeded = &history[0];
        assert_eq!(succeeded.benchmark, "scan_basic_project");
        assert_eq!(succeeded.git_commit, None);
        assert_eq!(succeeded.points.len(), 1);
        let point = &succeeded.points[0];
        assert!(point.params.is_empty());
        assert!(point.success);
        assert_eq!(point.iterations, 5);
        assert_eq!(point.warmup, 1);
        assert_eq!(point.runtime_median_ms, Some(12.5));
        assert_eq!(point.runtime_p95_ms, Some(15.2));
        assert_eq!(point.throughput_mb_s, Some(182.4));
        assert_eq!(point.peak_memory_mb, None);

        let failed = &history[1];
        assert!(failed.git_commit.is_some());
        assert!(!failed.points[0].success);
        assert!(failed.points[0].error.is_some());
    }

    #[test]
    fn v2_history_round_trips() {
        let history = VersionedHistory {
            schema_version: HISTORY_SCHEMA_VERSION,
            runs: vec![sample_record()],
        };
        let serialized = serde_json::to_string_pretty(&history).unwrap();
        let parsed = parse_history(&serialized).unwrap();
        assert_eq!(parsed.len(), 1);
        let point = &parsed[0].points[0];
        assert_eq!(point.params.get("threads").map(String::as_str), Some("4"));
        assert_eq!(point.peak_memory_mb, Some(64.5));
        assert_eq!(point.params_label(), "threads=4");
    }

    #[test]
    fn newer_history_schema_is_rejected() {
        let error = parse_history(r#"{"schema_version": 99, "runs": []}"#).unwrap_err();
        assert!(format!("{error:#}").contains("schema_version 99"));
    }

    #[test]
    fn history_object_without_version_is_rejected() {
        let error = parse_history(r#"{"runs": []}"#).unwrap_err();
        assert!(format!("{error:#}").contains("missing schema_version"));
    }

    fn string_matrix(entries: &[(&str, &[&str])]) -> BTreeMap<String, Vec<String>> {
        entries
            .iter()
            .map(|(name, values)| {
                (
                    name.to_string(),
                    values.iter().map(|value| value.to_string()).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn cartesian_points_expands_all_combinations_in_order() {
        let matrix = string_matrix(&[("threads", &["1", "2"]), ("mode", &["fast", "safe"])]);
        let points = cartesian_points(&matrix);
        let labels: Vec<String> = points
            .iter()
            .map(|params| {
                params
                    .iter()
                    .map(|(name, value)| format!("{name}={value}"))
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect();
        // BTreeMap iterates parameters alphabetically: mode, then threads.
        assert_eq!(
            labels,
            vec![
                "mode=fast,threads=1",
                "mode=fast,threads=2",
                "mode=safe,threads=1",
                "mode=safe,threads=2",
            ]
        );
    }

    #[test]
    fn empty_matrix_yields_single_empty_point() {
        let points = cartesian_points(&BTreeMap::new());
        assert_eq!(points.len(), 1);
        assert!(points[0].is_empty());
    }

    #[test]
    fn expand_params_substitutes_declared_placeholders_only() {
        let params = BTreeMap::from([("threads".to_string(), "4".to_string())]);
        assert_eq!(expand_params("--threads={threads}", &params), "--threads=4");
        assert_eq!(expand_params("{threads}{threads}", &params), "44");
        assert_eq!(expand_params("{unknown}", &params), "{unknown}");
        assert_eq!(expand_params("plain", &params), "plain");
    }

    #[test]
    fn invalid_matrix_fails_benchmark_with_named_problems() {
        let spec: BenchmarkSpec = serde_yaml::from_str(
            r#"
name: bad_matrix
fixture: basic_project
command: ["scan"]
matrix:
  threads: []
  temp_dir: [1]
"#,
        )
        .unwrap();
        let error = format!("{:#}", normalize_matrix(&spec).unwrap_err());
        assert!(error.contains("bad_matrix"), "{error}");
        assert!(error.contains("threads has no values"), "{error}");
        assert!(error.contains("temp_dir shadows a built-in"), "{error}");
    }

    #[test]
    fn scalar_matrix_values_are_stringified() {
        let spec: BenchmarkSpec = serde_yaml::from_str(
            r#"
name: ok_matrix
fixture: basic_project
command: ["scan", "--threads", "{threads}"]
matrix:
  threads: [1, 2]
  fast: [true, false]
  label: ["a"]
"#,
        )
        .unwrap();
        let matrix = normalize_matrix(&spec).unwrap();
        assert_eq!(matrix["threads"], vec!["1", "2"]);
        assert_eq!(matrix["fast"], vec!["true", "false"]);
        assert_eq!(matrix["label"], vec!["a"]);
        assert_eq!(cartesian_points(&matrix).len(), 4);
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
    fn speedup_uses_min_and_max_thread_medians() {
        let points = vec![
            point(&[("threads", "1")], Some(800.0)),
            point(&[("threads", "2")], Some(400.0)),
            point(&[("threads", "8")], Some(100.0)),
        ];
        let speedup = thread_speedup(&points).unwrap();
        assert_eq!(speedup.key, "speedup_1_to_8");
        assert!((speedup.value - 8.0).abs() < f64::EPSILON);
    }

    #[test]
    fn speedup_orders_thread_counts_numerically_not_lexically() {
        let points = vec![
            point(&[("threads", "2")], Some(400.0)),
            point(&[("threads", "16")], Some(100.0)),
        ];
        let speedup = thread_speedup(&points).unwrap();
        assert_eq!(speedup.key, "speedup_2_to_16");
        assert!((speedup.value - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn speedup_requires_two_distinct_thread_counts() {
        assert!(thread_speedup(&[point(&[("threads", "4")], Some(100.0))]).is_none());
        assert!(thread_speedup(&[point(&[], Some(100.0))]).is_none());
        assert!(thread_speedup(&[]).is_none());
    }

    #[test]
    fn speedup_skips_missing_medians_and_non_numeric_threads() {
        assert!(
            thread_speedup(&[
                point(&[("threads", "1")], None),
                point(&[("threads", "8")], Some(100.0)),
            ])
            .is_none()
        );
        assert!(
            thread_speedup(&[
                point(&[("threads", "one")], Some(800.0)),
                point(&[("threads", "8")], Some(100.0)),
            ])
            .is_none()
        );
    }

    #[test]
    fn speedup_requires_threads_to_be_the_only_varying_parameter() {
        let points = vec![
            point(&[("threads", "1"), ("mode", "fast")], Some(800.0)),
            point(&[("threads", "1"), ("mode", "safe")], Some(900.0)),
            point(&[("threads", "8"), ("mode", "fast")], Some(100.0)),
            point(&[("threads", "8"), ("mode", "safe")], Some(120.0)),
        ];
        assert!(thread_speedup(&points).is_none());
    }

    fn test_gate(metric: PerformanceMetric, params: &[(&str, &str)]) -> PerformanceGate {
        PerformanceGate {
            name: "test gate".to_string(),
            benchmark: "scan_basic_project".to_string(),
            metric,
            min: Some(2.0),
            max: None,
            params: params
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
            advice: Vec::new(),
        }
    }

    #[test]
    fn gate_evaluation_matches_the_exact_matrix_point() {
        let stage = StageSpec {
            id: "01_scan_files".to_string(),
            title: "Scan".to_string(),
            path: PathBuf::from("stages/01_scan_files"),
        };
        let mut record = sample_record();
        record.points = vec![
            point(&[("mode", "fast"), ("threads", "1")], Some(10.0)),
            point(&[("mode", "fast"), ("threads", "8")], Some(3.0)),
        ];
        let gate = test_gate(
            PerformanceMetric::RuntimeMedianMs,
            &[("mode", "fast"), ("threads", "8")],
        );

        let evaluated = evaluate_gate(&stage, &gate, &[&record]).unwrap();

        assert_eq!(evaluated.measured, Some(3.0));
        assert!(evaluated.passed);
    }

    #[test]
    fn gate_evaluation_rejects_ambiguous_and_non_finite_points() {
        let stage = StageSpec {
            id: "01_scan_files".to_string(),
            title: "Scan".to_string(),
            path: PathBuf::from("stages/01_scan_files"),
        };
        let gate = test_gate(PerformanceMetric::RuntimeMedianMs, &[("threads", "8")]);
        let mut ambiguous = sample_record();
        ambiguous.points = vec![
            point(&[("threads", "8")], Some(3.0)),
            point(&[("threads", "8")], Some(4.0)),
        ];
        assert_eq!(
            evaluate_gate(&stage, &gate, &[&ambiguous])
                .unwrap()
                .measured,
            None
        );

        let mut non_finite = sample_record();
        non_finite.points = vec![point(&[("threads", "8")], Some(f64::INFINITY))];
        let evaluated = evaluate_gate(&stage, &gate, &[&non_finite]).unwrap();
        assert_eq!(evaluated.measured, None);
        assert!(!evaluated.passed);
    }

    #[test]
    fn speedup_gate_uses_the_derived_extreme_thread_result() {
        let stage = StageSpec {
            id: "01_scan_files".to_string(),
            title: "Scan".to_string(),
            path: PathBuf::from("stages/01_scan_files"),
        };
        let mut record = sample_record();
        record.points = vec![
            point(&[("threads", "1")], Some(800.0)),
            point(&[("threads", "2")], Some(500.0)),
            point(&[("threads", "8")], Some(100.0)),
        ];
        let gate = test_gate(PerformanceMetric::Speedup, &[]);

        let evaluated = evaluate_gate(&stage, &gate, &[&record]).unwrap();

        assert_eq!(evaluated.measured, Some(8.0));
        assert!(evaluated.passed);
    }

    #[test]
    fn incomplete_stage_suppresses_selected_point_passes() {
        let mut gates = vec![EvaluatedGate {
            name: "selected point".to_string(),
            benchmark: "scan_basic_project".to_string(),
            metric: PerformanceMetric::RuntimeMedianMs,
            params: BTreeMap::new(),
            bound: GateBound::Max(100.0),
            measured: Some(10.0),
            passed: true,
            advice: Vec::new(),
        }];

        suppress_incomplete_passes(&mut gates, false);

        assert!(!gates[0].passed);
        assert_eq!(gates[0].measured, Some(10.0));
    }

    #[test]
    fn missing_history_file_reads_as_empty() {
        let path = std::env::temp_dir().join(format!(
            "deltaforge-bench-missing-history-{}.json",
            std::process::id()
        ));
        assert!(read_history(&path).unwrap().is_empty());
    }
}
