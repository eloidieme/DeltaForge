//! Evidence for the 1.0 surface-completeness claim: the whole learner journey
//! is reachable from the browser, and the terminal is used only to write code.
//!
//! Every request below is the exact HTTP exchange the workbench page makes,
//! Origin header and all. Nothing here shells out to a DeltaForge command
//! except the one call that writes source into the project, which stands in
//! for the learner's editor.

use std::fs;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);
static JOURNEY_LOCK: Mutex<()> = Mutex::new(());

/// These tests each run a workbench service that writes to a shared discovery
/// record, so they run one at a time.
fn journey_guard() -> MutexGuard<'static, ()> {
    JOURNEY_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn deltaforge_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_deltaforge"))
}

fn temp_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "deltaforge-journey-{label}-{}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed),
    ))
}

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// A running workbench service plus the sandbox it was given: its own
/// DeltaForge home (so the project registry is isolated) and its own workspace
/// root (so creation writes nowhere near the developer's home).
struct Service {
    _child: ChildGuard,
    port: u16,
    token: &'static str,
    workspace: PathBuf,
    home: PathBuf,
    _root: PathBuf,
}

const TOKEN: &str = "journey-token";

fn start_service(label: &str) -> Service {
    start_service_with(label, Workspace::Existing)
}

/// Whether the sandbox has a workspace directory before the service starts.
///
/// It matters, and 1.0 is the reason it now has a name: every harness this
/// project had created the workspace first, so the one path a real learner
/// takes — a machine where that directory has never existed — was the one path
/// nothing exercised.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Workspace {
    Existing,
    Missing,
}

/// Start a service that also exposes `POST /api/v1/__panic`.
fn start_service_with_panic_probe(label: &str) -> Service {
    // SAFETY: single-threaded here, and `journey_guard` serialises every test
    // in this file, so no other thread is reading the environment.
    unsafe { std::env::set_var("DELTAFORGE_PANIC_PROBE", "1") };
    let service = start_service(label);
    unsafe { std::env::remove_var("DELTAFORGE_PANIC_PROBE") };
    service
}

