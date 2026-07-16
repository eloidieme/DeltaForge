use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use deltaforge::application;
use deltaforge::context::GlobalOptions;

struct Case {
    name: &'static str,
    source: String,
    timeout_ms: Option<u64>,
    expected_priority: u32,
    expected_kind: &'static str,
    expected_headline: &'static str,
}

fn deltaforge_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_deltaforge"))
}

fn temp_project_path(case: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "deltaforge-phase1-corpus-{case}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn scanner_source(
    print_output: bool,
    maximum_depth: &str,
    absolute_paths: bool,
    reverse_order: bool,
    apply_ignores: bool,
) -> String {
    format!(
        r#"use std::env;
use std::fs;
use std::path::{{Path, PathBuf}};
use std::process::ExitCode;

const PRINT_OUTPUT: bool = {print_output};
const MAXIMUM_DEPTH: usize = {maximum_depth};
const ABSOLUTE_PATHS: bool = {absolute_paths};
const REVERSE_ORDER: bool = {reverse_order};
const APPLY_IGNORES: bool = {apply_ignores};

fn main() -> ExitCode {{
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 || args[0] != "scan" {{
        return ExitCode::FAILURE;
    }}
    match scan(Path::new(&args[1])) {{
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {{ eprintln!("{{error}}"); ExitCode::FAILURE }}
    }}
}}

fn scan(root: &Path) -> Result<(), String> {{
    let mut files = Vec::new();
    visit(root, root, 0, &mut files)?;
    let mut output = files.into_iter().map(|path| {{
        let visible = if ABSOLUTE_PATHS {{ path }} else {{
            path.strip_prefix(root).map_err(|error| error.to_string())?.to_path_buf()
        }};
        Ok(visible.components().map(|part| part.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/"))
    }}).collect::<Result<Vec<String>, String>>()?;
    output.sort();
    if REVERSE_ORDER {{ output.reverse(); }}
    if PRINT_OUTPUT {{ for path in output {{ println!("{{path}}"); }} }}
    Ok(())
}}

fn visit(root: &Path, current: &Path, depth: usize, files: &mut Vec<PathBuf>) -> Result<(), String> {{
    for entry in fs::read_dir(current).map_err(|error| error.to_string())? {{
        let entry = entry.map_err(|error| error.to_string())?;
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        let path = entry.path();
        if kind.is_dir() {{
            let name = entry.file_name();
            let ignored = matches!(name.to_string_lossy().as_ref(), ".git" | "target" | "build" | "node_modules");
            if (!APPLY_IGNORES || !ignored) && depth < MAXIMUM_DEPTH {{
                visit(root, &path, depth + 1, files)?;
            }}
        }} else if kind.is_file() {{ files.push(path); }}
    }}
    Ok(())
}}
"#
    )
}

fn timeout_source() -> String {
    r#"use std::env;
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 || args[0] != "scan" { return ExitCode::FAILURE; }
    thread::sleep(Duration::from_secs(2));
    ExitCode::SUCCESS
}
"#
    .to_string()
}

fn cases() -> Vec<Case> {
    vec![
        Case {
            name: "no-output",
            source: scanner_source(false, "usize::MAX", false, false, true),
            timeout_ms: None,
            expected_priority: 10,
            expected_kind: "stdout-contains",
            expected_headline: "Required project files are missing",
        },
        Case {
            name: "missing-nested-files",
            source: scanner_source(true, "1", false, false, true),
            timeout_ms: None,
            expected_priority: 20,
            expected_kind: "stdout-contains",
            expected_headline: "Discovery stops before nested files",
        },
        Case {
            name: "absolute-paths",
            source: scanner_source(true, "usize::MAX", true, false, true),
            timeout_ms: None,
            expected_priority: 25,
            expected_kind: "stdout-excludes",
            expected_headline: "A discovered path leaks the machine-specific root",
        },
        Case {
            name: "unstable-ordering",
            source: scanner_source(true, "usize::MAX", false, true, true),
            timeout_ms: None,
            expected_priority: 40,
            expected_kind: "stdout-exact",
            expected_headline: "The path stream is not deterministic",
        },
        Case {
            name: "unexpected-files",
            source: scanner_source(true, "usize::MAX", false, false, false),
            timeout_ms: None,
            expected_priority: 30,
            expected_kind: "stdout-excludes",
            expected_headline: "Generated or dependency files entered the scan",
        },
        Case {
            name: "build-failure",
            source: "fn main( {\n".to_string(),
            timeout_ms: None,
            expected_priority: 0,
            expected_kind: "build",
            expected_headline: "The project did not build",
        },
        Case {
            name: "timeout",
            source: timeout_source(),
            timeout_ms: Some(100),
            expected_priority: 1,
            expected_kind: "runner",
            expected_headline: "The test command did not finish",
        },
    ]
}

fn prepare_project(path: &Path, case: &Case) {
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            path.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "{}: initialization failed: {}",
        case.name,
        String::from_utf8_lossy(&init.stderr)
    );
    fs::write(path.join("src/main.rs"), &case.source).unwrap();
    if let Some(timeout_ms) = case.timeout_ms {
        let config_path = path.join(".deltaforge/config.toml");
        let config = fs::read_to_string(&config_path).unwrap();
        let config = config.replace("timeout_ms = 5000", &format!("timeout_ms = {timeout_ms}"));
        assert!(config.contains(&format!("timeout_ms = {timeout_ms}")));
        fs::write(config_path, config).unwrap();
    }
}

