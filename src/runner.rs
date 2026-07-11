use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Output};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cli::TestArgs;
use crate::context::ProjectContext;
use crate::integrity::is_safe_relative_path;
use crate::pack::{CommandSpec, StageSpec, safe_expectation_path};
use crate::process;

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
    pub fixture: Option<String>,
    pub stdin: Option<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    #[serde(default)]
    pub command: Vec<String>,
    pub expect: Expectations,
}

#[derive(Debug, Deserialize)]
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
    pub regex_match: Vec<String>,
    pub json_equals: Option<Value>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
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
    pub failures: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub stdout: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub stderr: String,
}

impl TestRunSummary {
    pub fn is_success(&self) -> bool {
        self.failed == 0 && !self.results.is_empty()
    }
}

pub fn run_stage_tests(
    context: &ProjectContext,
    stage: &StageSpec,
    args: &TestArgs,
) -> Result<TestRunSummary> {
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

    let human_output = !args.json;
    if human_output {
        println!("Stage {}: {}", stage.id, stage.title);
        println!();
    }

    if args.list_tests {
        let results = selected_tests
            .iter()
            .map(|test| TestResult {
                name: test.name.clone(),
                passed: true,
                failures: Vec::new(),
                stdout: String::new(),
                stderr: String::new(),
            })
            .collect::<Vec<_>>();
        if human_output {
            for test in &selected_tests {
                println!("{}", test.name);
            }
        }
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
            args.verbose && !args.json,
        )?;
    }

    let mut results = Vec::new();

    for test in selected_tests {
        let result = run_test_case(context, stage, &language.run, test, args)?;
        let failed = !result.passed;
        if human_output {
            if result.passed {
                println!("✓ {}", test.name);
            } else {
                println!("✗ {}", test.name);
                for failure in &result.failures {
                    println!("  {failure}");
                }
            }
            if args.verbose {
                if !result.stdout.is_empty() {
                    println!("stdout:\n{}", result.stdout);
                }
                if !result.stderr.is_empty() {
                    println!("stderr:\n{}", result.stderr);
                }
                println!();
            }
        }
        results.push(result);
        if failed && args.fail_fast {
            break;
        }
    }

    let passed = results.iter().filter(|result| result.passed).count();
    let failed = results.len() - passed;

    if human_output {
        println!();
        if failed == 0 {
            println!("{passed} passed, 0 failed");
            println!("Stage passed.");
            println!("Run deltaforge next to continue.");
        } else {
            println!("{passed} passed, {failed} failed");
        }
    }

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