fn start_service_with(label: &str, workspace_state: Workspace) -> Service {
    let root = temp_root(label);
    let home = root.join("home");
    let workspace = root.join("workspace");
    fs::create_dir_all(&home).unwrap();
    if workspace_state == Workspace::Existing {
        fs::create_dir_all(&workspace).unwrap();
    }

    let child = Command::new(deltaforge_bin())
        .env("DELTAFORGE_HOME", &home)
        .env("DELTAFORGE_WORKSPACE", &workspace)
        // Hooks disabled, and a Git identity set explicitly: `cli_flow.rs`
        // configures a repo-local identity before its own snapshot tests, but
        // the real `git commit` here (via POST /api/v1/snapshots) has no
        // repo yet to configure one into, and a minimal environment (no
        // global identity, unlike this machine or the CI runners) makes Git
        // refuse with "unable to auto-detect email address" for reasons
        // unrelated to what this test is checking.
        .env("GIT_CONFIG_COUNT", "3")
        .env("GIT_CONFIG_KEY_0", "core.hooksPath")
        .env("GIT_CONFIG_VALUE_0", "NUL")
        .env("GIT_CONFIG_KEY_1", "user.name")
        .env("GIT_CONFIG_VALUE_1", "DeltaForge Tests")
        .env("GIT_CONFIG_KEY_2", "user.email")
        .env("GIT_CONFIG_VALUE_2", "deltaforge@example.com")
        .args([
            "__workbench",
            "--token",
            TOKEN,
            "--idle-timeout-ms",
            "10000",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let guard = ChildGuard(child);

    let record_path = home.join("workbench.json");
    let deadline = Instant::now() + Duration::from_secs(10);
    let port = loop {
        if let Ok(source) = fs::read_to_string(&record_path)
            && let Ok(record) = serde_json::from_str::<serde_json::Value>(&source)
            && let Some(port) = record["port"].as_u64()
        {
            break port as u16;
        }
        assert!(
            Instant::now() < deadline,
            "the workbench service did not start"
        );
        std::thread::sleep(Duration::from_millis(50));
    };

    Service {
        _child: guard,
        port,
        token: TOKEN,
        workspace,
        home,
        _root: root,
    }
}

impl Service {
    fn get(&self, path: &str) -> serde_json::Value {
        json(&self.raw("GET", path, ""))
    }

    fn post(&self, path: &str, body: serde_json::Value) -> serde_json::Value {
        json(&self.raw("POST", path, &body.to_string()))
    }

    fn raw(&self, method: &str, path: &str, body: &str) -> String {
        let separator = if path.contains('?') { '&' } else { '?' };
        let target = format!("{path}{separator}token={}", self.token);
        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, self.port)).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .unwrap();
        let mut request = format!(
            "{method} {target} HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nOrigin: http://127.0.0.1:{}\r\nConnection: close\r\n",
            self.port, self.port
        );
        if method == "POST" {
            request.push_str(&format!(
                "Content-Type: application/json\r\nContent-Length: {}\r\n",
                body.len()
            ));
        }
        request.push_str("\r\n");
        stream.write_all(request.as_bytes()).unwrap();
        stream.write_all(body.as_bytes()).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    }

    /// Poll project state until `predicate` holds, then return it.
    fn wait_for(
        &self,
        project: &str,
        what: &str,
        predicate: impl Fn(&serde_json::Value) -> bool,
    ) -> serde_json::Value {
        let deadline = Instant::now() + Duration::from_secs(180);
        loop {
            let state = self.get(&format!("/api/v1/state?project={project}"));
            if predicate(&state) {
                return state;
            }
            assert!(Instant::now() < deadline, "timed out waiting for {what}");
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    fn idle(&self, project: &str, what: &str) -> serde_json::Value {
        self.wait_for(project, what, |state| state["active_job"].is_null())
    }
}

fn json(response: &str) -> serde_json::Value {
    let (head, body) = response.split_once("\r\n\r\n").unwrap();
    serde_json::from_str(body).unwrap_or_else(|error| {
        panic!("response was not JSON ({error}):\n{head}\n{body}");
    })
}

fn status(response: &str) -> &str {
    response.split_whitespace().nth(1).unwrap_or_default()
}

/// The FlashIndex stage-01 contract, satisfied. Stands in for what the learner
/// types into their editor; every other action in these tests is a browser
/// request.
fn passing_scan_source() -> &'static str {
    r#"
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 || args[0] != "scan" {
        eprintln!("usage: flashindex scan <path>");
        return ExitCode::FAILURE;
    }
    let root = Path::new(&args[1]);
    if !root.is_dir() {
        eprintln!("error: not a directory");
        return ExitCode::FAILURE;
    }
    let mut files = Vec::new();
    if let Err(error) = visit(root, root, &mut files) {
        eprintln!("error: {error}");
        return ExitCode::FAILURE;
    }
    let mut printable = files
        .iter()
        .map(|path| {
            path.components()
                .map(|part| part.as_os_str().to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join("/")
        })
        .collect::<Vec<_>>();
    printable.sort();
    for path in printable {
        println!("{path}");
    }
    ExitCode::SUCCESS
}

fn visit(root: &Path, current: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    const IGNORED: [&str; 4] = [".git", "target", "build", "node_modules"];
    for entry in fs::read_dir(current).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        if kind.is_dir() {
            if IGNORED.contains(&name.as_str()) {
                continue;
            }
            visit(root, &path, files)?;
        } else if kind.is_file() {
            files.push(
                path.strip_prefix(root)
                    .map_err(|error| error.to_string())?
                    .to_path_buf(),
            );
        }
    }
    Ok(())
}
"#
}

