use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::cli::BenchArgs;
use crate::context::{GlobalOptions, ProjectContext};
use crate::fs_util::atomic_write;
use crate::pack::{CommandSpec, StageSpec};
use crate::process;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BenchmarksFile {
    #[serde(default)]
    benchmarks: Vec<BenchmarkSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BenchmarkSpec {
    name: String,
    fixture: String,
    #[serde(default)]
    command: Vec<String>,
    iterations: Option<u64>,
    warmup: Option<u64>,
    timeout_ms: Option<u64>,
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

pub fn run(args: BenchArgs, options: &GlobalOptions) -> Result<()> {
    if args.iterations == Some(0) {
        bail!("benchmark iterations must be greater than 0");
    }
    let context = ProjectContext::load(options)?;
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

    let mut records = Vec::new();
    for stage in &stages {
        records.extend(run_stage_benchmarks(&context, stage, &args)?);
    }

    if args.save && !records.is_empty() {
        append_history(&context, &records)?;
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&records)?);
    } else if records.is_empty() {
        println!("No benchmarks defined for selected stage(s).");
    } else {
        for record in &records {
            println!("{} / {}", record.stage, record.benchmark);
            for point in &record.points {
                let label = point.params_label();
                let indent = if label.is_empty() {
                    "  ".to_string()
                } else {
                    println!("  [{label}]");
                    "    ".to_string()
                };
                if point.success {
                    println!(
                        "{indent}median: {:.2} ms",
                        point.runtime_median_ms.unwrap_or_default()
                    );
                    println!(
                        "{indent}p95: {:.2} ms",
                        point.runtime_p95_ms.unwrap_or_default()
                    );
                    if let Some(throughput) = point.throughput_mb_s {
                        println!("{indent}throughput: {throughput:.2} MB/s");
                    }
                    if let Some(peak) = point.peak_memory_mb {
                        println!("{indent}peak memory: {peak:.1} MB");
                    }
                } else if let Some(error) = &point.error {
                    println!("{indent}failed: {error}");
                }
            }
        }
        if args.save {
            println!(
                "Saved benchmark history: {}",
                history_path(&context).display()
            );
        }
    }

    if records
        .iter()
        .any(|record| record.points.iter().any(|point| !point.success))
    {
        bail!("one or more benchmarks failed");
    }
    Ok(())
}

fn run_stage_benchmarks(
    context: &ProjectContext,
    stage: &StageSpec,
    args: &BenchArgs,
) -> Result<Vec<BenchmarkRecord>> {
    let path = context.pack.benchmarks_path(stage);
    if !path.is_file() {
        return Ok(Vec::new());
    }

    let source = fs::read_to_string(&path)
        .with_context(|| format!("failed to read benchmarks file {}", path.display()))?;
    let parsed: BenchmarksFile = serde_yaml::from_str(&source)
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
        let output = run_command(
            &build.command,
            &context.root,
            context.config.runner.build_timeout_ms,
        )?;
        if !output.status.success() {
            bail!(
                "benchmark build failed\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let bench_command = resolve_bench_command(language.bench_command(), &context.root);

    let mut records = Vec::new();
    for benchmark in parsed.benchmarks {
        records.push(run_one_benchmark(
            context,
            stage,
            &bench_command,
            benchmark,
            args,
        )?);
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
    args: &BenchArgs,
) -> Result<BenchmarkRecord> {
    if benchmark.command.is_empty() {
        bail!("benchmark {} command is empty", benchmark.name);
    }

    let temp_dir = create_temp_dir(stage, &benchmark.name)?;
    let source_fixture = context.pack.fixture_path(stage, &benchmark.fixture);
    let fixture_path = temp_dir.join("fixture");
    copy_dir_recursive(&source_fixture, &fixture_path)?;
    let fixture_bytes = directory_size(&fixture_path)?;

    let mut command = run_spec.command.clone();
    command.extend(
        benchmark
            .command
            .iter()
            .map(|arg| expand_variables(arg, &fixture_path, &temp_dir)),
    );

    let iterations = args
        .iterations
        .or(benchmark.iterations)
        .unwrap_or(context.config.bench.iterations);
    if iterations == 0 {
        bail!(
            "benchmark {} iterations must be greater than 0",
            benchmark.name
        );
    }
    let warmup = args
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
    let point = run_iterations(
        &command,
        &context.root,
        iterations,
        warmup,
        timeout_ms,
        &fixture,
    );

    let _ = fs::remove_dir_all(&temp_dir);

    Ok(BenchmarkRecord {
        project: context.state.project.clone(),
        language: context.state.language.clone(),
        stage: stage.id.clone(),
        benchmark: benchmark.name,
        timestamp: current_timestamp()?,
        git_commit: current_git_commit(&context.root),
        command,
        points: vec![point],
        machine: machine_metadata(),
    })
}

struct BenchmarkFixture<'a> {
    bytes: u64,
    source: &'a Path,
    path: &'a Path,
}

fn run_iterations(
    command: &[String],
    cwd: &Path,
    iterations: u64,
    warmup: u64,
    timeout_ms: u64,
    fixture: &BenchmarkFixture<'_>,
) -> BenchmarkPoint {
    for _ in 0..warmup {
        if let Err(error) = reset_fixture(fixture.source, fixture.path) {
            return failed_point(iterations, warmup, error);
        }
        if let Err(error) = run_timed_command(command, cwd, timeout_ms) {
            return failed_point(iterations, warmup, error);
        }
    }

    let mut durations = Vec::new();
    for _ in 0..iterations {
        if let Err(error) = reset_fixture(fixture.source, fixture.path) {
            return failed_point(iterations, warmup, error);
        }
        match run_timed_command(command, cwd, timeout_ms) {
            Ok(duration) => durations.push(duration.as_secs_f64() * 1000.0),
            Err(error) => return failed_point(iterations, warmup, error),
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
        params: BTreeMap::new(),
        success: true,
        iterations,
        warmup,
        runtime_median_ms: median,
        runtime_p95_ms: p95,
        throughput_mb_s: throughput,
        peak_memory_mb: None,
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

fn failed_point(iterations: u64, warmup: u64, error: anyhow::Error) -> BenchmarkPoint {
    BenchmarkPoint {
        params: BTreeMap::new(),
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

fn append_history(context: &ProjectContext, records: &[BenchmarkRecord]) -> Result<()> {
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

fn run_timed_command(command: &[String], cwd: &Path, timeout_ms: u64) -> Result<Duration> {
    let start = Instant::now();
    let output = run_command(command, cwd, timeout_ms)?;
    if !output.status.success() {
        bail!(
            "benchmark command failed: {}\nstdout:\n{}\nstderr:\n{}",
            command.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(start.elapsed())
}

fn run_command(command: &[String], cwd: &Path, timeout_ms: u64) -> Result<Output> {
    process::run_command(command, cwd, timeout_ms, None, &BTreeMap::new())
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

    #[test]
    fn missing_history_file_reads_as_empty() {
        let path = std::env::temp_dir().join(format!(
            "deltaforge-bench-missing-history-{}.json",
            std::process::id()
        ));
        assert!(read_history(&path).unwrap().is_empty());
    }
}