#[test]
fn every_phase1_failure_has_the_expected_primary_diagnosis() {
    for case in cases() {
        let project = temp_project_path(case.name);
        prepare_project(&project, &case);
        let run = Command::new(deltaforge_bin())
            .args(["test", "--fail-fast"])
            .current_dir(&project)
            .output()
            .unwrap();
        assert!(!run.status.success(), "{} unexpectedly passed", case.name);

        let options = GlobalOptions {
            project_dir: Some(project.clone()),
            ..GlobalOptions::default()
        };
        let state = application::load_workbench_state(&options)
            .unwrap_or_else(|error| panic!("{}: state failed: {error:#}", case.name));
        let failure = state
            .primary_failure
            .unwrap_or_else(|| panic!("{}: no primary failure", case.name));
        let diagnosis = failure
            .diagnosis
            .unwrap_or_else(|| panic!("{}: failure has no diagnosis", case.name));

        assert_eq!(diagnosis.priority, case.expected_priority, "{}", case.name);
        assert_eq!(diagnosis.kind, case.expected_kind, "{}", case.name);
        assert_eq!(diagnosis.headline, case.expected_headline, "{}", case.name);
        assert!(!diagnosis.summary.trim().is_empty(), "{}", case.name);
        assert!(!diagnosis.contract.trim().is_empty(), "{}", case.name);
        assert!(
            diagnosis
                .expected
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty()),
            "{}: expected evidence is empty",
            case.name
        );
        assert!(
            diagnosis
                .actual
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty()),
            "{}: observed evidence is empty",
            case.name
        );
        if case.expected_kind != "build" {
            assert!(!diagnosis.command.is_empty(), "{}", case.name);
            assert!(!diagnosis.fixture_entries.is_empty(), "{}", case.name);
        }
        let temporary_root = std::env::temp_dir().to_string_lossy().to_string();
        for evidence in failure
            .failures
            .iter()
            .chain(diagnosis.command.iter())
            .chain(std::iter::once(&diagnosis.summary))
            .chain(diagnosis.expected.iter())
            .chain(diagnosis.actual.iter())
        {
            assert!(
                !evidence.contains(&temporary_root),
                "{} leaked a temporary path in {evidence:?}",
                case.name
            );
        }

        let _ = fs::remove_dir_all(project);
    }
}