#[test]
fn the_whole_journey_is_reachable_from_the_browser() {
    let _guard = journey_guard();
    let service = start_service("full");

    // 1. The catalog. Flagship first, with everything a choice needs.
    let catalog = service.get("/api/v1/catalog");
    let entries = catalog.as_array().unwrap();
    assert!(!entries.is_empty(), "the catalog is empty");
    assert_eq!(entries[0]["id"], "flashindex");
    assert_eq!(entries[0]["tier"], "flagship");
    assert_eq!(entries[0]["step_count"], 14);
    assert!(entries[0]["estimated_hours"]["low"].as_u64().unwrap() > 0);
    assert_eq!(entries[0]["languages"][0]["id"], "rust");
    assert_eq!(entries[0]["languages"][0]["available"], true);
    assert!(
        entries
            .iter()
            .skip(1)
            .all(|entry| entry["tier"] == "preview"),
        "only FlashIndex ships at flagship quality in 1.0"
    );

    // The default location is offered without the learner typing a path.
    let workspace = service.get("/api/v1/workspace");
    let offered = Path::new(workspace["default_directory"].as_str().unwrap());
    assert_eq!(
        offered.canonicalize().unwrap(),
        service.workspace.canonicalize().unwrap()
    );

    // 2. Preflight before committing to anything.
    let preflight = service.post(
        "/api/v1/projects/preflight",
        serde_json::json!({
            "pack": "flashindex",
            "language": "rust",
            "parent_directory": service.workspace.to_str().unwrap(),
            "name": "flashindex-rust",
        }),
    );
    assert_eq!(preflight["ok"], true);
    assert_eq!(preflight["location"]["ok"], true);
    assert!(
        preflight["tools"]
            .as_array()
            .unwrap()
            .iter()
            .any(|tool| tool["program"] == "cargo" && tool["found"] == true),
        "preflight did not report the declared toolchain"
    );

    // 3. Creation.
    let created = service.post(
        "/api/v1/projects",
        serde_json::json!({
            "pack": "flashindex",
            "language": "rust",
            "parent_directory": service.workspace.to_str().unwrap(),
            "name": "flashindex-rust",
            "git": true,
        }),
    );
    let project = created["project_id"].as_str().unwrap().to_string();
    let project_path = PathBuf::from(created["path"].as_str().unwrap());
    assert_eq!(created["stage_id"], "01_scan_files");
    assert!(project_path.join(".deltaforge/state.json").is_file());
    assert!(project_path.join(".git").is_dir());

    // The new project is immediately addressable by identifier alone.
    let listed = service.get("/api/v1/projects");
    assert!(
        listed
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["id"] == project.as_str())
    );

    // 4. The correctness loop: the starter template fails, with a diagnosis.
    assert_eq!(
        status(&service.raw("POST", &format!("/api/v1/runs?project={project}"), "{}")),
        "202"
    );
    let failed = service.wait_for(&project, "the first run to finish", |state| {
        state["active_job"].is_null() && state["primary_failure"].is_object()
    });
    assert_eq!(failed["capability"]["completed"], false);
    let diagnosis = &failed["primary_failure"]["diagnosis"];
    assert!(!diagnosis["headline"].as_str().unwrap().is_empty());
    assert!(!diagnosis["contract"].as_str().unwrap().is_empty());

    // Help is reachable, and the retrospective stays locked until the step passes.
    let content = service.post(
        &format!("/api/v1/hints?project={project}"),
        serde_json::json!({}),
    );
    assert_eq!(content["revealed_help"][0]["level"], 1);
    assert_eq!(content["revealed_help"][0]["label"], "Observation");
    assert_eq!(content["help_levels"], 5);

    // 5. The learner writes code. This is the only terminal-shaped action.
    fs::write(project_path.join("src/main.rs"), passing_scan_source()).unwrap();

    assert_eq!(
        status(&service.raw("POST", &format!("/api/v1/runs?project={project}"), "{}")),
        "202"
    );
    let passed = service.wait_for(&project, "the passing run", |state| {
        state["active_job"].is_null() && state["capability"]["completed"] == true
    });
    assert_eq!(passed["freshness"], "fresh");
    assert_eq!(passed["primary_action"]["kind"], "begin_next_capability");

    // 6. The performance loop, from the browser.
    let performance = &passed["performance"];
    assert_eq!(performance["has_benchmarks"], true);
    // The prompt is rich text now, like every other authored string the page
    // renders: the source it was written as, plus the blocks the workbench
    // draws. It used to arrive as one flat string, so a prompt that named a
    // function in backticks showed the backticks.
    let prompt = &performance["prediction_prompt"];
    assert!(
        prompt["source"]
            .as_str()
            .is_some_and(|text| !text.is_empty()),
        "a measured step must offer a prediction prompt"
    );
    assert!(
        !prompt["blocks"].as_array().unwrap().is_empty(),
        "the prediction prompt did not render into blocks"
    );
    assert!(performance["latest"].as_array().unwrap().is_empty());

    let predicted = service.post(
        &format!("/api/v1/predictions?project={project}"),
        serde_json::json!({ "text": "Directory reads dominate.", "skipped": false }),
    );
    assert_eq!(predicted["performance"]["prediction"]["skipped"], false);

    assert_eq!(
        status(&service.raw(
            "POST",
            &format!("/api/v1/benchmarks?project={project}"),
            "{}"
        )),
        "202"
    );
    let measured = service.wait_for(&project, "the benchmark", |state| {
        state["active_job"].is_null()
            && !state["performance"]["latest"]
                .as_array()
                .unwrap()
                .is_empty()
    });
    let benchmark = &measured["performance"]["latest"][0];
    assert_eq!(benchmark["name"], "scan_basic_project");
    assert!(
        benchmark["points"][0]["runtime_median_ms"]
            .as_f64()
            .unwrap()
            > 0.0
    );

    // A benchmark is a job like any other: it appears in the same history with
    // its own kind.
    let attempts = measured["attempt_history"].as_array().unwrap();
    assert!(
        attempts
            .iter()
            .any(|attempt| attempt["kind"] == "benchmarks" && attempt["status"] == "passed"),
        "the benchmark did not appear in the attempt history"
    );
    assert!(attempts.iter().any(|attempt| attempt["kind"] == "tests"));

    let reflected = service.post(
        &format!("/api/v1/reflections?project={project}"),
        serde_json::json!({ "text": "", "skipped": true }),
    );
    assert_eq!(reflected["performance"]["reflection"]["skipped"], true);

    // 7. The snapshot offered at the pass moment shows the change first.
    let preview = service.get(&format!("/api/v1/snapshots/preview?project={project}"));
    assert_eq!(preview["available"], true);
    assert_eq!(preview["message"], "Complete Stage 01: Scan files");
    assert!(
        preview["changed_files"]
            .as_array()
            .unwrap()
            .iter()
            .any(|file| file["path"].as_str().unwrap().contains("main.rs")),
        "the snapshot preview did not list the edited source"
    );
    let snapshot = service.post(
        &format!("/api/v1/snapshots?project={project}"),
        serde_json::json!({}),
    );
    assert_eq!(snapshot["tag"], "deltaforge-01_scan_files");
    assert!(!snapshot["commit"].as_str().unwrap().is_empty());

    // 8. The record, with claims traced to what was recorded.
    let exported = service.post(
        &format!("/api/v1/reports?project={project}"),
        serde_json::json!({ "format": "markdown" }),
    );
    let record = exported["markdown"].as_str().unwrap();
    assert!(record.contains("1 of 14 steps complete."));
    assert!(record.contains("behavioral checks passing"));
    assert!(record.contains("scan_basic_project"));
    assert!(record.contains("deltaforge-01_scan_files"));
    assert!(record.contains("Directory reads dominate."));
    assert!(!record.contains("Profile benchmark hot paths"));
    assert!(fs::read_to_string(exported["path"].as_str().unwrap()).is_ok());

    // 9. Progression, still in the browser.
    let advanced = service.post(
        &format!("/api/v1/capabilities/next?project={project}"),
        serde_json::json!({}),
    );
    assert_eq!(advanced["capability"]["id"], "02_filter_files");

    service.idle(&project, "the service to settle");
    let _ = fs::remove_dir_all(project_path);
}

