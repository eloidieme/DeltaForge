use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

fn deltaforge_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_deltaforge"))
}

fn deltaforge_pack_mcp_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_deltaforge-pack-mcp"))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn temp_project_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "deltaforge-it-{name}-{}-{nanos}",
        std::process::id()
    ))
}

fn run_deltaforge<I, S>(args: I, cwd: &Path) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    Command::new(deltaforge_bin())
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap()
}

fn run_deltaforge_with_env<I, S>(args: I, cwd: &Path, envs: &[(&str, &Path)]) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut command = Command::new(deltaforge_bin());
    command.args(args).current_dir(cwd);
    for (key, value) in envs {
        command.env(key, value);
    }
    command.output().unwrap()
}

fn output_text(output: &Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "expected success, got status {:?}\n{}",
        output.status.code(),
        output_text(output)
    );
}

fn assert_failure(output: &Output) {
    assert!(
        !output.status.success(),
        "expected failure, got success\n{}",
        output_text(output)
    );
}

fn assert_stdout_contains(output: &Output, expected: &str) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(expected),
        "expected stdout to contain {expected:?}\n{}",
        output_text(output)
    );
}

fn assert_stdout_not_contains(output: &Output, unexpected: &str) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains(unexpected),
        "expected stdout not to contain {unexpected:?}\n{}",
        output_text(output)
    );
}

fn assert_stderr_contains(output: &Output, expected: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(expected),
        "expected stderr to contain {expected:?}\n{}",
        output_text(output)
    );
}

fn cleanup_kept_temp_dirs(output: &Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(path) = line.strip_prefix("Kept temp dir: ") {
            let _ = fs::remove_dir_all(path);
        }
    }
}

fn run_git<I, S>(args: I, cwd: &Path) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap()
}

fn run_mcp_request(request: serde_json::Value) -> serde_json::Value {
    let body = serde_json::to_vec(&request).unwrap();
    let mut child = Command::new(deltaforge_pack_mcp_bin())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(&body).unwrap();
        stdin.write_all(b"\n").unwrap();
    }
    let output = child.wait_with_output().unwrap();
    assert_success(&output);
    parse_mcp_response(&output.stdout)
}

fn parse_mcp_response(bytes: &[u8]) -> serde_json::Value {
    serde_json::from_slice(bytes).unwrap()
}

fn copy_dir_recursive(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();

    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type().unwrap();

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path);
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path).unwrap();
        }
    }
}