fn run_build(
    build: &CommandSpec,
    project_root: &Path,
    timeout_ms: u64,
    verbose: bool,
) -> Result<()> {
    if build.command.is_empty() {
        return Ok(());
    }

    if verbose {
        println!("Build: {}", build.command.join(" "));
    }

    let output = run_command(&build.command, project_root, timeout_ms)?;
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
    args: &TestArgs,
) -> Result<TestResult> {
    let temp_dir = create_temp_dir(stage, &test.name)?;
    let keep_temp = args.keep_temp || context.config.runner.keep_temp;
    let mut result =
        match run_test_case_in_temp(context, stage, run_command_spec, test, args, &temp_dir) {
            Ok(result) => result,
            Err(error) => TestResult {
                name: test.name.clone(),
                passed: false,
                failures: vec![format!("{error:#}")],
                stdout: String::new(),
                stderr: String::new(),
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
    } else if args.verbose && !args.json {
        println!("Kept temp dir: {}", temp_dir.display());
    }

    Ok(result)
}

fn run_test_case_in_temp(
    context: &ProjectContext,
    stage: &StageSpec,
    run_command_spec: &CommandSpec,
    test: &TestCase,
    args: &TestArgs,
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

    if args.verbose && !args.json {
        println!("Command: {}", full_command.join(" "));
    }

    let timeout_ms = test
        .expect
        .timeout_ms
        .unwrap_or(context.config.runner.timeout_ms);
    let output = run_command_with_input_and_env(
        &full_command,
        &context.root,
        timeout_ms,
        test.stdin.as_deref(),
        &test.env,
    )?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let failures = compare_expectations(&test.expect, &output.status, &stdout, &stderr, temp_dir);

    Ok(TestResult {
        name: test.name.clone(),
        passed: failures.is_empty(),
        failures,
        stdout,
        stderr,
    })
}

fn compare_expectations(
    expect: &Expectations,
    status: &ExitStatus,
    stdout: &str,
    stderr: &str,
    temp_dir: &Path,
) -> Vec<String> {
    let mut failures = Vec::new();

    if let Some(expected_code) = expect.exit_code {
        let actual_code = status.code().unwrap_or(-1);
        if actual_code != expected_code {
            failures.push(format!(
                "expected exit code {expected_code}, got {actual_code}"
            ));
        }
    }

    if let Some(expected_stdout) = &expect.stdout_exact
        && stdout != expected_stdout
    {
        failures.push(format!(
            "expected stdout exactly {:?}, got {:?}",
            expected_stdout, stdout
        ));
    }

    for expected in &expect.stdout_contains {
        if !stdout.contains(expected) {
            failures.push(format!("expected stdout to contain {:?}", expected));
        }
    }

    for unexpected in &expect.stdout_not_contains {
        if stdout.contains(unexpected) {
            failures.push(format!("expected stdout not to contain {:?}", unexpected));
        }
    }

    for expected in &expect.stderr_contains {
        if !stderr.contains(expected) {
            failures.push(format!("expected stderr to contain {:?}", expected));
        }
    }

    for expected_path in &expect.file_exists {
        let path = resolve_expectation_path(temp_dir, expected_path);
        if !path.exists() {
            failures.push(format!("expected file to exist: {}", path.display()));
        }
    }

    for unexpected_path in &expect.file_not_exists {
        let path = resolve_expectation_path(temp_dir, unexpected_path);
        if path.exists() {
            failures.push(format!("expected file not to exist: {}", path.display()));
        }
    }

    for expected_file in &expect.file_contains {
        let path = resolve_expectation_path(temp_dir, &expected_file.path);
        match fs::read_to_string(&path) {
            Ok(contents) if contents.contains(&expected_file.contains) => {}
            Ok(_) => failures.push(format!(
                "expected file {} to contain {:?}",
                path.display(),
                expected_file.contains
            )),
            Err(error) => failures.push(format!("failed to read file {}: {error}", path.display())),
        }
    }

    for pattern in &expect.regex_match {
        match Regex::new(pattern) {
            Ok(regex) if regex.is_match(stdout) => {}
            Ok(_) => failures.push(format!("expected stdout to match regex {pattern:?}")),
            Err(error) => failures.push(format!("invalid regex {pattern:?}: {error}")),
        }
    }

    if let Some(expected_json) = &expect.json_equals {
        match serde_json::from_str::<Value>(stdout) {
            Ok(actual_json) if &actual_json == expected_json => {}
            Ok(actual_json) => failures.push(format!(
                "expected stdout JSON {}, got {}",
                expected_json, actual_json
            )),
            Err(error) => failures.push(format!("expected stdout to be JSON: {error}")),
        }
    }

    failures
}

fn run_command(command: &[String], cwd: &Path, timeout_ms: u64) -> Result<Output> {
    run_command_with_input_and_env(command, cwd, timeout_ms, None, &BTreeMap::new())
}

fn run_command_with_input_and_env(
    command: &[String],
    cwd: &Path,
    timeout_ms: u64,
    stdin: Option<&str>,
    envs: &BTreeMap<String, String>,
) -> Result<Output> {
    process::run_command(command, cwd, timeout_ms, stdin, envs)
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
            regex_match: vec![r#""name"\s*:\s*"delta""#.to_string()],
            json_equals: Some(serde_json::json!({"name": "delta"})),
            timeout_ms: None,
        };

        let failures = compare_expectations(&expect, &status, r#"{"name":"delta"}"#, "", &temp);

        let _ = std::fs::remove_dir_all(temp);
        assert_eq!(failures, Vec::<String>::new());
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