#[test]
fn creation_refuses_every_location_it_should() {
    let _guard = journey_guard();
    let service = start_service("paths");

    let refuse = |parent: Option<&str>, name: &str| -> String {
        let mut body = serde_json::json!({
            "pack": "flashindex",
            "language": "rust",
            "name": name,
        });
        if let Some(parent) = parent {
            body["parent_directory"] = serde_json::Value::String(parent.to_string());
        }
        let response = service.post("/api/v1/projects", body);
        response["error"]
            .as_str()
            .unwrap_or_else(|| panic!("expected a refusal, got {response}"))
            .to_string()
    };

    let workspace = service.workspace.to_str().unwrap().to_string();

    // Traversal, separators, and hidden names never reach the filesystem.
    for name in ["..", "../escape", "a/b", "a\\b", ".hidden", "", "   "] {
        let error = refuse(Some(&workspace), name);
        assert!(
            error.contains("project name") || error.contains("choose a name"),
            "{name:?} was refused for the wrong reason: {error}"
        );
    }

    // Reserved Windows device names are refused on every platform, so a project
    // created on one stays openable on another.
    assert!(refuse(Some(&workspace), "CON").contains("reserved"));

    // A system directory is outside the permitted root.
    let outside = if cfg!(windows) { "C:\\Windows" } else { "/etc" };
    assert!(refuse(Some(outside), "project").contains("home directory"));

    // A relative parent is not a location.
    assert!(refuse(Some("relative/path"), "project").contains("absolute"));

    // A parent that does not exist is not created on the learner's behalf.
    let missing = service.workspace.join("nope").to_str().unwrap().to_string();
    assert!(refuse(Some(&missing), "project").contains("not an existing directory"));

    // A real creation, then the two refusals that depend on it.
    let created = service.post(
        "/api/v1/projects",
        serde_json::json!({
            "pack": "flashindex",
            "language": "rust",
            "parent_directory": workspace,
            "name": "taken",
            "git": false,
        }),
    );
    let path = PathBuf::from(created["path"].as_str().unwrap());
    assert!(path.is_dir());

    // Creation never overwrites.
    assert!(refuse(Some(&workspace), "taken").contains("already exists"));

    // A project is never nested inside another project's source tree.
    let nested = path.join("src").to_str().unwrap().to_string();
    assert!(refuse(Some(&nested), "inner").contains("inside the DeltaForge project"));

    let _ = fs::remove_dir_all(path);
}