#[test]
fn starter_project_initializes_and_fails_current_stage() {
    let project_dir = temp_project_path("starter-fails");

    let init = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);
    assert_stdout_contains(&init, "Created project.");

    assert!(project_dir.join("Cargo.toml").is_file());
    assert!(project_dir.join("src/main.rs").is_file());
    assert!(project_dir.join(".deltaforge/state.json").is_file());
    assert!(project_dir.join(".deltaforge/config.toml").is_file());
    let project_readme = fs::read_to_string(project_dir.join("README.md")).unwrap();
    assert!(project_readme.contains("## What you are building"));
    assert!(project_readme.contains("## Why this is useful"));
    assert!(project_readme.contains("## Stage Roadmap"));
    assert!(project_readme.contains("deltaforge overview"));

    let config = fs::read_to_string(project_dir.join(".deltaforge/config.toml")).unwrap();
    assert!(config.contains("timeout_ms = 5000"));
    assert!(config.contains("build_timeout_ms = 120000"));
    assert!(config.contains("keep_temp = false"));

    let state = fs::read_to_string(project_dir.join(".deltaforge/state.json")).unwrap();
    assert!(state.contains(r#""project": "flashindex""#));
    assert!(state.contains(r#""current_stage": "01_scan_files""#));
    let state_json: serde_json::Value = serde_json::from_str(&state).unwrap();
    assert_eq!(state_json["schema_version"], 1);
    let created_at = state_json["created_at"].as_str().unwrap();
    assert!(!created_at.starts_with("unix:"));
    assert!(created_at.ends_with('Z'));
    OffsetDateTime::parse(created_at, &Rfc3339).unwrap();

    let instructions = run_deltaforge(["instructions"], &project_dir);
    assert_success(&instructions);
    assert_stdout_contains(&instructions, "Stage 01_scan_files: Scan files");
    assert_stdout_contains(&instructions, "flashindex scan <path>");

    let overview = run_deltaforge(["overview"], &project_dir);
    assert_success(&overview);
    assert_stdout_contains(&overview, "What you are building");
    assert_stdout_contains(&overview, "Why this is useful");
    assert_stdout_contains(&overview, "Stage roadmap:");
    assert_stdout_contains(&overview, "→ 01_scan_files - Scan files");

    let overview_json = run_deltaforge(["overview", "--json"], &project_dir);
    assert_success(&overview_json);
    let parsed_overview: serde_json::Value = serde_json::from_slice(&overview_json.stdout).unwrap();
    assert_eq!(parsed_overview["project"], "flashindex");
    assert_eq!(parsed_overview["roadmap"].as_array().unwrap().len(), 10);

    let status = run_deltaforge(["status"], &project_dir);
    assert_success(&status);
    assert_stdout_contains(&status, "→ 01_scan_files - Scan files");

    let hint = run_deltaforge(["hint"], &project_dir);
    assert_success(&hint);
    assert_stdout_contains(&hint, "Hint 1/3:");

    let next = run_deltaforge(["next"], &project_dir);
    assert_success(&next);
    assert_stdout_contains(&next, "Current stage has not passed yet.");

    let test = run_deltaforge(["test"], &project_dir);
    assert_failure(&test);
    assert_stdout_contains(&test, "0 passed, 5 failed");
    assert_stderr_contains(&test, "error: tests failed");

    let json_test = run_deltaforge(["test", "--json"], &project_dir);
    assert_failure(&json_test);
    assert_stdout_not_contains(&json_test, "Stage 01_scan_files: Scan files");
    assert_stdout_not_contains(&json_test, "✗ scans files in a basic project");
    assert_stderr_contains(&json_test, "error: tests failed");

    let parsed: serde_json::Value =
        serde_json::from_slice(&json_test.stdout).expect("test --json should emit valid JSON");
    assert_eq!(parsed[0]["stage_id"], "01_scan_files");
    assert_eq!(parsed[0]["passed"], 0);
    assert_eq!(parsed[0]["failed"], 5);
    assert_eq!(parsed[0]["results"].as_array().unwrap().len(), 5);

    let config_json = run_deltaforge(["config", "show", "--json"], &project_dir);
    assert_success(&config_json);
    assert!(
        String::from_utf8_lossy(&config_json.stderr).is_empty(),
        "{}",
        output_text(&config_json)
    );
    let parsed_config: serde_json::Value =
        serde_json::from_slice(&config_json.stdout).expect("config show --json is valid JSON");
    assert_eq!(parsed_config["schema_version"], 1);
}

#[test]
fn list_and_validate_pack_are_user_facing() {
    let list = run_deltaforge(["list"], &repo_root());
    assert_success(&list);
    assert_stdout_contains(&list, "Available projects:");
    assert_stdout_contains(&list, "flashindex");
    assert_stdout_contains(&list, "minikv");
    assert_stdout_contains(&list, "tinyhttp");
    assert_stdout_contains(&list, "byteforgevm");
    assert_stdout_contains(&list, "Languages: rust");

    let validate = run_deltaforge(["validate-pack", "flashindex"], &repo_root());
    assert_success(&validate);
    assert_stdout_contains(&validate, "✓ flashindex is valid");

    let validate_json = run_deltaforge(["validate-pack", "flashindex", "--json"], &repo_root());
    assert_success(&validate_json);
    assert!(
        String::from_utf8_lossy(&validate_json.stderr).is_empty(),
        "{}",
        output_text(&validate_json)
    );
    let parsed: serde_json::Value = serde_json::from_slice(&validate_json.stdout).unwrap();
    assert_eq!(parsed[0]["id"], "flashindex");
    assert_eq!(parsed[0]["valid"], true);
}

#[test]
fn v2_pack_commands_doctor_and_json_report_work() {
    let pack_list = run_deltaforge(["pack", "list", "--json"], &repo_root());
    assert_success(&pack_list);
    let packs: serde_json::Value = serde_json::from_slice(&pack_list.stdout).unwrap();
    let ids = packs
        .as_array()
        .unwrap()
        .iter()
        .map(|pack| pack["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(ids.contains(&"flashindex"));
    assert!(ids.contains(&"minikv"));
    assert!(ids.contains(&"tinyhttp"));
    assert!(ids.contains(&"byteforgevm"));

    let show = run_deltaforge(["pack", "show", "minikv", "--json"], &repo_root());
    assert_success(&show);
    let shown: serde_json::Value = serde_json::from_slice(&show.stdout).unwrap();
    assert_eq!(shown["id"], "minikv");

    let install_root = temp_project_path("installed-packs");
    let install = run_deltaforge(
        [
            "pack",
            "install",
            "minikv",
            "--dest",
            install_root.to_str().unwrap(),
        ],
        &repo_root(),
    );
    assert_success(&install);
    assert!(install_root.join("minikv/project.yaml").is_file());

    let validate_installed = run_deltaforge(
        [
            "--packs-dir",
            install_root.to_str().unwrap(),
            "validate-pack",
            "minikv",
        ],
        &repo_root(),
    );
    assert_success(&validate_installed);

    let doctor = run_deltaforge(["doctor", "--json"], &repo_root());
    assert_success(&doctor);
    let doctor_json: serde_json::Value = serde_json::from_slice(&doctor.stdout).unwrap();
    assert!(doctor_json["pack_count"].as_u64().unwrap() >= 4);

    let project_dir = temp_project_path("json-report");
    let init = run_deltaforge(
        [
            "init",
            "minikv",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);

    let report = run_deltaforge(
        ["report", "--format", "json", "--output", "report.json"],
        &project_dir,
    );
    assert_success(&report);
    let report_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(project_dir.join("report.json")).unwrap())
            .unwrap();
    assert_eq!(report_json["project"], "minikv");
}

#[test]
fn pack_authoring_cli_scaffolds_and_diagnoses_packs() {
    let packs_root = temp_project_path("authoring-packs");
    let new_pack = run_deltaforge(
        [
            "pack",
            "new",
            "samplepack",
            "--name",
            "Sample Pack",
            "--description",
            "Sample generated pack",
            "--dest",
            packs_root.to_str().unwrap(),
            "--json",
        ],
        &repo_root(),
    );
    assert_success(&new_pack);
    let report: serde_json::Value = serde_json::from_slice(&new_pack.stdout).unwrap();
    assert_eq!(report["status"], "ok");
    assert!(packs_root.join("samplepack/project.yaml").is_file());

    let add_stage = run_deltaforge(
        [
            "pack",
            "add-stage",
            "--pack-dir",
            packs_root.join("samplepack").to_str().unwrap(),
            "02_second_stage",
            "--title",
            "Second stage",
            "--json",
        ],
        &repo_root(),
    );
    assert_success(&add_stage);
    let add_report: serde_json::Value = serde_json::from_slice(&add_stage.stdout).unwrap();
    assert_eq!(add_report["status"], "ok");
    assert!(
        packs_root
            .join("samplepack/stages/02_second_stage/tests.yaml")
            .is_file()
    );

    let validate = run_deltaforge(
        [
            "--packs-dir",
            packs_root.to_str().unwrap(),
            "validate-pack",
            "samplepack",
        ],
        &repo_root(),
    );
    assert_success(&validate);

    let doctor = run_deltaforge(
        [
            "--packs-dir",
            packs_root.to_str().unwrap(),
            "pack",
            "doctor",
            "samplepack",
            "--json",
        ],
        &repo_root(),
    );
    assert_success(&doctor);
    let doctor_report: serde_json::Value = serde_json::from_slice(&doctor.stdout).unwrap();
    assert_eq!(doctor_report["pack"], "samplepack");
}

#[test]
fn pack_mcp_lists_tools_and_creates_pack_with_structured_report() {
    let tools = run_mcp_request(serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list"
    }));
    let tool_names = tools["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(tool_names.contains(&"create_pack"));
    assert!(tool_names.contains(&"check_reference"));
    assert!(tool_names.contains(&"replace_stage_tests"));
    assert!(tool_names.contains(&"write_fixture_file"));
    let inspect = tools["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "inspect_packs")
        .unwrap();
    assert_eq!(inspect["annotations"]["readOnlyHint"], true);
    let replace_tests = tools["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "replace_stage_tests")
        .unwrap();
    assert_eq!(replace_tests["annotations"]["readOnlyHint"], false);
    assert_eq!(replace_tests["annotations"]["destructiveHint"], true);
    assert_eq!(replace_tests["annotations"]["idempotentHint"], true);
    assert_eq!(replace_tests["inputSchema"]["additionalProperties"], false);
    let create_tool = tools["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "create_pack")
        .unwrap();
    assert_eq!(create_tool["annotations"]["destructiveHint"], true);

    let packs_root = temp_project_path("mcp-packs");
    let create = run_mcp_request(serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "create_pack",
            "arguments": {
                "id": "mcppack",
                "name": "MCP Pack",
                "description": "Generated through MCP",
                "dest": packs_root.to_str().unwrap()
            }
        }
    }));
    assert_eq!(create["result"]["isError"], false);
    let text = create["result"]["content"][0]["text"].as_str().unwrap();
    let report: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(report["status"], "ok");
    assert!(packs_root.join("mcppack/project.yaml").is_file());
}

#[test]
fn explain_failure_uses_last_failed_test_details() {
    let project_dir = temp_project_path("explain-failure");
    let init = run_deltaforge(
        [
            "init",
            "minikv",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);

    let test = run_deltaforge(["test"], &project_dir);
    assert_failure(&test);

    let explain = run_deltaforge(["explain-failure"], &project_dir);
    assert_success(&explain);
    assert_stdout_contains(&explain, "Stage 01_memory_commands");
    assert_stdout_contains(&explain, "Failed:");
    assert_stdout_contains(&explain, "Suggested next steps:");

    let explain_json = run_deltaforge(["explain-failure", "--json"], &project_dir);
    assert_success(&explain_json);
    let parsed: serde_json::Value = serde_json::from_slice(&explain_json.stdout).unwrap();
    assert_eq!(parsed["stage_id"], "01_memory_commands");
    assert!(parsed["failed"].as_u64().unwrap() > 0);
}

#[test]
fn validate_pack_reports_invalid_external_pack() {
    let packs_root = temp_project_path("invalid-pack-root");
    let external_pack = packs_root.join("flashindex");
    copy_dir_recursive(&repo_root().join("packs/flashindex"), &external_pack);

    let tests_path = external_pack.join("stages/01_scan_files/tests.yaml");
    let tests = fs::read_to_string(&tests_path).unwrap();
    fs::write(
        &tests_path,
        tests.replace("fixture: basic_project", "fixture: missing_fixture"),
    )
    .unwrap();

    let validate = run_deltaforge_with_env(
        ["validate-pack", "flashindex"],
        &repo_root(),
        &[("DELTAFORGE_PACKS_DIR", &packs_root)],
    );

    assert_failure(&validate);
    assert_stdout_contains(&validate, "✗ flashindex is invalid");
    assert_stdout_contains(&validate, "references missing fixture");
    assert_stderr_contains(&validate, "error: pack validation failed");
}

#[cfg(unix)]
#[test]
fn validate_pack_reports_symlinks_in_pack_content() {
    let packs_root = temp_project_path("symlink-pack-root");
    let external_pack = packs_root.join("flashindex");
    copy_dir_recursive(&repo_root().join("packs/flashindex"), &external_pack);

    // Two offenders in one pack: validation must list them both in one pass,
    // not stop at the first like the digest does.
    std::os::unix::fs::symlink(
        external_pack.join("README.md"),
        external_pack.join("stages/01_scan_files/notes-link.md"),
    )
    .unwrap();
    std::os::unix::fs::symlink(
        external_pack.join("stages/01_scan_files/fixtures/basic_project"),
        external_pack.join("stages/02_filter_files/fixtures/linked_project"),
    )
    .unwrap();

    let validate = run_deltaforge_with_env(
        ["validate-pack", "flashindex"],
        &repo_root(),
        &[("DELTAFORGE_PACKS_DIR", &packs_root)],
    );

    assert_failure(&validate);
    assert_stdout_contains(&validate, "✗ flashindex is invalid");
    assert_stdout_contains(&validate, "notes-link.md is a symbolic link");
    assert_stdout_contains(&validate, "linked_project is a symbolic link");

    let _ = fs::remove_dir_all(packs_root);
}

#[test]
fn validate_pack_reports_invalid_benchmark_fixture() {
    let packs_root = temp_project_path("invalid-benchmark-pack-root");
    let external_pack = packs_root.join("flashindex");
    copy_dir_recursive(&repo_root().join("packs/flashindex"), &external_pack);

    fs::write(
        external_pack.join("stages/01_scan_files/benchmarks.yaml"),
        r#"
benchmarks:
  - name: broken
    fixture: missing_fixture
    command: ["scan", "{fixture_path}"]
"#,
    )
    .unwrap();

    let validate = run_deltaforge_with_env(
        ["validate-pack", "flashindex", "--json"],
        &repo_root(),
        &[("DELTAFORGE_PACKS_DIR", &packs_root)],
    );

    assert_failure(&validate);
    let parsed: serde_json::Value = serde_json::from_slice(&validate.stdout).unwrap();
    assert_eq!(parsed[0]["valid"], false);
    assert!(
        parsed[0]["problems"]
            .as_array()
            .unwrap()
            .iter()
            .any(|problem| problem.as_str().unwrap().contains("missing fixture"))
    );
}

#[test]
fn project_commands_explain_when_run_outside_deltaforge_project() {
    let outside_dir = temp_project_path("outside-project");
    fs::create_dir_all(&outside_dir).unwrap();

    let status = run_deltaforge(["status"], &outside_dir);

    assert_failure(&status);
    assert_stderr_contains(&status, "not inside a DeltaForge project");
    assert_stderr_contains(&status, ".deltaforge/state.json");
    assert_stderr_contains(&status, "deltaforge init <project> --lang <language>");
}

#[test]
fn project_commands_work_from_nested_dirs_and_project_dir_flag() {
    let project_dir = temp_project_path("nested-discovery");
    let init = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);

    let nested_dir = project_dir.join("src/nested/deeper");
    fs::create_dir_all(&nested_dir).unwrap();

    let nested_status = run_deltaforge(["status"], &nested_dir);
    assert_success(&nested_status);
    assert_stdout_contains(&nested_status, "Project: FlashIndex");
    assert_stdout_contains(&nested_status, "Current stage: 01_scan_files");

    let outside_dir = temp_project_path("project-dir-outside");
    fs::create_dir_all(&outside_dir).unwrap();
    let status_with_project_dir = run_deltaforge(
        ["--project-dir", project_dir.to_str().unwrap(), "status"],
        &outside_dir,
    );
    assert_success(&status_with_project_dir);
    assert_stdout_contains(&status_with_project_dir, "Project: FlashIndex");
}

#[test]
fn cli_packs_dir_overrides_env_and_dev_fallback() {
    let env_packs_root = temp_project_path("env-packs");
    let cli_packs_root = temp_project_path("cli-packs");
    let env_pack = env_packs_root.join("flashindex");
    let cli_pack = cli_packs_root.join("flashindex");
    copy_dir_recursive(&repo_root().join("packs/flashindex"), &env_pack);
    copy_dir_recursive(&repo_root().join("packs/flashindex"), &cli_pack);

    let env_manifest_path = env_pack.join("project.yaml");
    let env_manifest = fs::read_to_string(&env_manifest_path).unwrap();
    fs::write(
        &env_manifest_path,
        env_manifest.replace(
            "description: Local source-code search engine",
            "description: Env Pack",
        ),
    )
    .unwrap();

    let cli_manifest_path = cli_pack.join("project.yaml");
    let cli_manifest = fs::read_to_string(&cli_manifest_path).unwrap();
    fs::write(
        &cli_manifest_path,
        cli_manifest.replace(
            "description: Local source-code search engine",
            "description: CLI Pack",
        ),
    )
    .unwrap();

    let list = run_deltaforge_with_env(
        ["--packs-dir", cli_packs_root.to_str().unwrap(), "list"],
        &repo_root(),
        &[("DELTAFORGE_PACKS_DIR", &env_packs_root)],
    );

    assert_success(&list);
    assert_stdout_contains(&list, "CLI Pack");
    assert_stdout_not_contains(&list, "Env Pack");
}

#[test]
fn test_runner_selection_flags_are_user_facing() {
    let project_dir = temp_project_path("runner-flags");
    let init = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);

    let list = run_deltaforge(["test", "--list-tests", "--json"], &project_dir);
    assert_success(&list);
    let listed: serde_json::Value =
        serde_json::from_slice(&list.stdout).expect("test --list-tests --json is valid JSON");
    assert_eq!(listed[0]["results"].as_array().unwrap().len(), 5);
    assert_stdout_contains(&list, "scans files in a basic project");

    let filtered = run_deltaforge(["test", "--filter", "nested"], &project_dir);
    assert_failure(&filtered);
    assert_stdout_contains(&filtered, "0 passed, 1 failed");
    assert_stdout_contains(&filtered, "scans nested directories");
    assert_stdout_not_contains(&filtered, "skips ignored directories");

    let fail_fast = run_deltaforge(["test", "--fail-fast"], &project_dir);
    assert_failure(&fail_fast);
    assert_stdout_contains(&fail_fast, "0 passed, 1 failed");
    assert_stdout_not_contains(&fail_fast, "scans nested directories");
}

#[test]
fn bench_report_and_portfolio_generate_project_artifacts() {
    let project_dir = temp_project_path("bench-report");
    let init = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);

    let bench = run_deltaforge(
        [
            "bench",
            "--iterations",
            "1",
            "--warmup",
            "0",
            "--save",
            "--json",
        ],
        &project_dir,
    );
    assert_success(&bench);
    let bench_json: serde_json::Value =
        serde_json::from_slice(&bench.stdout).expect("bench --json is valid JSON");
    assert_eq!(bench_json[0]["benchmark"], "scan_basic_project");
    assert_eq!(bench_json[0]["points"][0]["success"], true);
    #[cfg(any(target_os = "linux", target_os = "macos", windows))]
    assert!(
        bench_json[0]["points"][0]["peak_memory_mb"].is_number(),
        "expected peak_memory_mb to be measured on this OS, got: {}",
        bench_json[0]["points"][0]
    );
    let history_path = project_dir.join(".deltaforge/benchmark_history.json");
    let history: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&history_path).unwrap())
            .expect("benchmark history is valid JSON");
    assert_eq!(history["schema_version"], 2);
    assert_eq!(history["runs"][0]["benchmark"], "scan_basic_project");
    assert!(
        history["runs"][0]["points"][0]["params"]
            .as_object()
            .expect("points carry a params object")
            .is_empty()
    );

    let comparison = run_deltaforge(
        ["bench", "--iterations", "1", "--warmup", "0", "--compare"],
        &project_dir,
    );
    assert_success(&comparison);
    assert_stdout_contains(&comparison, "Comparison with prior saved run:");
    assert_stdout_contains(&comparison, "median:");
    assert_stdout_contains(&comparison, "throughput:");
    assert_stdout_contains(&comparison, "peak memory:");

    let markdown_report = run_deltaforge(
        ["report", "--format", "markdown", "--output", "report.md"],
        &project_dir,
    );
    assert_success(&markdown_report);
    let report = fs::read_to_string(project_dir.join("report.md")).unwrap();
    assert!(report.contains("## Project Metadata"));
    assert!(report.contains("## Benchmark History"));

    let html_report = run_deltaforge(
        ["report", "--format", "html", "--output", "report.html"],
        &project_dir,
    );
    assert_success(&html_report);
    let html = fs::read_to_string(project_dir.join("report.html")).unwrap();
    assert!(html.contains("<!doctype html>"));

    let portfolio = run_deltaforge(["portfolio", "--output", "PORTFOLIO.md"], &project_dir);
    assert_success(&portfolio);
    let portfolio_text = fs::read_to_string(project_dir.join("PORTFOLIO.md")).unwrap();
    assert!(portfolio_text.contains("## Project Summary"));
    assert!(portfolio_text.contains("scan_basic_project"));
}

