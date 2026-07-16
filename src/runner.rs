use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Output};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::application::{EventSink, RunEvent};
use crate::context::ProjectContext;
use crate::integrity::is_safe_relative_path;
use crate::pack::{CommandSpec, StageSpec, safe_expectation_path};
use crate::process;

#[derive(Debug, Clone, Default)]
pub struct RunnerOptions {
    pub filter: Option<String>,
    pub list_tests: bool,
    pub fail_fast: bool,
    pub no_build: bool,
    pub keep_temp: bool,
    pub capture_details: bool,
    pub cancellation_path: Option<PathBuf>,
}

impl RunnerOptions {
    fn is_cancelled(&self) -> bool {
        self.cancellation_path
            .as_ref()
            .is_some_and(|path| path.exists())
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageTests {
    #[serde(default)]
    pub tests: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestCase {
    pub name: String,
    #[serde(default)]
    pub diagnosis: Option<TestDiagnosis>,
    pub fixture: Option<String>,
    pub stdin: Option<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    #[serde(default)]
    pub command: Vec<String>,
    pub expect: Expectations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestDiagnosis {
    pub priority: u32,
    pub headline: String,
    pub contract: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Expectations {
    pub exit_code: Option<i32>,
    pub stdout_exact: Option<String>,
    #[serde(default)]
    pub stdout_contains: Vec<String>,
    #[serde(default)]
    pub stdout_not_contains: Vec<String>,
    #[serde(default)]
    pub stderr_contains: Vec<String>,
    #[serde(default)]
    pub file_exists: Vec<String>,
    #[serde(default)]
    pub file_not_exists: Vec<String>,
    #[serde(default)]
    pub file_contains: Vec<FileContainsExpectation>,
    #[serde(default)]
    pub file_not_contains: Vec<FileContainsExpectation>,
    #[serde(default)]
    pub regex_match: Vec<String>,
    pub json_equals: Option<Value>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileContainsExpectation {
    pub path: String,
    pub contains: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestRunSummary {
    pub stage_id: String,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<TestResult>,
    pub total_defined: usize,
    pub completion_eligible: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<TestDiagnosis>,
    pub failures: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<TestDiagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expectations: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_exit_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u128>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub stdout: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub stderr: String,
    #[serde(skip)]
    pub report_stdout: Option<String>,
    #[serde(skip)]
    pub report_stderr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<TestInput>,
    #[serde(skip)]
    pub kept_temp_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestInput {
    pub command: Vec<String>,
    pub stdin: Option<String>,
    pub env: BTreeMap<String, String>,
    pub timeout_ms: u64,
    pub working_directory: String,
    pub fixture_name: Option<String>,
    pub fixture: Option<FixtureSnapshot>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FixtureSnapshot {
    pub entries: Vec<FixtureEntry>,
    pub omitted_entries: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct FixtureEntry {
    pub path: String,
    pub kind: FixtureEntryKind,
    pub size_bytes: Option<u64>,
    pub preview: Option<String>,
    pub preview_kind: Option<FixturePreviewKind>,
    pub preview_truncated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FixtureEntryKind {
    Directory,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FixturePreviewKind {
    Text,
    Binary,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestDiagnostic {
    pub kind: &'static str,
    pub title: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<String>,
}

struct TestComparison {
    failures: Vec<String>,
    diagnostics: Vec<TestDiagnostic>,
}

impl TestRunSummary {
    pub fn is_success(&self) -> bool {
        self.failed == 0 && !self.results.is_empty()
    }
}

pub fn run_stage_tests(
    context: &ProjectContext,
    stage: &StageSpec,
    args: &RunnerOptions,
    sink: &mut dyn EventSink,
) -> Result<TestRunSummary> {
    if args.is_cancelled() {
        bail!("run cancelled");
    }
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

    let tests_path = context.pack.tests_path(stage);
    let source = fs::read_to_string(&tests_path)
        .with_context(|| format!("failed to read tests file {}", tests_path.display()))?;
    let stage_tests: StageTests = serde_yaml::from_str(&source)
        .with_context(|| format!("failed to parse tests file {}", tests_path.display()))?;
    let selected_tests: Vec<&TestCase> = stage_tests
        .tests
        .iter()
        .filter(|test| {
            args.filter
                .as_ref()
                .is_none_or(|pattern| test.name.contains(pattern))
        })
        .collect();
    if selected_tests.is_empty() && !args.list_tests {
        bail!("no tests matched the selected filter");
    }
    let total_defined = stage_tests.tests.len();
    for test in &stage_tests.tests {
        if let Some(fixture) = &test.fixture
            && !is_safe_relative_path(Path::new(fixture))
        {
            bail!("test {} uses unsafe fixture path {fixture}", test.name);
        }
        for path in test
            .expect
            .file_exists
            .iter()
            .chain(&test.expect.file_not_exists)
            .chain(test.expect.file_contains.iter().map(|item| &item.path))
        {
            if !safe_expectation_path(path) {
                bail!("test {} uses unsafe expectation path {path}", test.name);
            }
        }
    }

    if args.list_tests {
        let results = selected_tests
            .iter()
            .map(|test| TestResult {
                name: test.name.clone(),
                passed: true,
                diagnosis: test.diagnosis.clone(),
                failures: Vec::new(),
                diagnostics: Vec::new(),
                expectations: describe_expectations(&test.expect),
                actual_exit_code: None,
                duration_ms: None,
                stdout: String::new(),
                stderr: String::new(),
                report_stdout: None,
                report_stderr: None,
                input: None,
                kept_temp_dir: None,
            })
            .collect::<Vec<_>>();
        return Ok(TestRunSummary {
            stage_id: stage.id.clone(),
            passed: results.len(),
            failed: 0,
            results,
            total_defined,
            completion_eligible: false,
        });
    }

    if !args.no_build
        && let Some(build) = &language.build
    {
        run_build(
            build,
            &context.root,
            context.config.runner.build_timeout_ms,
            args.cancellation_path.as_deref(),
            sink,
        )?;
    }

    let mut results = Vec::new();
    let selected_count = selected_tests.len();

    for (index, test) in selected_tests.into_iter().enumerate() {
        if args.is_cancelled() {
            bail!("run cancelled");
        }
        sink.emit(RunEvent::TestStarted {
            stage_id: stage.id.clone(),
            name: test.name.clone(),
            index: index + 1,
            total: selected_count,
        });
        let result = run_test_case(context, stage, &language.run, test, args)?;
        let failed = !result.passed;
        if false {
            if result.passed {
                println!("✓ {}", test.name);
            } else {
                println!("✗ {}", test.name);
                if false {
                    for failure in &result.failures {
                        println!("  {failure}");
                    }
                }
                if false {
                    print_actual_output(&result);
                }
            }
            if false {
                if !result.stdout.is_empty() {
                    println!("stdout:\n{}", result.stdout);
                }
                if !result.stderr.is_empty() {
                    println!("stderr:\n{}", result.stderr);
                }
                println!();
            }
        }
        results.push(result.clone());
        sink.emit(if result.passed {
            RunEvent::TestPassed {
                stage_id: stage.id.clone(),
                result: result.clone(),
            }
        } else {
            RunEvent::TestFailed {
                stage_id: stage.id.clone(),
                result: result.clone(),
            }
        });
        if failed && args.fail_fast {
            break;
        }
    }

    let passed = results.iter().filter(|result| result.passed).count();
    let failed = results.len() - passed;

    Ok(TestRunSummary {
        stage_id: stage.id.clone(),
        passed,
        failed,
        results,
        total_defined,
        completion_eligible: args.filter.is_none()
            && !args.no_build
            && passed == total_defined
            && failed == 0,
    })
}

const ACTUAL_OUTPUT_MAX_LINES: usize = 30;
const ACTUAL_OUTPUT_MAX_CHARS: usize = 2000;

fn print_actual_output(result: &TestResult) {
    print_actual_stream("stdout", &result.stdout);
    if !result.stderr.trim().is_empty() {
        print_actual_stream("stderr", &result.stderr);
    }
}

fn print_actual_stream(label: &str, output: &str) {
    if output.is_empty() {
        println!("  actual {label}: (empty)");
        return;
    }

    let (body, truncated) = truncate_output(output);
    println!("  actual {label} (first {ACTUAL_OUTPUT_MAX_LINES} lines):");
    for line in body.lines() {
        println!("    {line}");
    }
    if truncated {
        println!("    … truncated, run with --verbose");
    }
}

fn truncate_output(output: &str) -> (String, bool) {
    let mut truncated = false;
    let mut body: String = output
        .lines()
        .take(ACTUAL_OUTPUT_MAX_LINES)
        .collect::<Vec<_>>()
        .join("\n");
    if output.lines().count() > ACTUAL_OUTPUT_MAX_LINES {
        truncated = true;
    }
    if body.chars().count() > ACTUAL_OUTPUT_MAX_CHARS {
        body = body.chars().take(ACTUAL_OUTPUT_MAX_CHARS).collect();
        truncated = true;
    }
    (body, truncated)
}

fn run_build(
    build: &CommandSpec,
    project_root: &Path,
    timeout_ms: u64,
    cancellation_path: Option<&Path>,
    sink: &mut dyn EventSink,
) -> Result<()> {
    if build.command.is_empty() {
        return Ok(());
    }

    sink.emit(RunEvent::BuildStarted {
        command: build.command.clone(),
    });

    let output = {
        let mut stream_output = |stream, bytes: &[u8]| {
            sink.emit(RunEvent::BuildOutput {
                stream,
                text: String::from_utf8_lossy(bytes).to_string(),
            });
        };
        process::run_command_cancellable_streaming(
            &build.command,
            project_root,
            timeout_ms,
            None,
            &BTreeMap::new(),
            cancellation_path,
            &mut stream_output,
        )?
    };
    sink.emit(RunEvent::BuildCompleted {
        passed: output.status.success(),
    });

    if !output.status.success() {
        bail!(
            "build failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

fn run_test_case(
    context: &ProjectContext,
    stage: &StageSpec,
    run_command_spec: &CommandSpec,
    test: &TestCase,
    args: &RunnerOptions,
) -> Result<TestResult> {
    let temp_dir = create_temp_dir(stage, &test.name)?;
    let keep_temp = args.keep_temp || context.config.runner.keep_temp;
    let mut result =
        match run_test_case_in_temp(context, stage, run_command_spec, test, args, &temp_dir) {
            Ok(result) => result,
            Err(error) => TestResult {
                name: test.name.clone(),
                passed: false,
                diagnosis: test.diagnosis.clone(),
                failures: vec![format!("{error:#}")],
                diagnostics: vec![TestDiagnostic {
                    kind: "runner",
                    title: "The test could not run".to_string(),
                    summary: format!("{error:#}"),
                    expected: None,
                    actual: None,
                }],
                expectations: describe_expectations(&test.expect),
                actual_exit_code: None,
                duration_ms: None,
                stdout: String::new(),
                stderr: String::new(),
                report_stdout: None,
                report_stderr: None,
                input: None,
                kept_temp_dir: None,
            },
        };

    if !keep_temp {
        if let Err(error) = fs::remove_dir_all(&temp_dir) {
            result.passed = false;
            result.failures.push(format!(
                "failed to remove temp dir {}: {error}",
                temp_dir.display()
            ));
        }
    } else {
        result.kept_temp_dir = Some(temp_dir.clone());
    }

    Ok(result)
}

fn run_test_case_in_temp(
    context: &ProjectContext,
    stage: &StageSpec,
    run_command_spec: &CommandSpec,
    test: &TestCase,
    args: &RunnerOptions,
    temp_dir: &Path,
) -> Result<TestResult> {
    let fixture_path = if let Some(fixture) = &test.fixture {
        let source_fixture = context.pack.fixture_path(stage, fixture);
        let copied_fixture = temp_dir.join("fixture");
        copy_dir_recursive(&source_fixture, &copied_fixture).with_context(|| {
            format!(
                "failed to copy fixture {} to {}",
                source_fixture.display(),
                copied_fixture.display()
            )
        })?;
        copied_fixture
    } else {
        temp_dir.to_path_buf()
    };

    let mut full_command = run_command_spec.command.clone();
    full_command.extend(
        test.command
            .iter()
            .map(|arg| expand_variables(arg, &fixture_path, temp_dir)),
    );

    let timeout_ms = test
        .expect
        .timeout_ms
        .unwrap_or(context.config.runner.timeout_ms);
    let stdin = test
        .stdin
        .as_ref()
        .map(|value| expand_variables(value, &fixture_path, temp_dir));
    let env = test
        .env
        .iter()
        .map(|(key, value)| {
            (
                key.clone(),
                expand_variables(value, &fixture_path, temp_dir),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let input = should_capture_test_input(args).then(|| TestInput {
        command: full_command
            .iter()
            .map(|value| sanitize_report_text(value, &fixture_path, temp_dir))
            .collect(),
        stdin: stdin
            .as_ref()
            .map(|value| sanitize_report_text(value, &fixture_path, temp_dir)),
        env: env
            .iter()
            .map(|(key, value)| {
                (
                    key.clone(),
                    sanitize_report_text(value, &fixture_path, temp_dir),
                )
            })
            .collect(),
        timeout_ms,
        working_directory: "{project_root}".to_string(),
        fixture_name: test.fixture.clone(),
        fixture: test
            .fixture
            .as_ref()
            .map(|_| capture_fixture_snapshot(&fixture_path)),
    });
    let started = Instant::now();
    let output = match run_command_with_input_and_env(
        &full_command,
        &context.root,
        timeout_ms,
        stdin.as_deref(),
        &env,
        args.cancellation_path.as_deref(),
    ) {
        Ok(output) => output,
        Err(error) if format!("{error:#}").contains("run cancelled") => return Err(error),
        Err(error) => {
            let message = sanitize_report_text(&format!("{error:#}"), &fixture_path, temp_dir);
            return Ok(TestResult {
                name: test.name.clone(),
                passed: false,
                diagnosis: test.diagnosis.clone(),
                failures: vec![message.clone()],
                diagnostics: vec![TestDiagnostic {
                    kind: "runner",
                    title: "The test command did not finish".to_string(),
                    summary: message,
                    expected: None,
                    actual: None,
                }],
                expectations: describe_expectations(&test.expect),
                actual_exit_code: None,
                duration_ms: Some(started.elapsed().as_millis()),
                stdout: String::new(),
                stderr: String::new(),
                report_stdout: None,
                report_stderr: None,
                input,
                kept_temp_dir: None,
            });
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let expanded_expectations = expand_output_expectations(&test.expect, &fixture_path, temp_dir);
    let comparison_stdout = normalize_comparison_paths(&stdout, &fixture_path, temp_dir);
    let comparison_stderr = normalize_comparison_paths(&stderr, &fixture_path, temp_dir);
    let mut comparison = compare_expectations(
        &expanded_expectations,
        &output.status,
        &comparison_stdout,
        &comparison_stderr,
        temp_dir,
    );
    for failure in &mut comparison.failures {
        *failure = sanitize_report_text(failure, &fixture_path, temp_dir);
    }
    for diagnostic in &mut comparison.diagnostics {
        diagnostic.summary = sanitize_report_text(&diagnostic.summary, &fixture_path, temp_dir);
        if let Some(expected) = &mut diagnostic.expected {
            *expected = sanitize_report_text(expected, &fixture_path, temp_dir);
        }
        if let Some(actual) = &mut diagnostic.actual {
            *actual = sanitize_report_text(actual, &fixture_path, temp_dir);
        }
    }
    let report_stdout = sanitize_report_text(&stdout, &fixture_path, temp_dir);
    let report_stderr = sanitize_report_text(&stderr, &fixture_path, temp_dir);

    Ok(TestResult {
        name: test.name.clone(),
        passed: comparison.failures.is_empty(),
        diagnosis: test.diagnosis.clone(),
        failures: comparison.failures,
        diagnostics: comparison.diagnostics,
        expectations: describe_expectations(&test.expect),
        actual_exit_code: output.status.code(),
        duration_ms: Some(started.elapsed().as_millis()),
        stdout,
        stderr,
        report_stdout: Some(report_stdout),
        report_stderr: Some(report_stderr),
        input,
        kept_temp_dir: None,
    })
}

fn expand_output_expectations(
    expectations: &Expectations,
    fixture_path: &Path,
    temp_dir: &Path,
) -> Expectations {
    let mut expanded = expectations.clone();
    expanded.stdout_exact = expanded
        .stdout_exact
        .map(|value| expand_variables(&value, fixture_path, temp_dir));
    for values in [
        &mut expanded.stdout_contains,
        &mut expanded.stdout_not_contains,
        &mut expanded.stderr_contains,
    ] {
        for value in values {
            *value = expand_variables(value, fixture_path, temp_dir);
        }
    }
    expanded
}

fn should_capture_test_input(args: &RunnerOptions) -> bool {
    args.capture_details
}

fn sanitize_report_text(value: &str, fixture_path: &Path, temp_dir: &Path) -> String {
    let value = replace_report_path(value, fixture_path, "{fixture_path}");
    replace_report_path(&value, temp_dir, "{temp_dir}")
}

fn normalize_comparison_paths(value: &str, fixture_path: &Path, temp_dir: &Path) -> String {
    let fixture = fixture_path.to_string_lossy();
    let value = replace_report_path(value, fixture_path, fixture.as_ref());
    let temp = temp_dir.to_string_lossy();
    replace_report_path(&value, temp_dir, temp.as_ref())
}

fn replace_report_path(value: &str, path: &Path, replacement: &str) -> String {
    equivalent_path_spellings(path)
        .iter()
        .fold(value.to_string(), |value, native| {
            replace_path_spelling(&value, native, replacement)
        })
}

fn replace_path_spelling(value: &str, native: &str, replacement: &str) -> String {
    let escaped = native.replace('\\', "\\\\");
    let value = value.replace(&escaped, replacement);
    let value = value.replace(native, replacement);
    let portable = native.replace('\\', "/");
    if portable == native {
        value
    } else {
        value.replace(&portable, replacement)
    }
}

fn equivalent_path_spellings(path: &Path) -> Vec<String> {
    #[cfg(not(windows))]
    {
        vec![path.to_string_lossy().to_string()]
    }
    #[cfg(windows)]
    {
        let mut spellings = vec![path.to_string_lossy().to_string()];
        for spelling in [windows_long_path(path), windows_short_path(path)]
            .into_iter()
            .flatten()
        {
            let spelling = spelling.to_string_lossy().to_string();
            if !spellings.contains(&spelling) {
                spellings.push(spelling);
            }
        }
        spellings
    }
}

#[cfg(windows)]
fn windows_long_path(path: &Path) -> Option<PathBuf> {
    windows_path_spelling(path, WindowsPathSpelling::Long)
}

#[cfg(windows)]
fn windows_short_path(path: &Path) -> Option<PathBuf> {
    windows_path_spelling(path, WindowsPathSpelling::Short)
}

#[cfg(windows)]
enum WindowsPathSpelling {
    Long,
    Short,
}

#[cfg(windows)]
fn windows_path_spelling(path: &Path, spelling: WindowsPathSpelling) -> Option<PathBuf> {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetLongPathNameW(short_path: *const u16, long_path: *mut u16, capacity: u32) -> u32;
        fn GetShortPathNameW(long_path: *const u16, short_path: *mut u16, capacity: u32) -> u32;
    }

    let input = path
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let required = unsafe {
        match spelling {
            WindowsPathSpelling::Long => GetLongPathNameW(input.as_ptr(), std::ptr::null_mut(), 0),
            WindowsPathSpelling::Short => {
                GetShortPathNameW(input.as_ptr(), std::ptr::null_mut(), 0)
            }
        }
    };
    if required == 0 {
        return None;
    }
    let mut output = vec![0_u16; required as usize + 1];
    let written = unsafe {
        match spelling {
            WindowsPathSpelling::Long => {
                GetLongPathNameW(input.as_ptr(), output.as_mut_ptr(), output.len() as u32)
            }
            WindowsPathSpelling::Short => {
                GetShortPathNameW(input.as_ptr(), output.as_mut_ptr(), output.len() as u32)
            }
        }
    };
    if written == 0 || written as usize >= output.len() {
        return None;
    }
    output.truncate(written as usize);
    Some(PathBuf::from(OsString::from_wide(&output)))
}

const FIXTURE_ENTRY_LIMIT: usize = 300;
const FIXTURE_TOTAL_PREVIEW_LIMIT: usize = 150_000;
const FIXTURE_FILE_PREVIEW_LIMIT: usize = 20_000;
const FIXTURE_BINARY_PREVIEW_LIMIT: usize = 256;

fn capture_fixture_snapshot(root: &Path) -> FixtureSnapshot {
    let mut discovered = Vec::new();
    collect_fixture_entries(root, root, &mut discovered);
    discovered.sort_by(|left, right| left.0.cmp(&right.0));

    let omitted_entries = discovered.len().saturating_sub(FIXTURE_ENTRY_LIMIT);
    discovered.truncate(FIXTURE_ENTRY_LIMIT);
    let mut preview_budget = FIXTURE_TOTAL_PREVIEW_LIMIT;
    let entries = discovered
        .into_iter()
        .map(|(path, kind)| {
            let absolute_path = root.join(path.replace('/', std::path::MAIN_SEPARATOR_STR));
            let size_bytes = if kind == FixtureEntryKind::File {
                fs::metadata(&absolute_path)
                    .ok()
                    .map(|metadata| metadata.len())
            } else {
                None
            };
            let (preview, preview_kind, preview_truncated) = if kind == FixtureEntryKind::File {
                fixture_file_preview(&absolute_path, size_bytes, &mut preview_budget)
            } else {
                (None, None, false)
            };
            FixtureEntry {
                path,
                kind,
                size_bytes,
                preview,
                preview_kind,
                preview_truncated,
            }
        })
        .collect();

    FixtureSnapshot {
        entries,
        omitted_entries,
    }
}

fn collect_fixture_entries(
    root: &Path,
    directory: &Path,
    entries: &mut Vec<(String, FixtureEntryKind)>,
) {
    let Ok(read_dir) = fs::read_dir(directory) else {
        return;
    };
    let mut children = read_dir.filter_map(Result::ok).collect::<Vec<_>>();
    children.sort_by_key(|entry| entry.file_name());
    for child in children {
        let path = child.path();
        let Ok(file_type) = child.file_type() else {
            continue;
        };
        let relative = portable_relative_path(root, &path);
        if file_type.is_dir() {
            entries.push((relative, FixtureEntryKind::Directory));
            collect_fixture_entries(root, &path, entries);
        } else if file_type.is_file() {
            entries.push((relative, FixtureEntryKind::File));
        }
    }
}

fn portable_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn fixture_file_preview(
    path: &Path,
    size_bytes: Option<u64>,
    preview_budget: &mut usize,
) -> (Option<String>, Option<FixturePreviewKind>, bool) {
    if *preview_budget == 0 {
        return (None, None, true);
    }
    let limit = FIXTURE_FILE_PREVIEW_LIMIT.min(*preview_budget);
    let Ok(file) = fs::File::open(path) else {
        return (None, None, false);
    };
    let mut bytes = Vec::new();
    if file
        .take((limit + 1) as u64)
        .read_to_end(&mut bytes)
        .is_err()
    {
        return (None, None, false);
    }
    let read_past_limit = bytes.len() > limit;
    bytes.truncate(limit);
    *preview_budget = preview_budget.saturating_sub(bytes.len());
    let truncated = read_past_limit || size_bytes.is_some_and(|size| size > bytes.len() as u64);

    if !bytes.contains(&0) {
        match std::str::from_utf8(&bytes) {
            Ok(text) => {
                return (
                    Some(text.to_string()),
                    Some(FixturePreviewKind::Text),
                    truncated,
                );
            }
            Err(error) if error.error_len().is_none() => {
                let text = String::from_utf8_lossy(&bytes[..error.valid_up_to()]).to_string();
                return (Some(text), Some(FixturePreviewKind::Text), true);
            }
            Err(_) => {}
        }
    }

    let binary = bytes
        .iter()
        .take(FIXTURE_BINARY_PREVIEW_LIMIT)
        .enumerate()
        .fold(String::new(), |mut output, (index, byte)| {
            if index > 0 {
                if index % 16 == 0 {
                    output.push('\n');
                } else {
                    output.push(' ');
                }
            }
            let _ = std::fmt::Write::write_fmt(&mut output, format_args!("{byte:02X}"));
            output
        });
    (
        Some(binary),
        Some(FixturePreviewKind::Binary),
        truncated || bytes.len() > FIXTURE_BINARY_PREVIEW_LIMIT,
    )
}

fn compare_expectations(
    expect: &Expectations,
    status: &ExitStatus,
    stdout: &str,
    stderr: &str,
    temp_dir: &Path,
) -> TestComparison {
    let mut failures = Vec::new();
    let mut diagnostics = Vec::new();

    if let Some(expected_code) = expect.exit_code {
        let actual_code = status.code().unwrap_or(-1);
        if actual_code != expected_code {
            let message = format!("expected exit code {expected_code}, got {actual_code}");
            failures.push(message.clone());
            diagnostics.push(TestDiagnostic {
                kind: "exit-code",
                title: "Unexpected exit code".to_string(),
                summary: message,
                expected: Some(expected_code.to_string()),
                actual: Some(actual_code.to_string()),
            });
        }
    }

    if let Some(expected_stdout) = &expect.stdout_exact
        && stdout != expected_stdout
    {
        failures.push(format!(
            "expected stdout exactly {:?}, got {:?}",
            expected_stdout, stdout
        ));
        diagnostics.push(TestDiagnostic {
            kind: "stdout-exact",
            title: "Standard output differs".to_string(),
            summary: "The program's standard output does not exactly match the required text."
                .to_string(),
            expected: Some(expected_stdout.clone()),
            actual: Some(stdout.to_string()),
        });
    }

    for expected in &expect.stdout_contains {
        if !stdout.contains(expected) {
            let message = format!("expected stdout to contain {:?}", expected);
            failures.push(message.clone());
            diagnostics.push(TestDiagnostic {
                kind: "stdout-contains",
                title: "Required text is missing".to_string(),
                summary: message,
                expected: Some(expected.clone()),
                actual: Some(stdout.to_string()),
            });
        }
    }

    for unexpected in &expect.stdout_not_contains {
        if stdout.contains(unexpected) {
            let message = format!("expected stdout not to contain {:?}", unexpected);
            failures.push(message.clone());
            diagnostics.push(TestDiagnostic {
                kind: "stdout-excludes",
                title: "Unexpected text was printed".to_string(),
                summary: message,
                expected: Some(format!("output without {unexpected:?}")),
                actual: Some(stdout.to_string()),
            });
        }
    }

    for expected in &expect.stderr_contains {
        if !stderr.contains(expected) {
            let message = format!("expected stderr to contain {:?}", expected);
            failures.push(message.clone());
            diagnostics.push(TestDiagnostic {
                kind: "stderr-contains",
                title: "The error explanation is missing".to_string(),
                summary: message,
                expected: Some(expected.clone()),
                actual: Some(stderr.to_string()),
            });
        }
    }

    for expected_path in &expect.file_exists {
        let path = resolve_expectation_path(temp_dir, expected_path);
        if !path.exists() {
            let message = format!("expected file to exist: {}", path.display());
            failures.push(message.clone());
            diagnostics.push(TestDiagnostic {
                kind: "file-exists",
                title: "Expected file was not created".to_string(),
                summary: format!("The command did not create {expected_path}."),
                expected: Some(expected_path.clone()),
                actual: Some("file not found".to_string()),
            });
        }
    }

    for unexpected_path in &expect.file_not_exists {
        let path = resolve_expectation_path(temp_dir, unexpected_path);
        if path.exists() {
            let message = format!("expected file not to exist: {}", path.display());
            failures.push(message.clone());
            diagnostics.push(TestDiagnostic {
                kind: "file-absent",
                title: "An unwanted file was created".to_string(),
                summary: format!("The command created {unexpected_path}, which should be absent."),
                expected: Some(format!("{} absent", unexpected_path)),
                actual: Some("file exists".to_string()),
            });
        }
    }

    for expected_file in &expect.file_contains {
        let path = resolve_expectation_path(temp_dir, &expected_file.path);
        match fs::read_to_string(&path) {
            Ok(contents) if contents.contains(&expected_file.contains) => {}
            Ok(contents) => {
                let message = format!(
                    "expected file {} to contain {:?}",
                    path.display(),
                    expected_file.contains
                );
                failures.push(message.clone());
                diagnostics.push(TestDiagnostic {
                    kind: "file-contains",
                    title: "Generated file is missing required text".to_string(),
                    summary: format!(
                        "{} does not contain {:?}.",
                        expected_file.path, expected_file.contains
                    ),
                    expected: Some(expected_file.contains.clone()),
                    actual: Some(contents),
                });
            }
            Err(error) => {
                let message = format!("failed to read file {}: {error}", path.display());
                failures.push(message.clone());
                diagnostics.push(TestDiagnostic {
                    kind: "file-read",
                    title: "Generated file could not be read".to_string(),
                    summary: format!("{} could not be read: {error}", expected_file.path),
                    expected: Some(expected_file.path.clone()),
                    actual: None,
                });
            }
        }
    }

    for unexpected_file in &expect.file_not_contains {
        let path = resolve_expectation_path(temp_dir, &unexpected_file.path);
        match fs::read_to_string(&path) {
            Ok(contents) if !contents.contains(&unexpected_file.contains) => {}
            Ok(contents) => {
                let message = format!(
                    "expected file {} not to contain {:?}",
                    path.display(),
                    unexpected_file.contains
                );
                failures.push(message.clone());
                diagnostics.push(TestDiagnostic {
                    kind: "file-excludes",
                    title: "Generated file retained unwanted text".to_string(),
                    summary: format!(
                        "{} still contains {:?}.",
                        unexpected_file.path, unexpected_file.contains
                    ),
                    expected: Some(format!("file without {:?}", unexpected_file.contains)),
                    actual: Some(contents),
                });
            }
            Err(error) => {
                let message = format!("failed to read file {}: {error}", path.display());
                failures.push(message.clone());
                diagnostics.push(TestDiagnostic {
                    kind: "file-read",
                    title: "Generated file could not be read".to_string(),
                    summary: format!("{} could not be read: {error}", unexpected_file.path),
                    expected: Some(unexpected_file.path.clone()),
                    actual: None,
                });
            }
        }
    }

    for pattern in &expect.regex_match {
        match Regex::new(pattern) {
            Ok(regex) if regex.is_match(stdout) => {}
            Ok(_) => {
                let message = format!("expected stdout to match regex {pattern:?}");
                failures.push(message.clone());
                diagnostics.push(TestDiagnostic {
                    kind: "stdout-regex",
                    title: "Output does not match the required pattern".to_string(),
                    summary: message,
                    expected: Some(pattern.clone()),
                    actual: Some(stdout.to_string()),
                });
            }
            Err(error) => {
                let message = format!("invalid regex {pattern:?}: {error}");
                failures.push(message.clone());
                diagnostics.push(TestDiagnostic {
                    kind: "invalid-regex",
                    title: "The pack contains an invalid test pattern".to_string(),
                    summary: message,
                    expected: Some(pattern.clone()),
                    actual: None,
                });
            }
        }
    }

    if let Some(expected_json) = &expect.json_equals {
        match serde_json::from_str::<Value>(stdout) {
            Ok(actual_json) if &actual_json == expected_json => {}
            Ok(actual_json) => {
                let message = format!(
                    "expected stdout JSON {}, got {}",
                    expected_json, actual_json
                );
                failures.push(message.clone());
                diagnostics.push(TestDiagnostic {
                    kind: "json",
                    title: "JSON values differ".to_string(),
                    summary: message,
                    expected: Some(pretty_json(expected_json)),
                    actual: Some(pretty_json(&actual_json)),
                });
            }
            Err(error) => {
                let message = format!("expected stdout to be JSON: {error}");
                failures.push(message.clone());
                diagnostics.push(TestDiagnostic {
                    kind: "json-parse",
                    title: "Standard output is not valid JSON".to_string(),
                    summary: message,
                    expected: Some(pretty_json(expected_json)),
                    actual: Some(stdout.to_string()),
                });
            }
        }
    }

    TestComparison {
        failures,
        diagnostics,
    }
}

fn pretty_json(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
}

fn describe_expectations(expect: &Expectations) -> Vec<String> {
    let mut descriptions = Vec::new();
    if let Some(code) = expect.exit_code {
        descriptions.push(format!("exit code is {code}"));
    }
    if let Some(stdout) = &expect.stdout_exact {
        descriptions.push(format!("stdout exactly matches {stdout:?}"));
    }
    descriptions.extend(
        expect
            .stdout_contains
            .iter()
            .map(|value| format!("stdout contains {value:?}")),
    );
    descriptions.extend(
        expect
            .stdout_not_contains
            .iter()
            .map(|value| format!("stdout excludes {value:?}")),
    );
    descriptions.extend(
        expect
            .stderr_contains
            .iter()
            .map(|value| format!("stderr contains {value:?}")),
    );
    descriptions.extend(
        expect
            .file_exists
            .iter()
            .map(|path| format!("file exists: {path}")),
    );
    descriptions.extend(
        expect
            .file_not_exists
            .iter()
            .map(|path| format!("file is absent: {path}")),
    );
    descriptions.extend(
        expect
            .file_contains
            .iter()
            .map(|item| format!("file {} contains {:?}", item.path, item.contains)),
    );
    descriptions.extend(
        expect
            .file_not_contains
            .iter()
            .map(|item| format!("file {} excludes {:?}", item.path, item.contains)),
    );
    descriptions.extend(
        expect
            .regex_match
            .iter()
            .map(|pattern| format!("stdout matches {pattern:?}")),
    );
    if let Some(value) = &expect.json_equals {
        descriptions.push(format!("stdout JSON equals {}", value));
    }
    descriptions
}

fn run_command_with_input_and_env(
    command: &[String],
    cwd: &Path,
    timeout_ms: u64,
    stdin: Option<&str>,
    envs: &BTreeMap<String, String>,
    cancellation_path: Option<&Path>,
) -> Result<Output> {
    process::run_command_cancellable(command, cwd, timeout_ms, stdin, envs, cancellation_path)
}

fn create_temp_dir(stage: &StageSpec, test_name: &str) -> Result<PathBuf> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?
        .as_nanos();
    let name = format!(
        "deltaforge-{}-{}-{}-{}",
        std::process::id(),
        timestamp,
        stage.id,
        sanitize_name(test_name)
    );
    let path = std::env::temp_dir().join(name);
    fs::create_dir_all(&path)
        .with_context(|| format!("failed to create temp dir {}", path.display()))?;
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

fn resolve_expectation_path(temp_dir: &Path, value: &str) -> PathBuf {
    let expanded = value.replace("{temp_dir}", &temp_dir.to_string_lossy());
    let path = PathBuf::from(expanded);
    if path.is_absolute() {
        path
    } else {
        temp_dir.join(path)
    }
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    if !source.is_dir() {
        bail!("fixture directory does not exist: {}", source.display());
    }

    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create directory {}", destination.display()))?;

    for entry in fs::read_dir(source)
        .with_context(|| format!("failed to read directory {}", source.display()))?
    {
        let entry = entry
            .with_context(|| format!("failed to read directory entry in {}", source.display()))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to inspect {}", source_path.display()))?;

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            copy_file(&source_path, &destination_path)?;
        }
    }

    Ok(())
}

fn copy_file(source: &Path, destination: &Path) -> Result<u64> {
    fs::copy(source, destination).with_context(|| {
        format!(
            "failed to copy {} to {}",
            source.display(),
            destination.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_stage_tests_yaml() {
        let source = r#"
tests:
  - name: finds a file
    fixture: basic_project
    command: ["scan", "{fixture_path}"]
    expect:
      exit_code: 0
      stdout_contains:
        - "src/main.rs"
      stdout_not_contains:
        - "target"
"#;

        let parsed: StageTests = serde_yaml::from_str(source).unwrap();

        assert_eq!(parsed.tests.len(), 1);
        assert_eq!(parsed.tests[0].name, "finds a file");
        assert_eq!(parsed.tests[0].fixture.as_deref(), Some("basic_project"));
        assert_eq!(parsed.tests[0].expect.exit_code, Some(0));
        assert_eq!(parsed.tests[0].expect.stdout_contains, ["src/main.rs"]);
        assert_eq!(parsed.tests[0].expect.stdout_not_contains, ["target"]);
    }

    #[test]
    fn expands_fixture_and_temp_variables() {
        let fixture = Path::new("/tmp/fixture");
        let temp = Path::new("/tmp/run");

        assert_eq!(
            expand_variables("{fixture_path}/src:{temp_dir}", fixture, temp),
            "/tmp/fixture/src:/tmp/run"
        );
    }

    #[test]
    fn expands_fixture_paths_in_output_expectations() {
        let source = r#"
exit_code: 0
stdout_not_contains: ["{fixture_path}"]
stderr_contains: ["work: {temp_dir}"]
"#;
        let expectations: Expectations = serde_yaml::from_str(source).unwrap();
        let expanded = expand_output_expectations(
            &expectations,
            Path::new("/tmp/fixture"),
            Path::new("/tmp/run"),
        );

        assert_eq!(expanded.stdout_not_contains, ["/tmp/fixture"]);
        assert_eq!(expanded.stderr_contains, ["work: /tmp/run"]);
    }

    #[test]
    fn sanitizes_portable_windows_paths_in_report_output() {
        let fixture = Path::new(r"C:\Temp\deltaforge-123\fixture");
        let temp = Path::new(r"C:\Temp\deltaforge-123");

        assert_eq!(
            sanitize_report_text(
                concat!(
                    "C:/Temp/deltaforge-123/fixture/src/main.rs\n",
                    r#"args: ["C:\\Temp\\deltaforge-123\\fixture"]"#,
                    "\nC:/Temp/deltaforge-123/log.txt",
                ),
                fixture,
                temp,
            ),
            concat!(
                "{fixture_path}/src/main.rs\n",
                r#"args: ["{fixture_path}"]"#,
                "\n{temp_dir}/log.txt",
            )
        );
    }

    #[cfg(windows)]
    #[test]
    fn normalizes_equivalent_windows_path_spellings() {
        let root = std::env::temp_dir().join(format!(
            "deltaforge-path-alias-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        let original = root.to_string_lossy().to_string();
        if let Some(alias) = equivalent_path_spellings(&root)
            .into_iter()
            .find(|spelling| spelling != &original)
        {
            assert_eq!(
                replace_report_path(&format!(r#"args: ["{alias}"]"#), &root, "{temp_dir}"),
                r#"args: ["{temp_dir}"]"#
            );
        }
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn fixture_snapshot_captures_portable_tree_and_bounded_previews() {
        let fixture = tempfile_dir_for_test();
        std::fs::create_dir_all(fixture.join("src").join("empty")).unwrap();
        std::fs::write(fixture.join("README.md"), "hello\n").unwrap();
        std::fs::write(fixture.join("src").join("main.rs"), "fn main() {}\n").unwrap();
        std::fs::write(fixture.join("bytes.bin"), [0, 1, 255]).unwrap();

        let snapshot = capture_fixture_snapshot(&fixture);
        let readme = snapshot
            .entries
            .iter()
            .find(|entry| entry.path == "README.md")
            .unwrap();
        let binary = snapshot
            .entries
            .iter()
            .find(|entry| entry.path == "bytes.bin")
            .unwrap();

        assert_eq!(snapshot.omitted_entries, 0);
        assert!(snapshot.entries.iter().any(|entry| {
            entry.path == "src/empty" && entry.kind == FixtureEntryKind::Directory
        }));
        assert_eq!(readme.preview.as_deref(), Some("hello\n"));
        assert_eq!(readme.preview_kind, Some(FixturePreviewKind::Text));
        assert_eq!(binary.preview.as_deref(), Some("00 01 FF"));
        assert_eq!(binary.preview_kind, Some(FixturePreviewKind::Binary));

        std::fs::remove_dir_all(fixture).unwrap();
    }

    #[test]
    fn compares_regex_json_and_file_expectations() {
        let temp = tempfile_dir_for_test();
        std::fs::write(temp.join("created.txt"), "ok").unwrap();
        let status = successful_status();
        let expect = Expectations {
            exit_code: Some(0),
            stdout_exact: None,
            stdout_contains: vec!["name".to_string()],
            stdout_not_contains: vec!["missing".to_string()],
            stderr_contains: vec!["".to_string()],
            file_exists: vec!["created.txt".to_string()],
            file_not_exists: vec!["absent.txt".to_string()],
            file_contains: Vec::new(),
            file_not_contains: vec![FileContainsExpectation {
                path: "created.txt".to_string(),
                contains: "stale".to_string(),
            }],
            regex_match: vec![r#""name"\s*:\s*"delta""#.to_string()],
            json_equals: Some(serde_json::json!({"name": "delta"})),
            timeout_ms: None,
        };

        let comparison = compare_expectations(&expect, &status, r#"{"name":"delta"}"#, "", &temp);

        let _ = std::fs::remove_dir_all(temp);
        assert_eq!(comparison.failures, Vec::<String>::new());
        assert!(comparison.diagnostics.is_empty());
    }

    #[cfg(unix)]
    fn successful_status() -> ExitStatus {
        use std::os::unix::process::ExitStatusExt;
        ExitStatus::from_raw(0)
    }

    #[cfg(windows)]
    fn successful_status() -> ExitStatus {
        use std::os::windows::process::ExitStatusExt;
        ExitStatus::from_raw(0)
    }

    fn tempfile_dir_for_test() -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "deltaforge-runner-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&path).unwrap();
        path
    }
}
