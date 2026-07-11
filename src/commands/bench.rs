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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRecord {
    pub project: String,
    pub language: String,
    pub stage: String,
    pub benchmark: String,
    pub timestamp: String,
    pub git_commit: Option<String>,
    pub command: Vec<String>,
    pub results: BenchmarkResult,
    pub machine: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub success: bool,
    pub iterations: u64,
    pub warmup: u64,
    pub runtime_median_ms: Option<f64>,
    pub runtime_p95_ms: Option<f64>,
    pub throughput_mb_s: Option<f64>,
    pub error: Option<String>,
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
            if record.results.success {
                println!(
                    "  median: {:.2} ms",
                    record.results.runtime_median_ms.unwrap_or_default()
                );
                println!(
                    "  p95: {:.2} ms",
                    record.results.runtime_p95_ms.unwrap_or_default()
                );
                if let Some(throughput) = record.results.throughput_mb_s {
                    println!("  throughput: {throughput:.2} MB/s");
                }
            } else if let Some(error) = &record.results.error {
                println!("  failed: {error}");
            }
        }
        if args.save {
            println!(
                "Saved benchmark history: {}",
                history_path(&context).display()
            );
        }
    }

    if records.iter().any(|record| !record.results.success) {
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

    let mut records = Vec::new();
    for benchmark in parsed.benchmarks {
        records.push(run_one_benchmark(
            context,
            stage,
            &language.run,
            benchmark,
            args,
        )?);
    }
    Ok(records)
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
    let result = run_iterations(
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
        results: result,
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
) -> BenchmarkResult {
    for _ in 0..warmup {
        if let Err(error) = reset_fixture(fixture.source, fixture.path) {
            return failed_result(iterations, warmup, error);
        }
        if let Err(error) = run_timed_command(command, cwd, timeout_ms) {
            return failed_result(iterations, warmup, error);
        }
    }

    let mut durations = Vec::new();
    for _ in 0..iterations {
        if let Err(error) = reset_fixture(fixture.source, fixture.path) {
            return failed_result(iterations, warmup, error);
        }
        match run_timed_command(command, cwd, timeout_ms) {
            Ok(duration) => durations.push(duration.as_secs_f64() * 1000.0),
            Err(error) => return failed_result(iterations, warmup, error),
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

    BenchmarkResult {
        success: true,
        iterations,
        warmup,
        runtime_median_ms: median,
        runtime_p95_ms: p95,
        throughput_mb_s: throughput,
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

fn failed_result(iterations: u64, warmup: u64, error: anyhow::Error) -> BenchmarkResult {
    BenchmarkResult {
        success: false,
        iterations,
        warmup,
        runtime_median_ms: None,
        runtime_p95_ms: None,
        throughput_mb_s: None,
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
    let mut history = read_history(&path)?;
    history.extend_from_slice(records);
    atomic_write(&path, serde_json::to_string_pretty(&history)?)
}

pub fn read_history(path: &Path) -> Result<Vec<BenchmarkRecord>> {
    match fs::read_to_string(path) {
        Ok(source) => serde_json::from_str(&source)
            .with_context(|| format!("failed to parse benchmark history {}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error)
            .with_context(|| format!("failed to read benchmark history {}", path.display())),
    }
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
}