#[test]
fn bench_matrix_prints_table_speedup_and_saves_per_point_history() {
    let root = temp_project_path("bench-matrix");
    let packs = root.join("packs");
    let external_pack = packs.join("flashindex");
    copy_dir_recursive(&repo_root().join("packs/flashindex"), &external_pack);
    let benchmarks_path = external_pack.join("stages/01_scan_files/benchmarks.yaml");
    let mut benchmarks = fs::read_to_string(&benchmarks_path).unwrap();
    benchmarks.push_str("    matrix:\n      threads: [1, 2]\n");
    fs::write(&benchmarks_path, benchmarks).unwrap();

    let project = root.join("project");
    let init = run_deltaforge_with_env(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
        &[("DELTAFORGE_PACKS_DIR", &packs)],
    );
    assert_success(&init);

    let bench = run_deltaforge_with_env(
        ["bench", "--iterations", "1", "--warmup", "0", "--save"],
        &project,
        &[("DELTAFORGE_PACKS_DIR", &packs)],
    );
    assert_success(&bench);
    assert_stdout_contains(&bench, "params");
    assert_stdout_contains(&bench, "median");
    assert_stdout_contains(&bench, "threads=1");
    assert_stdout_contains(&bench, "threads=2");
    assert_stdout_contains(&bench, "speedup_1_to_2:");

    let history: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(project.join(".deltaforge/benchmark_history.json")).unwrap(),
    )
    .unwrap();
    let points = history["runs"][0]["points"].as_array().unwrap();
    assert_eq!(points.len(), 2);
    assert_eq!(points[0]["params"]["threads"], "1");
    assert_eq!(points[1]["params"]["threads"], "2");
    assert!(history["runs"][0]["derived"].is_null());

    let comparison = run_deltaforge_with_env(
        ["bench", "--iterations", "1", "--warmup", "0", "--compare"],
        &project,
        &[("DELTAFORGE_PACKS_DIR", &packs)],
    );
    assert_success(&comparison);
    assert_stdout_contains(&comparison, "Comparison with prior saved run:");
    assert_stdout_contains(&comparison, "[threads=1] prior:");
    assert_stdout_contains(&comparison, "[threads=2] prior:");
    assert_stdout_contains(&comparison, "median:");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn parallel_indexing_stage_benchmark_matrix_evaluates_gate_per_thread_count() {
    // The stage 09 benchmark declares a threads:[1,2,4,8] matrix and a speedup
    // gate. This exercises the machinery end to end against the reference
    // solution: every thread point is measured and saved, and the gate is
    // evaluated and reported. We deliberately do NOT assert the speedup value —
    // CI runners are noisy and the point is that the gate ran, not that a
    // particular ratio was hit.
    let project_dir = temp_project_path("parallel-bench");
    let init = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);
    fs::copy(
        repo_root().join("tools/reference_solutions/flashindex_rust/src/main.rs"),
        project_dir.join("src/main.rs"),
    )
    .unwrap();

    let bench = run_deltaforge(
        [
            "bench",
            "--stage",
            "09_parallel_indexing",
            "--iterations",
            "2",
            "--warmup",
            "0",
            "--save",
            "--json",
        ],
        &project_dir,
    );
    assert_success(&bench);
    let json: serde_json::Value = serde_json::from_slice(&bench.stdout).unwrap();
    let record = &json[0];
    assert_eq!(record["benchmark"], "index_with_threads");

    let points = record["points"].as_array().unwrap();
    assert_eq!(points.len(), 4);
    let thread_values: Vec<&str> = points
        .iter()
        .map(|point| point["params"]["threads"].as_str().unwrap())
        .collect();
    assert_eq!(thread_values, ["1", "2", "4", "8"]);
    assert!(points.iter().all(|point| point["success"] == true));

    // The gate was evaluated and reported for the speedup metric.
    let gate = &record["gate_results"][0];
    assert_eq!(gate["metric"], "speedup");
    assert_eq!(gate["name"], "parallel speedup");
    assert!(gate["passed"].is_boolean());
    assert!(record["performance"].is_string());

    // Per-thread points are persisted to versioned history.
    let history: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(project_dir.join(".deltaforge/benchmark_history.json")).unwrap(),
    )
    .unwrap();
    let saved_points = history["runs"][0]["points"].as_array().unwrap();
    assert_eq!(saved_points.len(), 4);

    let _ = fs::remove_dir_all(project_dir);
}