/// P0-1, from the browser's side.
///
/// The page prefills Location from `GET /api/v1/workspace` and posts a body
/// built by `DeltaForgeCore.preflightRequest`. This asserts both shapes that
/// body can take on a machine where the workspace has never existed: the null
/// this page now sends, and the prefilled path 1.0 sent — which was refused,
/// making the product's first screen unusable on every clean machine.
#[test]
fn a_machine_with_no_workspace_creates_a_project_from_the_defaults() {
    let _guard = journey_guard();
    let service = start_service_with("clean-machine", Workspace::Missing);
    assert!(
        !service.workspace.exists(),
        "this test is only meaningful while the workspace is absent"
    );

    let advertised = service.get("/api/v1/workspace")["default_directory"]
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(PathBuf::from(&advertised), service.workspace);

    let body = |parent: serde_json::Value| {
        serde_json::json!({
            "pack": "flashindex",
            "language": "rust",
            "parent_directory": parent,
            "name": "defaults",
        })
    };

    for parent in [
        serde_json::Value::Null,
        serde_json::Value::String(advertised.clone()),
    ] {
        let preflight = service.post("/api/v1/projects/preflight", body(parent.clone()));
        assert_eq!(
            preflight["ok"].as_bool(),
            Some(true),
            "preflight refused {parent}: {preflight}"
        );
        assert_eq!(
            preflight["location"]["creates_parent"].as_bool(),
            Some(true),
            "preflight did not say the workspace would be created: {preflight}"
        );
        assert!(
            !service.workspace.exists(),
            "a preflight must not create the directory it is only describing"
        );
    }

    let mut created_body = body(serde_json::Value::Null);
    created_body["git"] = serde_json::Value::Bool(false);
    let created = service.post("/api/v1/projects", created_body);
    let path = PathBuf::from(
        created["path"]
            .as_str()
            .unwrap_or_else(|| panic!("creation failed: {created}")),
    );
    assert!(path.is_dir());
    // Canonical on both sides: the service reports the resolved path, and on
    // macOS the temporary directory reaches it through a symlink.
    assert_eq!(
        path.parent().unwrap().canonicalize().unwrap(),
        service.workspace.canonicalize().unwrap(),
    );

    // The project is reachable from the browser immediately afterwards, which
    // is the whole point: catalog to a working project with no terminal.
    let project = created["project_id"].as_str().unwrap();
    let health = service.get(&format!("/api/v1/project-health?project={project}"));
    assert_eq!(health["status"].as_str(), Some("healthy"), "{health}");

    let _ = fs::remove_dir_all(path);
}

