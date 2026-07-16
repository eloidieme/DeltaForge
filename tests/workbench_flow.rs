use std::fs;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

static PROJECT_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn deltaforge_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_deltaforge"))
}

fn temp_project_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "deltaforge-workbench-it-{}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        PROJECT_SEQUENCE.fetch_add(1, Ordering::Relaxed),
    ))
}

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn request(port: u16, method: &str, path: &str, origin: Option<&str>, body: &str) -> String {
    let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    let mut headers =
        format!("{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n");
    if let Some(origin) = origin {
        headers.push_str(&format!("Origin: {origin}\r\n"));
    }
    if method == "POST" {
        headers.push_str(&format!(
            "Content-Type: application/json\r\nContent-Length: {}\r\n",
            body.len()
        ));
    }
    headers.push_str("\r\n");
    stream.write_all(headers.as_bytes()).unwrap();
    stream.write_all(body.as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response
}

fn response_json(response: &str) -> serde_json::Value {
    let (_, body) = response.split_once("\r\n\r\n").unwrap();
    serde_json::from_str(body).unwrap()
}

fn wait_for_record(project: &Path) -> serde_json::Value {
    let path = project.join(".deltaforge/workbench.json");
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if let Ok(source) = fs::read_to_string(&path)
            && let Ok(record) = serde_json::from_str(&source)
        {
            return record;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    panic!("workbench service record was not created");
}

fn wait_for_new_record(project: &Path, previous_pid: u64) -> serde_json::Value {
    let path = project.join(".deltaforge/workbench.json");
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if let Ok(source) = fs::read_to_string(&path)
            && let Ok(record) = serde_json::from_str::<serde_json::Value>(&source)
            && record["pid"]
                .as_u64()
                .is_some_and(|pid| pid != previous_pid)
        {
            return record;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    panic!("replacement workbench service record was not created");
}

fn source_event_count(project: &Path) -> usize {
    let source = fs::read_to_string(project.join(".deltaforge/workbench-events.json")).unwrap();
    let journal: serde_json::Value = serde_json::from_str(&source).unwrap();
    journal["events"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| entry["event"]["type"] == "source_changed")
        .count()
}

fn wait_for_source_revision(port: u16, token: &str, revision: u64) -> serde_json::Value {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let state = response_json(&request(
            port,
            "GET",
            &format!("/api/v1/state?token={token}"),
            None,
            "",
        ));
        if state["source_revision"].as_u64() == Some(revision) {
            return state;
        }
        assert!(
            Instant::now() < deadline,
            "source revision {revision} was not observed"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn wait_for_health(port: u16, token: &str, status: &str) -> serde_json::Value {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let response = request(
            port,
            "GET",
            &format!("/api/v1/project-health?token={token}"),
            None,
            "",
        );
        if response.starts_with("HTTP/1.1 200") {
            let health = response_json(&response);
            if health["status"] == status {
                return health;
            }
        }
        assert!(
            Instant::now() < deadline,
            "project health never became {status}"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn read_run_events(port: u16, token: &str, cursor: u64, needle: &str) -> String {
    let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    let request = format!(
        "GET /api/v1/events?token={token}&after={cursor} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: keep-alive\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    let mut received = String::new();
    let mut chunk = [0_u8; 8192];
    while Instant::now() < deadline && !received.contains(needle) {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => received.push_str(&String::from_utf8_lossy(&chunk[..read])),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(error) => panic!("failed to read event stream: {error}"),
        }
    }
    received
}

fn passing_scan_source() -> &'static str {
    r#"use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 || args[0] != "scan" {
        return ExitCode::FAILURE;
    }
    match scan(Path::new(&args[1])) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => { eprintln!("{error}"); ExitCode::FAILURE }
    }
}

fn scan(root: &Path) -> Result<(), String> {
    let mut files = Vec::new();
    visit(root, root, &mut files)?;
    files.sort();
    for file in files {
        println!("{}", file.components().map(|part| part.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/"));
    }
    Ok(())
}

fn visit(root: &Path, current: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        let path = entry.path();
        if kind.is_dir() {
            let name = entry.file_name();
            if matches!(name.to_string_lossy().as_ref(), ".git" | "target" | "build" | "node_modules") { continue; }
            visit(root, &path, files)?;
        } else if kind.is_file() {
            files.push(path.strip_prefix(root).map_err(|error| error.to_string())?.to_path_buf());
        }
    }
    Ok(())
}
"#
}

#[test]
fn cli_run_reaches_the_open_workbench_event_stream() {
    let project = temp_project_path();
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );

    let token = "integration-token";
    let service = Command::new(deltaforge_bin())
        .args([
            "--project-dir",
            project.to_str().unwrap(),
            "__workbench",
            "--token",
            token,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _service = ChildGuard(service);
    let record = wait_for_record(&project);
    let port = record["port"].as_u64().unwrap() as u16;

    let state_response = request(
        port,
        "GET",
        &format!("/api/v1/state?token={token}"),
        None,
        "",
    );
    assert!(state_response.starts_with("HTTP/1.1 200"));
    let cursor = response_json(&state_response)["event_cursor"]
        .as_u64()
        .unwrap();
    let capability = response_json(&request(
        port,
        "GET",
        &format!("/api/v1/capability?token={token}"),
        None,
        "",
    ));
    assert_eq!(capability["stage_id"], "01_scan_files");
    assert_eq!(capability["help_levels"], 5);
    assert!(
        capability["mission"]
            .as_str()
            .unwrap()
            .contains("deterministic")
    );
    assert!(capability["requirements"].as_array().unwrap().len() >= 4);
    assert_eq!(capability["next"]["id"], "02_filter_files");

    let reader = std::thread::spawn({
        let token = token.to_string();
        move || read_run_events(port, &token, cursor, "\"type\":\"run_completed\"")
    });
    let run = Command::new(deltaforge_bin())
        .arg("test")
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(
        !run.status.success(),
        "the starter implementation should fail"
    );

    let events = reader.join().unwrap();
    assert!(events.starts_with("HTTP/1.1 200"), "{events}");
    assert!(events.contains("event: run"), "{events}");
    assert!(events.contains("\"trigger\":\"cli\""), "{events}");
    let build_started = events.find("\"type\":\"build_started\"").unwrap();
    let build_output = events.find("\"type\":\"build_output\"").unwrap();
    let build_completed = events.find("\"type\":\"build_completed\"").unwrap();
    assert!(build_started < build_output && build_output < build_completed);
    assert!(events.contains("\"type\":\"test_failed\""), "{events}");
    assert!(events.contains("\"type\":\"run_completed\""), "{events}");

    let final_state = response_json(&request(
        port,
        "GET",
        &format!("/api/v1/state?token={token}"),
        None,
        "",
    ));
    assert!(final_state["active_job"].is_null());
    assert_eq!(final_state["latest_run"]["failed"].as_u64(), Some(9));
    assert_eq!(
        final_state["primary_failure"]["name"],
        "scans files in a basic project"
    );
    assert_eq!(
        final_state["primary_failure"]["diagnosis"]["headline"],
        "Required project files are missing"
    );
    assert_eq!(final_state["primary_failure"]["diagnosis"]["priority"], 10);
    assert!(
        final_state["primary_failure"]["diagnosis"]["expected"]
            .as_str()
            .is_some_and(|expected| !expected.is_empty())
    );
    assert!(
        final_state["primary_failure"]["diagnosis"]["fixture_entries"]
            .as_array()
            .is_some_and(|entries| !entries.is_empty())
    );

    let origin = format!("http://127.0.0.1:{port}");
    for level in 1..=4 {
        let revealed = request(
            port,
            "POST",
            &format!("/api/v1/hints?token={token}"),
            Some(&origin),
            "{}",
        );
        assert!(revealed.starts_with("HTTP/1.1 200"), "{revealed}");
        let revealed = response_json(&revealed);
        assert_eq!(revealed["revealed_help"].as_array().unwrap().len(), level);
    }
    let retrospective = request(
        port,
        "POST",
        &format!("/api/v1/hints?token={token}"),
        Some(&origin),
        "{}",
    );
    assert!(retrospective.starts_with("HTTP/1.1 409"), "{retrospective}");

    let focused_cursor = final_state["event_cursor"].as_u64().unwrap();
    let focused_reader = std::thread::spawn({
        let token = token.to_string();
        move || read_run_events(port, &token, focused_cursor, "\"type\":\"run_completed\"")
    });
    let focused = request(
        port,
        "POST",
        &format!("/api/v1/runs/rerun?token={token}"),
        Some(&origin),
        r#"{"test":"scans files in a basic project"}"#,
    );
    assert!(focused.starts_with("HTTP/1.1 202"), "{focused}");
    let focused_events = focused_reader.join().unwrap();
    assert!(focused_events.contains("\"trigger\":\"workbench\""));
    assert!(focused_events.contains("\"total\":1"));
    assert!(focused_events.contains("\"type\":\"run_completed\""));
    let after_focused = response_json(&request(
        port,
        "GET",
        &format!("/api/v1/state?token={token}"),
        None,
        "",
    ));
    assert_eq!(after_focused["latest_run"]["failed"].as_u64(), Some(9));

    let secondary_name = final_state["latest_run"]["failed_tests"][1]["name"]
        .as_str()
        .unwrap();
    let secondary_cursor = after_focused["event_cursor"].as_u64().unwrap();
    let secondary_reader = std::thread::spawn({
        let token = token.to_string();
        move || read_run_events(port, &token, secondary_cursor, "\"type\":\"run_completed\"")
    });
    let secondary_body = serde_json::json!({"test": secondary_name}).to_string();
    let secondary = request(
        port,
        "POST",
        &format!("/api/v1/runs/rerun?token={token}"),
        Some(&origin),
        &secondary_body,
    );
    assert!(secondary.starts_with("HTTP/1.1 202"), "{secondary}");
    let secondary_events = secondary_reader.join().unwrap();
    assert!(secondary_events.contains(&format!("\"name\":{secondary_name:?}")));
    assert!(secondary_events.contains("\"total\":1"));
    assert!(secondary_events.contains("\"type\":\"run_completed\""));

    let browser_run = request(
        port,
        "POST",
        &format!("/api/v1/runs?token={token}"),
        Some(&origin),
        "{}",
    );
    assert!(browser_run.starts_with("HTTP/1.1 202"), "{browser_run}");
    let active_deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let state = response_json(&request(
            port,
            "GET",
            &format!("/api/v1/state?token={token}"),
            None,
            "",
        ));
        if !state["active_job"].is_null() {
            break;
        }
        assert!(
            Instant::now() < active_deadline,
            "browser run never became active"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    let cancel = request(
        port,
        "POST",
        &format!("/api/v1/runs/cancel?token={token}"),
        Some(&origin),
        "{}",
    );
    assert!(cancel.starts_with("HTTP/1.1 202"), "{cancel}");
    let cancel_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let state = response_json(&request(
            port,
            "GET",
            &format!("/api/v1/state?token={token}"),
            None,
            "",
        ));
        if state["active_job"].is_null() && state["latest_attempt"]["status"] == "cancelled" {
            break;
        }
        assert!(
            Instant::now() < cancel_deadline,
            "browser run was not cancelled"
        );
        std::thread::sleep(Duration::from_millis(20));
    }

    fs::write(project.join("src/main.rs"), "fn main( {\n").unwrap();
    let broken_build = Command::new(deltaforge_bin())
        .arg("test")
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(!broken_build.status.success());
    let unhealthy = response_json(&request(
        port,
        "GET",
        &format!("/api/v1/state?token={token}"),
        None,
        "",
    ));
    assert_eq!(unhealthy["primary_failure"]["diagnosis"]["kind"], "build");
    assert_eq!(
        unhealthy["primary_failure"]["diagnosis"]["headline"],
        "The project did not build"
    );
    assert_eq!(unhealthy["primary_action"]["kind"], "run_checks");

    drop(_service);
    let _ = fs::remove_dir_all(project);
}

#[test]
fn passing_run_unlocks_retrospective_and_browser_progression() {
    let project = temp_project_path();
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );
    fs::write(project.join("src/main.rs"), passing_scan_source()).unwrap();

    let token = "progression-token";
    let service = Command::new(deltaforge_bin())
        .args([
            "--project-dir",
            project.to_str().unwrap(),
            "__workbench",
            "--token",
            token,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _service = ChildGuard(service);
    let record = wait_for_record(&project);
    let first_pid = record["pid"].as_u64().unwrap();
    let port = record["port"].as_u64().unwrap() as u16;
    let origin = format!("http://127.0.0.1:{port}");

    let run = Command::new(deltaforge_bin())
        .arg("test")
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );

    let acquired = response_json(&request(
        port,
        "GET",
        &format!("/api/v1/state?token={token}"),
        None,
        "",
    ));
    assert_eq!(acquired["capability"]["completed"], true);
    assert_eq!(acquired["primary_action"]["kind"], "begin_next_capability");
    assert_eq!(acquired["primary_action"]["enabled"], true);

    for level in 1..=5 {
        let revealed = request(
            port,
            "POST",
            &format!("/api/v1/hints?token={token}"),
            Some(&origin),
            "{}",
        );
        assert!(revealed.starts_with("HTTP/1.1 200"), "{revealed}");
        assert_eq!(
            response_json(&revealed)["revealed_help"]
                .as_array()
                .unwrap()
                .len(),
            level
        );
    }

    let next = request(
        port,
        "POST",
        &format!("/api/v1/capabilities/next?token={token}"),
        Some(&origin),
        "{}",
    );
    assert!(next.starts_with("HTTP/1.1 200"), "{next}");
    let next = response_json(&next);
    assert_eq!(next["capability"]["id"], "02_filter_files");
    assert_eq!(next["primary_action"]["kind"], "run_checks");
    assert_eq!(next["freshness"], "never_run");

    let content = response_json(&request(
        port,
        "GET",
        &format!("/api/v1/capability?token={token}"),
        None,
        "",
    ));
    assert_eq!(content["stage_id"], "02_filter_files");

    drop(_service);
    let return_token = "progression-return-token";
    let return_service = ChildGuard(
        Command::new(deltaforge_bin())
            .args([
                "--project-dir",
                project.to_str().unwrap(),
                "__workbench",
                "--token",
                return_token,
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let return_record = wait_for_new_record(&project, first_pid);
    let return_port = return_record["port"].as_u64().unwrap() as u16;
    let returned = response_json(&request(
        return_port,
        "GET",
        &format!("/api/v1/state?token={return_token}"),
        None,
        "",
    ));
    assert_eq!(returned["resumption"]["kind"], "capability_changed");
    assert_eq!(
        returned["resumption"]["stage_change"]["from_id"],
        "01_scan_files"
    );
    assert_eq!(
        returned["resumption"]["stage_change"]["to_id"],
        "02_filter_files"
    );
    assert_eq!(returned["primary_action"]["kind"], "resume_checks");

    drop(return_service);
    let _ = fs::remove_dir_all(project);
}

#[test]
fn source_changes_are_durable_filtered_and_recovered_after_restart() {
    let project = temp_project_path();
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );
    let source_path = project.join("src/main.rs");
    fs::write(&source_path, passing_scan_source()).unwrap();

    let first_token = "freshness-token-one";
    let first_service = Command::new(deltaforge_bin())
        .args([
            "--project-dir",
            project.to_str().unwrap(),
            "__workbench",
            "--token",
            first_token,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let first_service = ChildGuard(first_service);
    let first_record = wait_for_record(&project);
    let first_pid = first_record["pid"].as_u64().unwrap();
    let first_port = first_record["port"].as_u64().unwrap() as u16;

    let run = Command::new(deltaforge_bin())
        .arg("test")
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let fresh = response_json(&request(
        first_port,
        "GET",
        &format!("/api/v1/state?token={first_token}"),
        None,
        "",
    ));
    assert_eq!(fresh["freshness"], "fresh");
    assert_eq!(fresh["source_revision"], 0);
    let cursor = fresh["event_cursor"].as_u64().unwrap();

    let reader = std::thread::spawn({
        let token = first_token.to_string();
        move || read_run_events(first_port, &token, cursor, "\"type\":\"source_changed\"")
    });
    let mut source = fs::read_to_string(&source_path).unwrap();
    source.push_str("\n// relevant edit\n");
    fs::write(&source_path, source).unwrap();
    let source_events = reader.join().unwrap();
    assert!(source_events.contains("\"type\":\"source_changed\""));
    assert!(source_events.contains("\"revision\":1"));

    let stale = wait_for_source_revision(first_port, first_token, 1);
    assert_eq!(stale["freshness"], "stale");
    assert_eq!(stale["primary_action"]["kind"], "run_checks");
    assert_eq!(stale["last_source_change"]["revision"], 1);
    assert_ne!(
        stale["last_source_change"]["previous_digest"],
        stale["last_source_change"]["current_digest"]
    );
    assert_eq!(source_event_count(&project), 1);
    let cursor_after_source = stale["event_cursor"].as_u64().unwrap();

    fs::create_dir_all(project.join("target")).unwrap();
    fs::write(project.join("target/watcher-noise.txt"), "ignored").unwrap();
    std::thread::sleep(Duration::from_millis(1_200));
    let after_ignored = response_json(&request(
        first_port,
        "GET",
        &format!("/api/v1/state?token={first_token}"),
        None,
        "",
    ));
    assert_eq!(after_ignored["source_revision"], 1);
    assert_eq!(after_ignored["event_cursor"], cursor_after_source);
    assert_eq!(source_event_count(&project), 1);

    drop(first_service);
    let mut source = fs::read_to_string(&source_path).unwrap();
    source.push_str("// edit while the service is stopped\n");
    fs::write(&source_path, source).unwrap();

    let second_token = "freshness-token-two";
    let second_service = Command::new(deltaforge_bin())
        .args([
            "--project-dir",
            project.to_str().unwrap(),
            "__workbench",
            "--token",
            second_token,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let second_service = ChildGuard(second_service);
    let second_record = wait_for_new_record(&project, first_pid);
    let second_port = second_record["port"].as_u64().unwrap() as u16;
    let resumed = wait_for_source_revision(second_port, second_token, 2);
    assert_eq!(resumed["freshness"], "stale");
    assert_eq!(resumed["resumption"]["kind"], "source_changed");
    assert_eq!(resumed["primary_action"]["kind"], "resume_checks");
    assert_eq!(resumed["last_source_change"]["revision"], 2);
    assert_eq!(source_event_count(&project), 2);

    std::thread::sleep(Duration::from_millis(1_200));
    let stable = wait_for_source_revision(second_port, second_token, 2);
    assert_eq!(stable["source_revision"], 2);
    assert_eq!(source_event_count(&project), 2);

    for expected_revision in 3..=4 {
        let mut source = fs::read_to_string(&source_path).unwrap();
        source.push_str(&format!("// successive edit {expected_revision}\n"));
        fs::write(&source_path, source).unwrap();
        let changed = wait_for_source_revision(second_port, second_token, expected_revision);
        assert_eq!(changed["last_source_change"]["revision"], expected_revision);
    }
    assert_eq!(source_event_count(&project), 4);

    drop(second_service);
    let _ = fs::remove_dir_all(project);
}

#[test]
fn unhealthy_project_still_opens_and_supports_bounded_recovery() {
    let project = temp_project_path();
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );
    let config_path = project.join(".deltaforge/config.toml");
    let valid_config = fs::read_to_string(&config_path).unwrap();
    fs::write(&config_path, "[runner]\ntimeout_ms = 0\n").unwrap();

    let token = "unhealthy-token";
    let service = Command::new(deltaforge_bin())
        .args([
            "--project-dir",
            project.to_str().unwrap(),
            "__workbench",
            "--token",
            token,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let service = ChildGuard(service);
    let record = wait_for_record(&project);
    let port = record["port"].as_u64().unwrap() as u16;
    let origin = format!("http://127.0.0.1:{port}");

    let health = wait_for_health(port, token, "unhealthy");
    assert_eq!(health["issue"]["code"], "configuration_invalid");
    assert_eq!(health["actions"][0]["kind"], "recheck");
    let shell = request(port, "GET", &format!("/?token={token}"), None, "");
    assert!(shell.starts_with("HTTP/1.1 200"));
    assert!(shell.contains("id=\"health-screen\""));

    let injected_path = request(
        port,
        "POST",
        &format!("/api/v1/project/open-folder?token={token}"),
        Some(&origin),
        r#"{"path":"/tmp"}"#,
    );
    assert!(injected_path.starts_with("HTTP/1.1 400"), "{injected_path}");

    fs::write(&config_path, valid_config).unwrap();
    let healthy = wait_for_health(port, token, "healthy");
    assert!(healthy["issue"].is_null());

    let state_path = project.join(".deltaforge/state.json");
    let valid_state = fs::read_to_string(&state_path).unwrap();
    fs::write(&state_path, "{ invalid json").unwrap();
    let invalid_state = wait_for_health(port, token, "unhealthy");
    assert_eq!(invalid_state["issue"]["code"], "state_invalid");
    fs::write(&state_path, &valid_state).unwrap();
    let restored = wait_for_health(port, token, "healthy");
    assert!(restored["issue"].is_null());

    let mut state: serde_json::Value = serde_json::from_str(&valid_state).unwrap();
    state["pack_digest"] = serde_json::Value::String("outdated-pack-digest".to_string());
    fs::write(&state_path, serde_json::to_vec_pretty(&state).unwrap()).unwrap();
    let changed_pack = wait_for_health(port, token, "unhealthy");
    assert_eq!(changed_pack["issue"]["code"], "pack_changed");
    assert!(
        changed_pack["actions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|action| action["kind"] == "repin_pack")
    );

    let recovered = request(
        port,
        "POST",
        &format!("/api/v1/project/repin-pack?token={token}"),
        Some(&origin),
        "{}",
    );
    assert!(recovered.starts_with("HTTP/1.1 200"), "{recovered}");
    assert_eq!(response_json(&recovered)["status"], "healthy");
    let restored_state = request(
        port,
        "GET",
        &format!("/api/v1/state?token={token}"),
        None,
        "",
    );
    assert!(
        restored_state.starts_with("HTTP/1.1 200"),
        "{restored_state}"
    );

    drop(service);
    let _ = fs::remove_dir_all(project);
}

#[test]
fn stopped_service_restores_an_interrupted_run_without_rerunning_it() {
    let project = temp_project_path();
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );

    let first_token = "resume-token-one";
    let first_service = ChildGuard(
        Command::new(deltaforge_bin())
            .args([
                "--project-dir",
                project.to_str().unwrap(),
                "__workbench",
                "--token",
                first_token,
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let first_record = wait_for_record(&project);
    let first_pid = first_record["pid"].as_u64().unwrap();
    let first_port = first_record["port"].as_u64().unwrap() as u16;
    let first_origin = format!("http://127.0.0.1:{first_port}");
    let initial = response_json(&request(
        first_port,
        "GET",
        &format!("/api/v1/state?token={first_token}"),
        None,
        "",
    ));
    assert!(initial["resumption"].is_null());

    let started = request(
        first_port,
        "POST",
        &format!("/api/v1/runs?token={first_token}"),
        Some(&first_origin),
        "{}",
    );
    assert!(started.starts_with("HTTP/1.1 202"), "{started}");
    let deadline = Instant::now() + Duration::from_secs(5);
    let interrupted_job_id = loop {
        let state = response_json(&request(
            first_port,
            "GET",
            &format!("/api/v1/state?token={first_token}"),
            None,
            "",
        ));
        if let Some(job_id) = state["active_job"]["id"].as_str() {
            break job_id.to_string();
        }
        assert!(Instant::now() < deadline, "run never became active");
        std::thread::sleep(Duration::from_millis(10));
    };

    drop(first_service);

    let second_token = "resume-token-two";
    let second_service = ChildGuard(
        Command::new(deltaforge_bin())
            .args([
                "--project-dir",
                project.to_str().unwrap(),
                "__workbench",
                "--token",
                second_token,
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let second_record = wait_for_new_record(&project, first_pid);
    let second_port = second_record["port"].as_u64().unwrap() as u16;
    let second_origin = format!("http://127.0.0.1:{second_port}");
    let resumed = response_json(&request(
        second_port,
        "GET",
        &format!("/api/v1/state?token={second_token}"),
        None,
        "",
    ));
    assert!(resumed["active_job"].is_null());
    assert_eq!(resumed["latest_attempt"]["job_id"], interrupted_job_id);
    assert_eq!(resumed["latest_attempt"]["status"], "interrupted");
    assert_eq!(resumed["resumption"]["kind"], "interrupted");
    assert_eq!(resumed["resumption"]["action_pending"], true);
    assert_eq!(resumed["recovered_interrupted_job"], true);
    assert_eq!(resumed["primary_action"]["kind"], "resume_checks");
    let attempts_after_recovery = fs::read_to_string(project.join(".deltaforge/state.json"))
        .ok()
        .and_then(|source| serde_json::from_str::<serde_json::Value>(&source).ok())
        .and_then(|state| state["attempt_history"].as_array().map(Vec::len))
        .unwrap();

    std::thread::sleep(Duration::from_millis(750));
    let still_resumed = response_json(&request(
        second_port,
        "GET",
        &format!("/api/v1/state?token={second_token}"),
        None,
        "",
    ));
    assert_eq!(
        still_resumed["latest_attempt"]["job_id"],
        interrupted_job_id
    );
    assert_eq!(still_resumed["primary_action"]["kind"], "resume_checks");
    let persisted: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(project.join(".deltaforge/state.json")).unwrap())
            .unwrap();
    assert_eq!(
        persisted["attempt_history"].as_array().unwrap().len(),
        attempts_after_recovery,
        "restoring a session must not rerun checks"
    );

    let restarted = request(
        second_port,
        "POST",
        &format!("/api/v1/runs?token={second_token}"),
        Some(&second_origin),
        "{}",
    );
    assert!(restarted.starts_with("HTTP/1.1 202"), "{restarted}");
    let completion_deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let state = response_json(&request(
            second_port,
            "GET",
            &format!("/api/v1/state?token={second_token}"),
            None,
            "",
        ));
        if state["active_job"].is_null() && state["latest_attempt"]["job_id"] != interrupted_job_id
        {
            assert_eq!(state["resumption"]["action_pending"], false);
            assert_eq!(state["recovered_interrupted_job"], false);
            assert_eq!(state["primary_action"]["kind"], "run_checks");
            break;
        }
        assert!(
            Instant::now() < completion_deadline,
            "resumed check run did not finish"
        );
        std::thread::sleep(Duration::from_millis(25));
    }

    drop(second_service);
    let _ = fs::remove_dir_all(project);
}

#[test]
fn diagnostic_shutdown_is_authenticated_and_never_interrupts_a_run() {
    let project = temp_project_path();
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );

    let token = "shutdown-token";
    let mut service = ChildGuard(
        Command::new(deltaforge_bin())
            .args([
                "--project-dir",
                project.to_str().unwrap(),
                "__workbench",
                "--token",
                token,
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let record = wait_for_record(&project);
    let port = record["port"].as_u64().unwrap() as u16;
    let origin = format!("http://127.0.0.1:{port}");
    let shutdown_path = format!("/api/v1/service/shutdown?token={token}");

    let hostile = request(
        port,
        "POST",
        &shutdown_path,
        Some("http://attacker.invalid"),
        "{}",
    );
    assert!(hostile.starts_with("HTTP/1.1 403"), "{hostile}");

    let started = request(
        port,
        "POST",
        &format!("/api/v1/runs?token={token}"),
        Some(&origin),
        "{}",
    );
    assert!(started.starts_with("HTTP/1.1 202"), "{started}");
    let active_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let state = response_json(&request(
            port,
            "GET",
            &format!("/api/v1/state?token={token}"),
            None,
            "",
        ));
        if !state["active_job"].is_null() {
            break;
        }
        assert!(
            Instant::now() < active_deadline,
            "shutdown test run never became active"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    let blocked = request(port, "POST", &shutdown_path, Some(&origin), "{}");
    assert!(blocked.starts_with("HTTP/1.1 409"), "{blocked}");
    assert_eq!(response_json(&blocked)["error"], "run_active");

    let cancel = request(
        port,
        "POST",
        &format!("/api/v1/runs/cancel?token={token}"),
        Some(&origin),
        "{}",
    );
    assert!(cancel.starts_with("HTTP/1.1 202"), "{cancel}");
    let cancelled_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let state = response_json(&request(
            port,
            "GET",
            &format!("/api/v1/state?token={token}"),
            None,
            "",
        ));
        if state["active_job"].is_null() {
            break;
        }
        assert!(
            Instant::now() < cancelled_deadline,
            "shutdown test run was not cancelled"
        );
        std::thread::sleep(Duration::from_millis(20));
    }

    let stopped = request(port, "POST", &shutdown_path, Some(&origin), "{}");
    assert!(stopped.starts_with("HTTP/1.1 202"), "{stopped}");
    assert_eq!(response_json(&stopped)["status"], "stopping");
    let stopped_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if service.0.try_wait().unwrap().is_some() {
            break;
        }
        assert!(
            Instant::now() < stopped_deadline,
            "workbench service did not stop"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(!project.join(".deltaforge/workbench.json").exists());

    drop(service);
    let _ = fs::remove_dir_all(project);
}

#[test]
fn lifecycle_recovers_stale_metadata_and_replaces_an_incompatible_service() {
    let project = temp_project_path();
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(init.status.success());
    let record_path = project.join(".deltaforge/workbench.json");
    fs::write(&record_path, "{ stale lifecycle metadata").unwrap();

    let first_launch = Command::new(deltaforge_bin())
        .env("DELTAFORGE_NO_BROWSER", "1")
        .current_dir(&project)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let second_launch = Command::new(deltaforge_bin())
        .env("DELTAFORGE_NO_BROWSER", "1")
        .current_dir(&project)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let first_launch = first_launch.wait_with_output().unwrap();
    let second_launch = second_launch.wait_with_output().unwrap();
    assert!(
        first_launch.status.success(),
        "{}",
        String::from_utf8_lossy(&first_launch.stderr)
    );
    assert!(
        second_launch.status.success(),
        "{}",
        String::from_utf8_lossy(&second_launch.stderr)
    );
    assert_eq!(first_launch.stdout, second_launch.stdout);
    let first = wait_for_record(&project);
    let first_pid = first["pid"].as_u64().unwrap();
    let first_token = first["token"].as_str().unwrap().to_string();
    let first_port = first["port"].as_u64().unwrap() as u16;
    assert!(
        request(
            first_port,
            "GET",
            &format!("/api/v1/health?token={first_token}"),
            None,
            ""
        )
        .starts_with("HTTP/1.1 200")
    );

    let mut incompatible = first.clone();
    incompatible["version"] = serde_json::Value::String("incompatible-version".to_string());
    fs::write(
        &record_path,
        serde_json::to_vec_pretty(&incompatible).unwrap(),
    )
    .unwrap();
    let replacement_launch = Command::new(deltaforge_bin())
        .env("DELTAFORGE_NO_BROWSER", "1")
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(
        replacement_launch.status.success(),
        "{}",
        String::from_utf8_lossy(&replacement_launch.stderr)
    );
    let replacement = wait_for_new_record(&project, first_pid);
    let replacement_pid = replacement["pid"].as_u64().unwrap();
    let replacement_port = replacement["port"].as_u64().unwrap() as u16;
    let replacement_token = replacement["token"].as_str().unwrap();
    assert_ne!(replacement_pid, first_pid);
    assert_ne!(replacement_token, first_token);

    let origin = format!("http://127.0.0.1:{replacement_port}");
    let stopped = request(
        replacement_port,
        "POST",
        &format!("/api/v1/service/shutdown?token={replacement_token}"),
        Some(&origin),
        "{}",
    );
    assert!(stopped.starts_with("HTTP/1.1 202"), "{stopped}");
    let stop_deadline = Instant::now() + Duration::from_secs(5);
    while record_path.exists() {
        assert!(
            Instant::now() < stop_deadline,
            "replacement service metadata was not cleaned up"
        );
        std::thread::sleep(Duration::from_millis(20));
    }

    let _ = fs::remove_dir_all(project);
}

#[test]
fn idle_service_exits_and_removes_its_discovery_record() {
    let project = temp_project_path();
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(init.status.success());

    let mut service = ChildGuard(
        Command::new(deltaforge_bin())
            .args([
                "--project-dir",
                project.to_str().unwrap(),
                "__workbench",
                "--token",
                "idle-token",
                "--idle-timeout-ms",
                "150",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let _ = wait_for_record(&project);
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if service.0.try_wait().unwrap().is_some() {
            break;
        }
        assert!(Instant::now() < deadline, "idle service did not exit");
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(!project.join(".deltaforge/workbench.json").exists());

    drop(service);
    let _ = fs::remove_dir_all(project);
}

#[test]
fn bare_launch_focuses_a_connected_workbench_without_opening_another_tab() {
    let project = temp_project_path();
    let init = Command::new(deltaforge_bin())
        .args([
            "init",
            "flashindex",
            "--lang",
            "rust",
            "--name",
            project.to_str().unwrap(),
            "--no-git",
        ])
        .output()
        .unwrap();
    assert!(init.status.success());

    let token = "focus-token";
    let service = ChildGuard(
        Command::new(deltaforge_bin())
            .args([
                "--project-dir",
                project.to_str().unwrap(),
                "__workbench",
                "--token",
                token,
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap(),
    );
    let record = wait_for_record(&project);
    let pid = record["pid"].as_u64().unwrap();
    let port = record["port"].as_u64().unwrap() as u16;
    let reader = std::thread::spawn({
        let token = token.to_string();
        move || read_run_events(port, &token, 0, "event: focus")
    });
    let client_deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let health = response_json(&request(
            port,
            "GET",
            &format!("/api/v1/health?token={token}"),
            None,
            "",
        ));
        if health["clients"].as_u64().unwrap_or(0) > 0 {
            break;
        }
        assert!(
            Instant::now() < client_deadline,
            "event-stream client was not registered"
        );
        std::thread::sleep(Duration::from_millis(20));
    }

    let launch = Command::new(deltaforge_bin())
        .env("DELTAFORGE_NO_BROWSER", "1")
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(launch.status.success());
    let stdout = String::from_utf8_lossy(&launch.stdout);
    assert_eq!(stdout.trim(), "DeltaForge is ready.");
    let focus_events = reader.join().unwrap();
    assert!(focus_events.contains("event: focus"), "{focus_events}");
    let reused = wait_for_record(&project);
    assert_eq!(reused["pid"].as_u64(), Some(pid));

    drop(service);
    let _ = fs::remove_dir_all(project);
}