#[test]
fn performance_gate_requires_current_complete_benchmark_and_stales_with_code() {
    let (root, packs, project) = init_project_from_pack_copy("performance-gate");
    let benchmarks = packs.join("flashindex/stages/01_scan_files/benchmarks.yaml");
    let mut source = fs::read_to_string(&benchmarks).unwrap();
    source.push_str(
        r#"
performance_gates:
  - name: quick scan
    benchmark: scan_basic_project
    metric: runtime_median_ms
    max: 1000000000
    advice: ["avoid needless work"]
"#,
    );
    fs::write(&benchmarks, source).unwrap();

    // Re-pin the changed external pack, then prove correctness under its new
    // behavioral digest. The gate has no historical fallback.
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    ));
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "test"],
        &project,
    ));
    let blocked = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "next"], &project);
    assert_failure(&blocked);
    assert_stderr_contains(&blocked, "Run: deltaforge bench");

    let bench = run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "bench",
            "--iterations",
            "1",
            "--warmup",
            "0",
            "--json",
        ],
        &project,
    );
    assert_success(&bench);
    let json: serde_json::Value = serde_json::from_slice(&bench.stdout).unwrap();
    assert_eq!(json[0]["performance"], "passed");
    assert!(json[0]["gate_results"][0]["passed"].as_bool().unwrap());

    let next = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "next"], &project);
    assert_success(&next);
    fs::write(project.join("src/main.rs"), "fn main() {}\n").unwrap();
    let status = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "status", "--json"],
        &project,
    );
    assert_success(&status);
    let status: serde_json::Value = serde_json::from_slice(&status.stdout).unwrap();
    assert_eq!(status["stages"][0]["performance"], "not_measured");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn failing_performance_gate_blocks_with_advice_and_enforcement_can_be_skipped() {
    let (root, packs, project) = init_project_from_pack_copy("performance-gate-failure");
    append_scan_gate(&packs, "runtime limit", "0", "reduce avoidable work");
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    ));
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "test"],
        &project,
    ));

    let bench = run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "bench",
            "--iterations",
            "1",
            "--warmup",
            "0",
        ],
        &project,
    );
    assert_success(&bench);
    assert_stdout_contains(&bench, "Correctness: passed");
    assert_stdout_contains(&bench, "Performance: not yet");
    assert_stdout_contains(&bench, "Gate: runtime limit");
    assert_stdout_contains(&bench, "Likely areas to investigate:");
    assert_stdout_contains(&bench, "reduce avoidable work");

    let status = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "status", "--json"],
        &project,
    );
    assert_success(&status);
    let status: serde_json::Value = serde_json::from_slice(&status.stdout).unwrap();
    assert_eq!(status["stages"][0]["performance"], "not_yet");

    let blocked = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "next"], &project);
    assert_failure(&blocked);
    assert_stderr_contains(&blocked, "performance gates are not passing");
    assert_stderr_contains(&blocked, "Run: deltaforge bench");

    fs::write(
        project.join(".deltaforge/config.toml"),
        "[gates]\nenforce = false\n",
    )
    .unwrap();
    let skipped = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "next"], &project);
    assert_success(&skipped);
    assert_stdout_contains(&skipped, "performance gates skipped: gates.enforce = false");
    assert_stdout_contains(&skipped, "Unlocked Stage 02_filter_files");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tightening_a_gate_requires_correctness_and_performance_revalidation() {
    let (root, packs, project) = init_project_from_pack_copy("performance-gate-tightening");
    append_scan_gate(&packs, "runtime limit", "1000000000", "inspect runtime");
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    ));
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "test"],
        &project,
    ));
    assert_success(&run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "bench",
            "--iterations",
            "1",
            "--warmup",
            "0",
        ],
        &project,
    ));

    let path = packs.join("flashindex/stages/01_scan_files/benchmarks.yaml");
    let source = fs::read_to_string(&path).unwrap();
    fs::write(&path, source.replace("max: 1000000000", "max: 0")).unwrap();
    let sync = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    );
    assert_success(&sync);
    assert_stdout_contains(&sync, "! 01_scan_files (needs revalidation)");

    let next = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "next"], &project);
    assert_failure(&next);
    assert_stderr_contains(&next, "must be revalidated");
    assert_stderr_contains(&next, "deltaforge test");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn gate_advice_methodology_yaml_format_and_display_name_do_not_stale_proofs() {
    let (root, packs, project) = init_project_from_pack_copy("performance-gate-canonical");
    append_scan_gate(&packs, "quick scan", "1000000000", "first advice");
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    ));
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "test"],
        &project,
    ));
    assert_success(&run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "bench",
            "--iterations",
            "1",
            "--warmup",
            "0",
        ],
        &project,
    ));

    let path = packs.join("flashindex/stages/01_scan_files/benchmarks.yaml");
    let source = fs::read_to_string(&path).unwrap();
    fs::write(&path, source.replace("first advice", "updated advice")).unwrap();
    let advice_sync = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    );
    assert_success(&advice_sync);
    assert_stdout_not_contains(&advice_sync, "needs revalidation");

    fs::write(
        &path,
        r#"# formatting and mapping order are not semantics
performance_gates:
  - advice:
      - updated advice
    max: 1000000000
    metric: runtime_median_ms
    benchmark: scan_basic_project
    name: quick scan
benchmarks:
  - timeout_ms: 2000
    warmup: 0
    iterations: 9
    command:
      - scan
      - "{fixture_path}"
    fixture: basic_project
    name: scan_basic_project
"#,
    )
    .unwrap();
    let format_sync = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    );
    assert_success(&format_sync);
    assert_stdout_not_contains(&format_sync, "needs revalidation");

    let source = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        source.replace("name: quick scan", "name: renamed scan"),
    )
    .unwrap();
    let rename_sync = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    );
    assert_success(&rename_sync);
    assert_stdout_not_contains(&rename_sync, "needs revalidation");
    let status = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "status", "--json"],
        &project,
    );
    assert_success(&status);
    let status: serde_json::Value = serde_json::from_slice(&status.stdout).unwrap();
    assert_eq!(status["stages"][0]["performance"], "passed");
    let next = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "next"], &project);
    assert_success(&next);

    let _ = fs::remove_dir_all(root);
}