/// P1-5. One panic used to end the service.
///
/// Thread-per-connection over shared state, with fifteen
/// `.expect("workbench lock poisoned")` calls and no `catch_unwind` anywhere.
/// A handler that panicked while holding a mutex poisoned it for the life of
/// the process, so every subsequent request panicked too — and the browser
/// hung, because nothing ever answered.
#[test]
fn a_panicking_handler_does_not_take_the_workbench_with_it() {
    let _guard = journey_guard();
    let service = start_service_with_panic_probe("panic");

    // Healthy before.
    assert_eq!(status(&service.raw("GET", "/api/v1/catalog", "")), "200");

    let response = service.raw("POST", "/api/v1/__panic", "{}");
    assert_eq!(
        status(&response),
        "500",
        "a panicking handler must answer, not hang: {response}"
    );

    // And healthy after — twice, because the first request after a poisoned
    // lock is the one that used to fail, and every one after it.
    for attempt in 0..2 {
        assert_eq!(
            status(&service.raw("GET", "/api/v1/catalog", "")),
            "200",
            "request {attempt} after the panic was refused"
        );
    }
    // Including the routes that take the mutex the panicking handler held.
    assert_eq!(status(&service.raw("GET", "/api/v1/projects", "")), "200");

    // The panic left a trace somewhere a learner can be asked to look. The
    // service is detached from any terminal, so stderr alone is no record.
    let log = service.home.join("panic.log");
    let recorded = fs::read_to_string(&log)
        .unwrap_or_else(|error| panic!("no panic log at {}: {error}", log.display()));
    assert!(
        recorded.contains("DeltaForge panic probe"),
        "the panic log does not describe the panic: {recorded}"
    );
}

#[test]
fn a_gate_is_visible_before_the_learner_reaches_it() {
    let _guard = journey_guard();
    let service = start_service("gates");

    let created = service.post(
        "/api/v1/projects",
        serde_json::json!({
            "pack": "flashindex",
            "language": "rust",
            "parent_directory": service.workspace.to_str().unwrap(),
            "name": "gate-visibility",
            "git": false,
        }),
    );
    let project = created["project_id"].as_str().unwrap().to_string();
    let path = PathBuf::from(created["path"].as_str().unwrap());

    // On the first step, before anything has run, the browser already knows
    // which later steps carry a measurement. Before this existed, a gate could
    // only ever appear as a wall at the moment progression was refused.
    let state = service.get(&format!("/api/v1/state?project={project}"));
    let roadmap = state["performance"]["roadmap"].as_array().unwrap();
    let measured: Vec<&str> = roadmap
        .iter()
        .filter(|marker| marker["has_benchmarks"] == true)
        .map(|marker| marker["stage_id"].as_str().unwrap())
        .collect();
    assert_eq!(
        measured,
        [
            "01_scan_files",
            "03_tokenize",
            "06_canonical_index",
            "12_parallel_performance"
        ]
    );
    let gated = roadmap
        .iter()
        .find(|marker| marker["stage_id"] == "12_parallel_performance")
        .unwrap();
    assert_eq!(gated["status"], "not_measured");

    // The current step's own gate view is present and empty of gates: step 01
    // is measured but not gated.
    assert_eq!(state["performance"]["has_benchmarks"], true);
    assert!(state["performance"]["gate_status"].is_null());
    assert_eq!(state["performance"]["gate_blocks_progress"], false);

    let _ = fs::remove_dir_all(path);
}