#[test]
fn timed_out_benchmark_reports_no_pass_and_preserves_prior_complete_record() {
    let (root, packs, project) = init_project_from_pack_copy("performance-gate-timeout");
    append_scan_gate(&packs, "quick scan", "1000000000", "inspect runtime");
    let source = fs::read_to_string(project.join("src/main.rs"))
        .unwrap()
        .replace(
            "fn main() -> ExitCode {",
            "fn main() -> ExitCode {\n    std::thread::sleep(std::time::Duration::from_millis(500));",
        );
    fs::write(project.join("src/main.rs"), source).unwrap();
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    ));
    assert_success(&run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "test"],
        &project,
    ));
    assert_success(&run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "bench",
            "--iterations",
            "1",
            "--warmup",
            "0",
        ],
        &project,
    ));

    let benchmarks = packs.join("flashindex/stages/01_scan_files/benchmarks.yaml");
    let source = fs::read_to_string(&benchmarks).unwrap();
    fs::write(
        &benchmarks,
        source.replace("timeout_ms: 5000", "timeout_ms: 25"),
    )
    .unwrap();
    let sync = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    );
    assert_success(&sync);
    assert_stdout_not_contains(&sync, "needs revalidation");
    let state_path = project.join(".deltaforge/state.json");
    let before: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&state_path).unwrap()).unwrap();

    let failed = run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "bench",
            "--iterations",
            "1",
            "--warmup",
            "0",
            "--json",
        ],
        &project,
    );
    assert_failure(&failed);
    let report: serde_json::Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert_eq!(report[0]["performance"], "not_measured");
    assert_eq!(report[0]["gate_results"][0]["passed"], false);
    let after: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&state_path).unwrap()).unwrap();
    assert_eq!(before["gate_results"], after["gate_results"]);

    let status = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "status", "--json"],
        &project,
    );
    assert_success(&status);
    let status: serde_json::Value = serde_json::from_slice(&status.stdout).unwrap();
    assert_eq!(status["stages"][0]["performance"], "passed");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn validate_pack_rejects_missing_performance_gate_benchmark() {
    let root = temp_project_path("invalid-performance-gate");
    let packs = root.join("packs");
    let pack = packs.join("flashindex");
    copy_dir_recursive(&repo_root().join("packs/flashindex"), &pack);
    let path = pack.join("stages/01_scan_files/benchmarks.yaml");
    let mut source = fs::read_to_string(&path).unwrap();
    source.push_str(
        "\nperformance_gates:\n  - name: missing\n    benchmark: absent\n    metric: throughput_mb_s\n    min: 1\n",
    );
    fs::write(path, source).unwrap();
    let result = run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "validate-pack",
            "--strict",
        ],
        &repo_root(),
    );
    assert_failure(&result);
    assert_stdout_contains(&result, "references missing or ambiguous benchmark absent");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn legacy_benchmark_history_is_converted_on_save() {
    let project_dir = temp_project_path("bench-legacy-history");
    let init = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);
    fs::copy(
        repo_root().join("tests/fixtures/legacy_benchmark_history.json"),
        project_dir.join(".deltaforge/benchmark_history.json"),
    )
    .unwrap();

    let bench = run_deltaforge(
        [
            "bench",
            "--iterations",
            "1",
            "--warmup",
            "0",
            "--save",
            "--json",
        ],
        &project_dir,
    );
    assert_success(&bench);

    let history: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(project_dir.join(".deltaforge/benchmark_history.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(history["schema_version"], 2);
    let runs = history["runs"].as_array().unwrap();
    assert_eq!(runs.len(), 3);
    assert!(
        runs[0]["points"][0]["params"]
            .as_object()
            .unwrap()
            .is_empty()
    );
    assert_eq!(runs[0]["points"][0]["runtime_median_ms"], 12.5);
    assert_eq!(runs[1]["points"][0]["success"], false);
    assert_eq!(runs[2]["points"][0]["success"], true);
    let _ = fs::remove_dir_all(project_dir);
}

#[test]
fn design_command_renders_prompt_and_note_path() {
    let project_dir = temp_project_path("design");
    let init = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);

    let design = run_deltaforge(["design"], &project_dir);
    assert_success(&design);
    assert_stdout_contains(&design, "Design notes:");
    assert_stdout_contains(&design, ".deltaforge");
    assert_stdout_contains(&design, "Explain how your scanner walks directories");
}

#[test]
fn commit_requires_git_and_passed_stage_unless_forced() {
    let no_git_project = temp_project_path("commit-no-git");
    let init_no_git = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            no_git_project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init_no_git);
    let no_git_commit = run_deltaforge(["commit", "--force"], &no_git_project);
    assert_failure(&no_git_commit);
    assert_stderr_contains(&no_git_commit, "not a git repository");

    let git_project = temp_project_path("commit-git");
    let init_git = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            git_project.to_str().unwrap(),
        ],
        &repo_root(),
    );
    assert_success(&init_git);
    assert_success(&run_git(
        ["config", "user.email", "deltaforge@example.com"],
        &git_project,
    ));
    assert_success(&run_git(
        ["config", "user.name", "DeltaForge Tests"],
        &git_project,
    ));

    let refused = run_deltaforge(["commit"], &git_project);
    assert_failure(&refused);
    assert_stderr_contains(&refused, "has not passed");

    let forced = run_deltaforge(["commit", "--force"], &git_project);
    assert_success(&forced);
    assert_stdout_contains(&forced, "Complete Stage 01: Scan files");
}

#[test]
fn pack_lookup_uses_env_override_before_dev_fallback() {
    let packs_root = temp_project_path("external-packs");
    let external_pack = packs_root.join("flashindex");
    copy_dir_recursive(&repo_root().join("packs/flashindex"), &external_pack);

    let manifest_path = external_pack.join("project.yaml");
    let manifest = fs::read_to_string(&manifest_path).unwrap();
    fs::write(
        &manifest_path,
        manifest.replace(
            "description: Local source-code search engine",
            "description: External FlashIndex Pack",
        ),
    )
    .unwrap();

    let project_dir = temp_project_path("external-pack-init");
    let init = run_deltaforge_with_env(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
        &[("DELTAFORGE_PACKS_DIR", &packs_root)],
    );

    assert_success(&init);
    let readme = fs::read_to_string(project_dir.join("README.md")).unwrap();
    assert!(readme.contains("External FlashIndex Pack"));
}

#[test]
fn missing_pack_error_lists_external_and_dev_search_paths() {
    let packs_root = temp_project_path("empty-packs");
    fs::create_dir_all(&packs_root).unwrap();
    let project_dir = temp_project_path("missing-pack-target");

    let init = run_deltaforge_with_env(
        [
            "init",
            "missingpack",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
        &[("DELTAFORGE_PACKS_DIR", &packs_root)],
    );

    assert_failure(&init);
    assert_stderr_contains(&init, "could not find project pack missingpack");
    assert_stderr_contains(
        &init,
        &packs_root
            .join("missingpack")
            .join("project.yaml")
            .display()
            .to_string(),
    );
    assert_stderr_contains(
        &init,
        &repo_root()
            .join("packs")
            .join("missingpack")
            .join("project.yaml")
            .display()
            .to_string(),
    );
}

#[test]
fn runner_config_controls_keep_temp_and_timeout() {
    let keep_temp_project = temp_project_path("config-keep-temp");
    let init_keep_temp = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            keep_temp_project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init_keep_temp);

    fs::write(
        keep_temp_project.join(".deltaforge/config.toml"),
        r#"
[runner]
timeout_ms = 5000
build_timeout_ms = 120000
keep_temp = true
"#,
    )
    .unwrap();

    let keep_temp = run_deltaforge(["test", "--verbose"], &keep_temp_project);
    assert_failure(&keep_temp);
    assert_stdout_contains(&keep_temp, "Kept temp dir:");
    cleanup_kept_temp_dirs(&keep_temp);

    let timeout_project = temp_project_path("config-timeout");
    let init_timeout = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            timeout_project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init_timeout);

    fs::write(
        timeout_project.join(".deltaforge/config.toml"),
        r#"
[runner]
timeout_ms = 1
build_timeout_ms = 120000
keep_temp = false
"#,
    )
    .unwrap();

    let timeout = run_deltaforge(["test"], &timeout_project);
    assert_failure(&timeout);
    assert_stdout_contains(&timeout, "command timed out after 1 ms");
    assert_stdout_contains(&timeout, "0 passed, 5 failed");
    assert_stderr_contains(&timeout, "error: tests failed");
}

#[test]
fn learner_can_pass_all_mvp_stages_and_unlock_progress() {
    let project_dir = temp_project_path("passes-all");

    let init = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);

    fs::write(project_dir.join("src/main.rs"), passing_flashindex_source()).unwrap();

    let stage1 = run_deltaforge(["test"], &project_dir);
    assert_success(&stage1);
    assert_stdout_contains(&stage1, "Stage 01_scan_files: Scan files");
    assert_stdout_contains(&stage1, "5 passed");

    let next1 = run_deltaforge(["next"], &project_dir);
    assert_success(&next1);
    assert_stdout_contains(
        &next1,
        "Unlocked Stage 02_filter_files: Filter source files",
    );

    let stage2 = run_deltaforge(["test"], &project_dir);
    assert_success(&stage2);
    assert_stdout_contains(&stage2, "Stage 02_filter_files: Filter source files");
    assert_stdout_contains(&stage2, "4 passed");

    let next2 = run_deltaforge(["next"], &project_dir);
    assert_success(&next2);
    assert_stdout_contains(&next2, "Unlocked Stage 03_tokenize: Tokenize files");

    let stage3 = run_deltaforge(["test"], &project_dir);
    assert_success(&stage3);
    assert_stdout_contains(&stage3, "Stage 03_tokenize: Tokenize files");
    assert_stdout_contains(&stage3, "5 passed");

    let next3 = run_deltaforge(["next"], &project_dir);
    assert_success(&next3);
    assert_stdout_contains(&next3, "Unlocked Stage 04_exact_search: Exact token search");

    let status = run_deltaforge(["status"], &project_dir);
    assert_success(&status);
    assert_stdout_contains(&status, "✓ 01_scan_files - Scan files");
    assert_stdout_contains(&status, "✓ 02_filter_files - Filter source files");
    assert_stdout_contains(&status, "✓ 03_tokenize - Tokenize files");

    let all = run_deltaforge(["test", "--all"], &project_dir);
    assert_failure(&all);
    assert_stdout_contains(&all, "Stage 01_scan_files: Scan files");
    assert_stdout_contains(&all, "Stage 02_filter_files: Filter source files");
    assert_stdout_contains(&all, "Stage 03_tokenize: Tokenize files");

    let state = fs::read_to_string(project_dir.join(".deltaforge/state.json")).unwrap();
    assert!(state.contains(r#""01_scan_files""#));
    assert!(state.contains(r#""02_filter_files""#));
    assert!(state.contains(r#""03_tokenize""#));
}

#[test]
fn reference_solution_passes_all_flashindex_v1_stages() {
    assert_reference_solution_passes(
        "flashindex",
        "tools/reference_solutions/flashindex_rust/src/main.rs",
        "Stage 10_ranked_search: Ranked search",
    );
}

#[test]
fn reference_solutions_pass_all_deepened_v2_packs() {
    assert_reference_solution_passes(
        "minikv",
        "tools/reference_solutions/minikv_rust/src/main.rs",
        "Stage 06_log_statistics: Log statistics",
    );
    assert_reference_solution_passes(
        "tinyhttp",
        "tools/reference_solutions/tinyhttp_rust/src/main.rs",
        "Stage 06_range_requests: Range requests",
    );
    assert_reference_solution_passes(
        "byteforgevm",
        "tools/reference_solutions/byteforgevm_rust/src/main.rs",
        "Stage 06_trace_mode: Trace mode",
    );
}

#[test]
fn filtered_tests_never_complete_a_stage_and_changed_code_blocks_progression() {
    let project_dir = temp_project_path("completion-integrity");
    let init = run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);
    fs::copy(
        repo_root().join("tools/reference_solutions/flashindex_rust/src/main.rs"),
        project_dir.join("src/main.rs"),
    )
    .unwrap();

    let filtered = run_deltaforge(["test", "--filter", "basic project"], &project_dir);
    assert_success(&filtered);
    let state: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(project_dir.join(".deltaforge/state.json")).unwrap(),
    )
    .unwrap();
    assert!(state["completed_stages"].as_array().unwrap().is_empty());

    let no_matches = run_deltaforge(["test", "--filter", "does-not-exist"], &project_dir);
    assert_failure(&no_matches);
    assert_stderr_contains(&no_matches, "no tests matched");

    let full = run_deltaforge(["test"], &project_dir);
    assert_success(&full);
    let mut source = fs::read_to_string(project_dir.join("src/main.rs")).unwrap();
    source.push_str("\n// changed after passing\n");
    fs::write(project_dir.join("src/main.rs"), source).unwrap();
    let next = run_deltaforge(["next"], &project_dir);
    assert_failure(&next);
    assert_stderr_contains(&next, "learner project changed since stage");

    let _ = fs::remove_dir_all(project_dir);
}

#[test]
fn project_pack_source_is_pinned_at_initialization() {
    let root = temp_project_path("pack-pin");
    let packs_a = root.join("packs-a");
    let packs_b = root.join("packs-b");
    copy_dir_recursive(
        &repo_root().join("packs/flashindex"),
        &packs_a.join("flashindex"),
    );
    copy_dir_recursive(
        &repo_root().join("packs/flashindex"),
        &packs_b.join("flashindex"),
    );
    let project = root.join("project");
    let init = run_deltaforge(
        [
            "--packs-dir",
            packs_a.to_str().unwrap(),
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);

    let overview = run_deltaforge(
        ["--packs-dir", packs_b.to_str().unwrap(), "overview"],
        &project,
    );
    assert_failure(&overview);
    assert_stderr_contains(&overview, "pinned to pack source");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn authoring_failures_are_transactional_and_yaml_values_are_escaped() {
    let root = temp_project_path("authoring-transaction");
    let create = run_deltaforge(
        [
            "pack",
            "new",
            "quotedpack",
            "--name",
            "Sample: Pack",
            "--description",
            "Description: still valid",
            "--dest",
            root.to_str().unwrap(),
        ],
        &repo_root(),
    );
    assert_success(&create);
    let pack = root.join("quotedpack");
    let manifest_path = pack.join("project.yaml");
    let manifest_before = fs::read_to_string(&manifest_path).unwrap();
    let parsed: serde_yaml::Value = serde_yaml::from_str(&manifest_before).unwrap();
    assert_eq!(parsed["name"], "Sample: Pack");
    fs::create_dir_all(pack.join("stages/02_conflict")).unwrap();

    let add = run_deltaforge(
        [
            "pack",
            "add-stage",
            "--pack-dir",
            pack.to_str().unwrap(),
            "02_conflict",
            "--title",
            "Conflict",
        ],
        &repo_root(),
    );
    assert_failure(&add);
    assert_eq!(fs::read_to_string(&manifest_path).unwrap(), manifest_before);

    let reference = run_deltaforge(
        [
            "--packs-dir",
            root.to_str().unwrap(),
            "pack",
            "check-reference",
            "quotedpack",
            "--reference",
            pack.join("templates/rust/src/main.rs").to_str().unwrap(),
            "--json",
        ],
        &repo_root(),
    );
    assert_failure(&reference);
    assert_stdout_contains(&reference, "reference solution failed");
    assert_stdout_not_contains(&reference, "reference init failed");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn invalid_hint_benchmark_and_config_values_are_actionable() {
    let project = temp_project_path("invalid-values");
    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));
    let hint = run_deltaforge(["hint", "--level", "0"], &project);
    assert_failure(&hint);
    assert_stderr_contains(&hint, "hint level must be greater than 0");
    let bench = run_deltaforge(["bench", "--iterations", "0", "--json"], &project);
    assert_failure(&bench);
    assert!(bench.stdout.is_empty());
    assert_stderr_contains(&bench, "iterations must be greater than 0");

    fs::write(
        project.join(".deltaforge/config.toml"),
        "schema_version = 1\nunknown_setting = true\n",
    )
    .unwrap();
    let doctor = run_deltaforge(["doctor", "--json"], &project);
    assert_success(&doctor);
    let report: serde_json::Value = serde_json::from_slice(&doctor.stdout).unwrap();
    assert!(
        report["project_error"]
            .as_str()
            .unwrap()
            .contains("unknown field")
    );
    let _ = fs::remove_dir_all(project);
}

#[test]
fn failed_benchmark_returns_failure_with_json_only_on_stdout() {
    let project = temp_project_path("bench-failure");
    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));
    fs::write(
        project.join("src/main.rs"),
        "fn main() { std::process::exit(1); }\n",
    )
    .unwrap();
    let bench = run_deltaforge(
        ["bench", "--iterations", "1", "--warmup", "0", "--json"],
        &project,
    );
    assert_failure(&bench);
    let report: serde_json::Value = serde_json::from_slice(&bench.stdout).unwrap();
    assert_eq!(report[0]["points"][0]["success"], false);
    assert_stderr_contains(&bench, "one or more benchmarks failed");
    let _ = fs::remove_dir_all(project);
}

#[test]
fn mcp_recovers_after_malformed_messages_and_negotiates_protocols() {
    let mut child = Command::new(deltaforge_pack_mcp_bin())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(b"not-json\n").unwrap();
        stdin.write_all(br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"1900-01-01"}}"#).unwrap();
        stdin.write_all(b"\n").unwrap();
        stdin.write_all(br#"{"jsonrpc":"2.0","id":2,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}"#).unwrap();
        stdin.write_all(b"\n").unwrap();
        stdin
            .write_all(br#"{"jsonrpc":"2.0","id":3,"method":"tools/list"}"#)
            .unwrap();
        stdin.write_all(b"\n").unwrap();
    }
    let output = child.wait_with_output().unwrap();
    assert_success(&output);
    let responses = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(responses.len(), 4);
    assert_eq!(responses[0]["error"]["code"], -32700);
    assert_eq!(responses[1]["result"]["protocolVersion"], "2025-11-25");
    assert_eq!(responses[2]["result"]["protocolVersion"], "2025-06-18");
    assert!(responses[3]["result"]["tools"].as_array().unwrap().len() >= 6);
}

#[test]
fn auto_commit_preserves_json_stdout_and_git_tag_errors_are_reported() {
    let project = temp_project_path("auto-commit");
    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
        ],
        &repo_root(),
    ));
    assert_success(&run_git(
        ["config", "user.name", "DeltaForge Test"],
        &project,
    ));
    assert_success(&run_git(
        ["config", "user.email", "deltaforge@example.invalid"],
        &project,
    ));
    fs::copy(
        repo_root().join("tools/reference_solutions/flashindex_rust/src/main.rs"),
        project.join("src/main.rs"),
    )
    .unwrap();
    fs::write(
        project.join(".deltaforge/config.toml"),
        "schema_version = 1\n[git]\nauto_commit = true\nauto_tag = false\n",
    )
    .unwrap();

    let test = run_deltaforge(["test", "--json"], &project);
    assert_success(&test);
    serde_json::from_slice::<serde_json::Value>(&test.stdout).unwrap();
    let log = run_git(["log", "-1", "--pretty=%s"], &project);
    assert_success(&log);
    assert_stdout_contains(&log, "Complete Stage 01: Scan files");

    assert_success(&run_git(["tag", "deltaforge-01_scan_files"], &project));
    fs::write(
        project.join(".deltaforge/config.toml"),
        "schema_version = 1\n[git]\nauto_commit = false\nauto_tag = true\n",
    )
    .unwrap();
    fs::write(project.join("design.txt"), "force another commit\n").unwrap();
    let commit = run_deltaforge(["commit", "--force"], &project);
    assert_failure(&commit);
    assert_stderr_contains(&commit, "git tag deltaforge-01_scan_files failed");
    let _ = fs::remove_dir_all(project);
}

#[test]
fn strict_pack_and_config_schemas_reject_typos_and_unsafe_paths() {
    let root = temp_project_path("strict-schemas");
    let packs = root.join("packs");
    copy_dir_recursive(
        &repo_root().join("packs/flashindex"),
        &packs.join("flashindex"),
    );
    let tests_path = packs.join("flashindex/stages/01_scan_files/tests.yaml");
    let tests = fs::read_to_string(&tests_path).unwrap();
    fs::write(
        &tests_path,
        tests.replacen(
            "    fixture:",
            "    misspelled_field: true\n    fixture:",
            1,
        ),
    )
    .unwrap();
    let validate = run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "validate-pack",
            "flashindex",
        ],
        &repo_root(),
    );
    assert_failure(&validate);
    assert_stdout_contains(&validate, "unknown field");

    copy_dir_recursive(
        &repo_root().join("packs/flashindex"),
        &packs.join("unsafe-pack"),
    );
    let manifest_path = packs.join("unsafe-pack/project.yaml");
    let manifest = fs::read_to_string(&manifest_path)
        .unwrap()
        .replace("id: flashindex", "id: unsafe-pack")
        .replace("path: stages/01_scan_files", "path: ../outside");
    fs::write(&manifest_path, manifest).unwrap();
    let unsafe_pack = run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "validate-pack",
            "unsafe-pack",
        ],
        &repo_root(),
    );
    assert_failure(&unsafe_pack);
    assert_stderr_contains(&unsafe_pack, "stage 01_scan_files path is unsafe");

    let project = root.join("project");
    assert_success(&run_deltaforge(
        [
            "init",
            "minikv",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));
    fs::write(
        project.join(".deltaforge/config.toml"),
        "schema_version = 1\n[bench]\niterations = 0\n",
    )
    .unwrap();
    let config = run_deltaforge(["config", "validate"], &project);
    assert_failure(&config);
    assert_stderr_contains(&config, "bench.iterations must be greater than 0");
    let _ = fs::remove_dir_all(root);
}

#[test]
fn legacy_schema_v1_state_loads_but_requires_a_fresh_completion_proof() {
    let project = temp_project_path("legacy-state");
    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));

    let state_path = project.join(".deltaforge/state.json");
    let mut state: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&state_path).unwrap()).unwrap();
    let state_object = state.as_object_mut().unwrap();
    state_object.remove("pack_version");
    state_object.remove("pack_source");
    state_object.remove("pack_digest");
    state_object.remove("completion_proofs");
    state_object.remove("gate_results");
    state_object.insert(
        "completed_stages".to_string(),
        serde_json::json!(["01_scan_files"]),
    );
    fs::write(&state_path, serde_json::to_string_pretty(&state).unwrap()).unwrap();

    let status = run_deltaforge(["status"], &project);
    assert_success(&status);
    assert_stdout_contains(&status, "01_scan_files");
    let next = run_deltaforge(["next"], &project);
    assert_failure(&next);
    assert_stderr_contains(&next, "has no integrity proof");
    assert_stderr_contains(&next, "deltaforge test");

    fs::copy(
        repo_root().join("tools/reference_solutions/flashindex_rust/src/main.rs"),
        project.join("src/main.rs"),
    )
    .unwrap();
    let test = run_deltaforge(["test"], &project);
    assert_success(&test);
    let next = run_deltaforge(["next"], &project);
    assert_success(&next);
    assert_stdout_contains(&next, "Unlocked Stage 02_filter_files");
    let migrated: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&state_path).unwrap()).unwrap();
    assert!(migrated["completion_proofs"]["01_scan_files"].is_object());

    let _ = fs::remove_dir_all(project);
}

fn assert_reference_solution_passes(pack: &str, source_path: &str, final_stage: &str) {
    let project_dir = temp_project_path(&format!("reference-{pack}"));
    let init = run_deltaforge(
        [
            "init",
            pack,
            "--lang",
            "rust",
            "--name",
            project_dir.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    );
    assert_success(&init);

    fs::copy(
        repo_root().join(source_path),
        project_dir.join("src/main.rs"),
    )
    .unwrap();

    let all = run_deltaforge(["test", "--all"], &project_dir);
    assert_success(&all);
    assert_stdout_contains(&all, final_stage);
    assert_stdout_contains(&all, "0 failed");
}

fn init_project_from_pack_copy(name: &str) -> (PathBuf, PathBuf, PathBuf) {
    let root = temp_project_path(name);
    let packs = root.join("packs");
    copy_dir_recursive(
        &repo_root().join("packs/flashindex"),
        &packs.join("flashindex"),
    );
    let project = root.join("project");

    assert_success(&run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));
    fs::write(project.join("src/main.rs"), passing_flashindex_source()).unwrap();
    (root, packs, project)
}

fn append_scan_gate(packs: &Path, name: &str, max: &str, advice: &str) {
    let path = packs.join("flashindex/stages/01_scan_files/benchmarks.yaml");
    let mut source = fs::read_to_string(&path).unwrap();
    source.push_str(&format!(
        "\nperformance_gates:\n  - name: {name}\n    benchmark: scan_basic_project\n    metric: runtime_median_ms\n    max: {max}\n    advice: [\"{advice}\"]\n"
    ));
    fs::write(path, source).unwrap();
}

#[test]
fn sync_pack_doc_only_update_keeps_proofs_valid() {
    let (root, packs, project) = init_project_from_pack_copy("sync-pack-docs");

    let test = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "test"], &project);
    assert_success(&test);

    // A documentation-only pack update: instructions change, behavior does not.
    let instructions = packs.join("flashindex/stages/01_scan_files/instructions.md");
    let mut updated = fs::read_to_string(&instructions).unwrap();
    updated.push_str("\n<!-- upgraded pack content -->\n");
    fs::write(&instructions, updated).unwrap();

    let stale = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "overview"],
        &project,
    );
    assert_failure(&stale);
    assert_stderr_contains(&stale, "deltaforge sync-pack");

    let sync = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    );
    assert_success(&sync);
    assert_stdout_contains(&sync, "Re-pinned project flashindex");
    assert_stdout_contains(&sync, "✓ 01_scan_files");
    assert_stdout_not_contains(&sync, "needs revalidation");

    // The stage's behavioral digest is unchanged, so the proof stays valid and
    // progression works without re-running tests.
    let next = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "next"], &project);
    assert_success(&next);
    assert_stdout_contains(&next, "Unlocked Stage 02_filter_files");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn sync_pack_behavioral_update_requires_revalidation() {
    let (root, packs, project) = init_project_from_pack_copy("sync-pack-behavior");

    let test = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "test"], &project);
    assert_success(&test);

    // A behavioral pack update: stage 01 gains a test the learner never ran.
    let tests_path = packs.join("flashindex/stages/01_scan_files/tests.yaml");
    let mut tests = fs::read_to_string(&tests_path).unwrap();
    tests.push_str(
        r#"
  - name: revalidation smoke test
    fixture: basic_project
    command: ["scan", "{fixture_path}"]
    expect:
      exit_code: 0
      stdout_contains:
        - "README.md"
"#,
    );
    fs::write(&tests_path, tests).unwrap();

    let sync = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "sync-pack"],
        &project,
    );
    assert_success(&sync);
    assert_stdout_contains(&sync, "! 01_scan_files (needs revalidation)");

    // Progression is blocked until the stage is revalidated against the new
    // tests; the proof must not pretend hash-B was proven by a hash-A run.
    let next = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "next"], &project);
    assert_failure(&next);
    assert_stderr_contains(&next, "must be revalidated");
    assert_stderr_contains(&next, "deltaforge test");

    let status = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "status"], &project);
    assert_success(&status);
    assert_stdout_contains(&status, "! 01_scan_files");
    assert_stdout_contains(&status, "older version of this pack");

    let retest = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "test"], &project);
    assert_success(&retest);
    assert_stdout_contains(&retest, "revalidation smoke test");
    let next = run_deltaforge(["--packs-dir", packs.to_str().unwrap(), "next"], &project);
    assert_success(&next);
    assert_stdout_contains(&next, "Unlocked Stage 02_filter_files");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn sync_pack_migrates_legacy_proofs_when_pack_is_unchanged() {
    let project = temp_project_path("sync-pack-legacy");
    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));
    fs::write(project.join("src/main.rs"), passing_flashindex_source()).unwrap();
    assert_success(&run_deltaforge(["test"], &project));

    // Simulate a proof recorded before behavioral digests existed.
    let state_path = project.join(".deltaforge/state.json");
    let mut state: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&state_path).unwrap()).unwrap();
    let proof = state["completion_proofs"]["01_scan_files"]
        .as_object_mut()
        .unwrap();
    proof.remove("behavioral_digest");
    fs::write(&state_path, serde_json::to_string_pretty(&state).unwrap()).unwrap();

    // The pack is bit-identical to the one that passed, so sync-pack can
    // safely upgrade the legacy proof to a behavioral digest.
    let sync = run_deltaforge(["sync-pack"], &project);
    assert_success(&sync);
    assert_stdout_contains(&sync, "migrated legacy completion proofs: 1");
    assert_stdout_contains(&sync, "✓ 01_scan_files");

    let migrated: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&state_path).unwrap()).unwrap();
    assert!(
        migrated["completion_proofs"]["01_scan_files"]["behavioral_digest"]
            .as_str()
            .unwrap()
            .starts_with("fnv1a64:")
    );

    let next = run_deltaforge(["next"], &project);
    assert_success(&next);
    assert_stdout_contains(&next, "Unlocked Stage 02_filter_files");

    let _ = fs::remove_dir_all(project);
}

#[test]
fn sync_pack_reports_changes_as_json() {
    let root = temp_project_path("sync-pack-json");
    let packs = root.join("packs");
    copy_dir_recursive(
        &repo_root().join("packs/flashindex"),
        &packs.join("flashindex"),
    );
    let project = root.join("project");
    assert_success(&run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));

    let sync = run_deltaforge(
        [
            "--packs-dir",
            packs.to_str().unwrap(),
            "sync-pack",
            "--json",
        ],
        &project,
    );
    assert_success(&sync);
    let report: serde_json::Value =
        serde_json::from_slice(&sync.stdout).expect("sync-pack --json emits valid JSON");
    assert_eq!(report["project"], "flashindex");
    assert!(
        report["digest"]["new"]
            .as_str()
            .unwrap()
            .starts_with("fnv1a64:")
    );
    assert_eq!(report["migrated_proofs"], 0);
    assert!(report["stages"].as_array().unwrap().is_empty());
    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn symlink_in_learner_project_does_not_block_completion() {
    let project = temp_project_path("symlink-project");
    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));
    fs::write(project.join("src/main.rs"), passing_flashindex_source()).unwrap();
    std::os::unix::fs::symlink(project.join("Cargo.toml"), project.join("venv-link")).unwrap();

    let test = run_deltaforge(["test"], &project);
    assert_success(&test);
    let next = run_deltaforge(["next"], &project);
    assert_success(&next);
    assert_stdout_contains(&next, "Unlocked Stage 02_filter_files");
    let _ = fs::remove_dir_all(project);
}

#[cfg(unix)]
#[test]
fn changing_a_symlink_target_invalidates_the_completion_proof() {
    let root = temp_project_path("symlink-target");
    let project = root.join("project");
    let external = root.join("external");
    fs::create_dir_all(&external).unwrap();
    fs::write(external.join("shared.md"), "notes v1").unwrap();

    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));
    fs::write(project.join("src/main.rs"), passing_flashindex_source()).unwrap();
    std::os::unix::fs::symlink(external.join("shared.md"), project.join("NOTES.md")).unwrap();

    assert_success(&run_deltaforge(["test"], &project));

    // The digest hashes the symlink target's contents, so editing the target
    // is detected exactly like editing a regular project file.
    fs::write(external.join("shared.md"), "notes v2").unwrap();
    let next = run_deltaforge(["next"], &project);
    assert_failure(&next);
    assert_stderr_contains(&next, "learner project changed");

    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn directory_symlink_blocks_proof_until_excluded_in_config() {
    let root = temp_project_path("symlink-dir");
    let project = root.join("project");
    let external = root.join("external");
    fs::create_dir_all(&external).unwrap();
    fs::write(external.join("data.txt"), "x").unwrap();

    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));
    fs::write(project.join("src/main.rs"), passing_flashindex_source()).unwrap();
    std::os::unix::fs::symlink(&external, project.join("extdata")).unwrap();

    // Tests pass, but the completion proof cannot be recorded because the
    // digest would not cover what the directory symlink points at.
    let test = run_deltaforge(["test"], &project);
    assert_failure(&test);
    assert_stderr_contains(&test, "symbolic link to a directory");
    assert_stderr_contains(&test, "integrity.exclude");

    // The learner-config escape hatch unblocks the digest.
    fs::write(
        project.join(".deltaforge/config.toml"),
        "[integrity]\nexclude = [\"extdata\"]\n",
    )
    .unwrap();
    assert_success(&run_deltaforge(["test"], &project));
    let next = run_deltaforge(["next"], &project);
    assert_success(&next);
    assert_stdout_contains(&next, "Unlocked Stage 02_filter_files");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn broken_pack_in_search_dir_does_not_break_list() {
    let root = temp_project_path("broken-pack");
    let packs = root.join("packs");
    fs::create_dir_all(packs.join("brokenpack")).unwrap();
    // Missing required fields makes the manifest fail to parse.
    fs::write(
        packs.join("brokenpack/project.yaml"),
        "schema_version: 1\nname: incomplete\n",
    )
    .unwrap();

    let list = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "list"],
        &repo_root(),
    );
    assert_success(&list);
    assert_stdout_contains(&list, "flashindex");
    assert_stderr_contains(&list, "skipping invalid pack");

    // validate-pack over the same directory surfaces the failure and exits non-zero.
    let validate = run_deltaforge(
        ["--packs-dir", packs.to_str().unwrap(), "validate-pack"],
        &repo_root(),
    );
    assert_failure(&validate);
    assert_stdout_contains(&validate, "is invalid");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_failure_output_includes_actual_stdout() {
    let project = temp_project_path("failure-output");
    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));

    let test = run_deltaforge(["test"], &project);
    assert_failure(&test);
    assert_stdout_contains(&test, "actual stdout");
    assert_stdout_contains(&test, "FlashIndex starter");
    let _ = fs::remove_dir_all(project);
}

#[test]
fn status_json_reports_per_stage_status() {
    let project = temp_project_path("status-json");
    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));

    let status = run_deltaforge(["status", "--json"], &project);
    assert_success(&status);
    assert!(
        String::from_utf8_lossy(&status.stderr).is_empty(),
        "{}",
        output_text(&status)
    );
    let parsed: serde_json::Value =
        serde_json::from_slice(&status.stdout).expect("status --json emits valid JSON");
    assert_eq!(parsed["project"], "flashindex");
    assert_eq!(parsed["current_stage"], "01_scan_files");
    assert_eq!(parsed["stages"][0]["status"], "current");
    let _ = fs::remove_dir_all(project);
}

#[test]
fn hint_level_never_lowers_recorded_progress() {
    let project = temp_project_path("hint-progress");
    assert_success(&run_deltaforge(
        [
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ],
        &repo_root(),
    ));

    assert_success(&run_deltaforge(["hint", "--level", "3"], &project));
    assert_success(&run_deltaforge(["hint", "--level", "1"], &project));

    let state: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(project.join(".deltaforge/state.json")).unwrap())
            .unwrap();
    assert_eq!(state["hint_state"]["01_scan_files"], 3);
    let _ = fs::remove_dir_all(project);
}

fn passing_flashindex_source() -> &'static str {
    r#"use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        eprintln!("usage: flashindex <scan|tokenize> <path>");
        return ExitCode::FAILURE;
    }

    let root = PathBuf::from(&args[1]);
    let result = match args[0].as_str() {
        "scan" => scan(&root),
        "tokenize" => tokenize(&root),
        command => {
            eprintln!("unknown command: {command}");
            return ExitCode::FAILURE;
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn scan(root: &Path) -> Result<(), String> {
    let mut files = collect_files(root)?;
    files.sort();

    for file in files {
        if is_source_like(&file) {
            println!("{}", display_relative(&file));
        }
    }

    Ok(())
}

fn tokenize(root: &Path) -> Result<(), String> {
    let mut files = collect_files(root)?;
    files.sort();

    for file in files {
        if !is_source_like(&file) {
            continue;
        }

        let path = root.join(&file);
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;

        for (line_index, line) in source.lines().enumerate() {
            let mut start_column = None;
            let mut token = String::new();

            for (column_index, ch) in line.chars().chain(std::iter::once(' ')).enumerate() {
                let column = column_index + 1;
                if ch == '_' || ch.is_ascii_alphanumeric() {
                    if token.is_empty() && ch.is_ascii_digit() {
                        continue;
                    }
                    if token.is_empty() {
                        start_column = Some(column);
                    }
                    token.push(ch);
                } else if !token.is_empty() {
                    println!(
                        "{}:{}:{} {}",
                        display_relative(&file),
                        line_index + 1,
                        start_column.unwrap(),
                        token
                    );
                    token.clear();
                    start_column = None;
                }
            }
        }
    }

    Ok(())
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    visit(root, root, &mut files)?;
    Ok(files)
}

fn visit(root: &Path, current: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(current)
        .map_err(|error| format!("failed to read {}: {error}", current.display()))?
    {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| error.to_string())?;
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if file_type.is_dir() {
            if matches!(name.as_ref(), ".git" | "target" | "build" | "node_modules") {
                continue;
            }
            visit(root, &path, files)?;
        } else if file_type.is_file() {
            files.push(
                path.strip_prefix(root)
                    .map_err(|error| error.to_string())?
                    .to_path_buf(),
            );
        }
    }

    Ok(())
}

fn is_source_like(path: &Path) -> bool {
    if path.file_name().and_then(|name| name.to_str()) == Some("CMakeLists.txt") {
        return true;
    }

    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("c" | "cpp" | "h" | "hpp" | "rs" | "py" | "glsl" | "md" | "txt" | "cmake")
    )
}

fn display_relative(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
"#
}
