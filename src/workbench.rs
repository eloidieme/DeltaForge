use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::application;
use crate::context::GlobalOptions;
use crate::fs_util::atomic_write;

const RECORD_FILE: &str = "workbench.json";
const START_LOCK_FILE: &str = "workbench-start.lock";
const API_VERSION: &str = "v1";
const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");
const PROBE_TIMEOUT: Duration = Duration::from_millis(500);
const STARTUP_TIMEOUT: Duration = Duration::from_secs(4);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const EVENT_POLL_INTERVAL: Duration = Duration::from_millis(500);
const IDLE_TIMEOUT: Duration = Duration::from_secs(30 * 60);
const MAX_HEADER_BYTES: usize = 32 * 1024;
const MAX_BODY_BYTES: usize = 8 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ServiceRecord {
    port: u16,
    pid: u32,
    token: String,
    version: String,
}

#[derive(Debug, Clone)]
struct ServiceStatus {
    version: String,
    pid: u32,
    clients: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StartupLeaseRecord {
    pid: u32,
}

struct StartupLease {
    path: PathBuf,
}

impl StartupLease {
    fn acquire(root: &Path) -> Result<Self> {
        let path = root.join(".deltaforge").join(START_LOCK_FILE);
        let deadline = Instant::now() + STARTUP_TIMEOUT;
        loop {
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    file.write_all(&serde_json::to_vec(&StartupLeaseRecord {
                        pid: std::process::id(),
                    })?)?;
                    file.sync_all()?;
                    return Ok(Self { path });
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    if !startup_lease_is_active(&path) {
                        continue;
                    }
                    if Instant::now() >= deadline {
                        bail!("another DeltaForge workbench launch is still starting");
                    }
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(error) => return Err(error.into()),
            }
        }
    }
}

impl Drop for StartupLease {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn startup_lease_is_active(path: &Path) -> bool {
    let record = fs::read(path)
        .ok()
        .and_then(|source| serde_json::from_slice::<StartupLeaseRecord>(&source).ok());
    match record {
        Some(record) if crate::run_lease::process_is_alive(record.pid) => true,
        Some(_) => {
            let _ = fs::remove_file(path);
            false
        }
        None => {
            let recent = path
                .metadata()
                .and_then(|metadata| metadata.modified())
                .ok()
                .and_then(|modified| modified.elapsed().ok())
                .is_some_and(|elapsed| elapsed < Duration::from_secs(1));
            if !recent {
                let _ = fs::remove_file(path);
            }
            recent
        }
    }
}

#[derive(Debug)]
struct Shared {
    options: GlobalOptions,
    token: String,
    session_id: String,
    port: u16,
    clients: AtomicUsize,
    last_activity: Mutex<Instant>,
    record_path: PathBuf,
    run_starting: Mutex<bool>,
    shutting_down: AtomicBool,
    idle_timeout: Duration,
    focus_revision: AtomicUsize,
}

#[derive(Debug)]
struct HttpRequest {
    method: String,
    target: String,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct StartRunBody {
    #[serde(default)]
    filter: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RerunBody {
    test: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct EmptyBody {}

#[derive(Debug, Clone, Copy)]
enum ProjectOpenKind {
    Editor,
    Folder,
}

pub fn launch(options: &GlobalOptions) -> Result<()> {
    let root = crate::context::locate_project_root(options)?;
    let startup_lease = StartupLease::acquire(&root)?;
    let record_path = root.join(".deltaforge").join(RECORD_FILE);
    let mut service = read_compatible_record(&record_path)?;

    if service.is_none() {
        let token = capability_token(&root);
        spawn_service(&root, options, &token)?;
        service = wait_for_service(&record_path)?;
    }

    let (record, status) = service.context("the DeltaForge workbench did not start in time")?;
    drop(startup_lease);
    let url = format!("http://127.0.0.1:{}/?token={}", record.port, record.token);
    if status.clients > 0 && request_focus(&record) {
        println!("DeltaForge is ready.");
        return Ok(());
    }
    if std::env::var_os("DELTAFORGE_NO_BROWSER").is_some() {
        println!("DeltaForge is ready at {url}");
        println!("You can run checks with: deltaforge test");
    } else {
        match open_in_browser(url.as_ref()) {
            Ok(()) => println!("DeltaForge is ready."),
            Err(error) => {
                println!("DeltaForge is ready at {url}");
                println!("Browser opening failed: {error:#}");
                println!("You can still run checks with: deltaforge test");
            }
        }
    }
    Ok(())
}

fn open_in_browser(target: &std::ffi::OsStr) -> Result<()> {
    let mut command = browser_command(target)?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
        .spawn()
        .with_context(|| format!("failed to open {} in a browser", target.to_string_lossy()))?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn browser_command(target: &std::ffi::OsStr) -> Result<Command> {
    let mut command = Command::new("open");
    command.arg(target);
    Ok(command)
}

#[cfg(target_os = "linux")]
fn browser_command(target: &std::ffi::OsStr) -> Result<Command> {
    let mut command = Command::new("xdg-open");
    command.arg(target);
    Ok(command)
}

#[cfg(windows)]
fn browser_command(target: &std::ffi::OsStr) -> Result<Command> {
    let mut command = Command::new("rundll32");
    command.arg("url.dll,FileProtocolHandler").arg(target);
    Ok(command)
}

#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
fn browser_command(_target: &std::ffi::OsStr) -> Result<Command> {
    bail!("opening a browser is not supported on this operating system")
}

fn spawn_service(root: &Path, options: &GlobalOptions, token: &str) -> Result<()> {
    let executable = std::env::current_exe().context("failed to locate the deltaforge binary")?;
    let mut command = Command::new(executable);
    command
        .arg("--project-dir")
        .arg(root)
        .arg("__workbench")
        .arg("--token")
        .arg(token)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if let Some(packs_dir) = &options.packs_dir {
        command.arg("--packs-dir").arg(packs_dir);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
        .spawn()
        .context("failed to start the DeltaForge workbench service")?;
    Ok(())
}

fn wait_for_service(record_path: &Path) -> Result<Option<(ServiceRecord, ServiceStatus)>> {
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    while Instant::now() < deadline {
        if let Some(record) = read_compatible_record(record_path)? {
            return Ok(Some(record));
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(None)
}

fn read_compatible_record(record_path: &Path) -> Result<Option<(ServiceRecord, ServiceStatus)>> {
    let source = match fs::read_to_string(record_path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let mut record: ServiceRecord = match serde_json::from_str(&source) {
        Ok(record) => record,
        Err(_) => {
            let _ = fs::remove_file(record_path);
            return Ok(None);
        }
    };
    let Some(status) = probe(&record) else {
        remove_record_if_matches(record_path, &record);
        return Ok(None);
    };
    if record.version != SERVICE_VERSION || status.version != SERVICE_VERSION {
        replace_incompatible_service(record_path, &record)?;
        return Ok(None);
    }
    if record.pid != status.pid {
        record.pid = status.pid;
        atomic_write(record_path, serde_json::to_string(&record)?)?;
    }
    Ok(Some((record, status)))
}

fn probe(record: &ServiceRecord) -> Option<ServiceStatus> {
    let path = format!("/api/{API_VERSION}/health?token={}", record.token);
    let body = http_get(record.port, &path)?;
    let value = serde_json::from_str::<serde_json::Value>(&body).ok()?;
    (value.get("service")?.as_str()? == "deltaforge").then_some(ServiceStatus {
        version: value.get("version")?.as_str()?.to_string(),
        pid: u32::try_from(value.get("pid")?.as_u64()?).ok()?,
        clients: usize::try_from(
            value
                .get("clients")
                .and_then(|value| value.as_u64())
                .unwrap_or(0),
        )
        .ok()?,
    })
}

fn replace_incompatible_service(record_path: &Path, record: &ServiceRecord) -> Result<()> {
    if !request_shutdown(record) {
        bail!(
            "an incompatible DeltaForge workbench is still active; finish or cancel its check run, then launch DeltaForge again"
        );
    }
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    while Instant::now() < deadline {
        if probe(record).is_none() {
            remove_record_if_matches(record_path, record);
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    bail!("the incompatible DeltaForge workbench did not stop in time")
}

fn remove_record_if_matches(record_path: &Path, expected: &ServiceRecord) {
    let current = fs::read_to_string(record_path)
        .ok()
        .and_then(|source| serde_json::from_str::<ServiceRecord>(&source).ok());
    if current.as_ref().is_some_and(|record| {
        record.port == expected.port && record.pid == expected.pid && record.token == expected.token
    }) {
        let _ = fs::remove_file(record_path);
    }
}

fn request_focus(record: &ServiceRecord) -> bool {
    let path = format!("/api/{API_VERSION}/focus?token={}", record.token);
    http_get_response(record.port, &path)
        .is_some_and(|response| response.starts_with("HTTP/1.1 202"))
}

fn request_shutdown(record: &ServiceRecord) -> bool {
    let path = format!("/api/{API_VERSION}/service/shutdown?token={}", record.token);
    let origin = format!("http://127.0.0.1:{}", record.port);
    let request = format!(
        "POST {path} HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nOrigin: {origin}\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}",
        record.port
    );
    http_exchange(record.port, &request)
        .is_some_and(|response| response.starts_with("HTTP/1.1 202"))
}

pub fn serve(options: &GlobalOptions, token: String, idle_timeout: Option<Duration>) -> Result<()> {
    let idle_timeout = idle_timeout.unwrap_or(IDLE_TIMEOUT);
    if idle_timeout.is_zero() {
        bail!("workbench idle timeout must be greater than zero");
    }
    let root = crate::context::locate_project_root(options)?;
    let session_id = format!(
        "{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let _ = application::load_workbench_state_for_session(options, &session_id);
    let _ = application::observe_source_changes(options);
    let record_path = root.join(".deltaforge").join(RECORD_FILE);
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
        .context("failed to bind the DeltaForge workbench to loopback")?;
    let port = listener.local_addr()?.port();
    let record = ServiceRecord {
        port,
        pid: std::process::id(),
        token: token.clone(),
        version: SERVICE_VERSION.to_string(),
    };
    atomic_write(&record_path, serde_json::to_string(&record)?)?;

    let shared = Arc::new(Shared {
        options: GlobalOptions {
            project_dir: Some(root),
            packs_dir: options.packs_dir.clone(),
        },
        token,
        session_id,
        port,
        clients: AtomicUsize::new(0),
        last_activity: Mutex::new(Instant::now()),
        record_path,
        run_starting: Mutex::new(false),
        shutting_down: AtomicBool::new(false),
        idle_timeout,
        focus_revision: AtomicUsize::new(0),
    });
    spawn_idle_watchdog(Arc::clone(&shared));
    spawn_source_watcher(Arc::clone(&shared));

    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            continue;
        };
        let shared = Arc::clone(&shared);
        std::thread::spawn(move || {
            let _ = handle_connection(stream, &shared);
        });
    }
    Ok(())
}

fn spawn_source_watcher(shared: Arc<Shared>) {
    std::thread::spawn(move || {
        loop {
            if application::observe_source_changes(&shared.options)
                .ok()
                .flatten()
                .is_some()
            {
                *shared
                    .last_activity
                    .lock()
                    .expect("workbench lock poisoned") = Instant::now();
            }
            std::thread::sleep(EVENT_POLL_INTERVAL);
        }
    });
}

fn spawn_idle_watchdog(shared: Arc<Shared>) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(
                shared
                    .idle_timeout
                    .min(Duration::from_secs(30))
                    .max(Duration::from_millis(10)),
            );
            let idle = shared
                .last_activity
                .lock()
                .map(|last| last.elapsed())
                .unwrap_or_default();
            let run_starting = *shared.run_starting.lock().expect("workbench lock poisoned");
            let run_active =
                run_starting || application::run_is_active(&shared.options).unwrap_or(false);
            if shared.clients.load(Ordering::SeqCst) == 0
                && !run_active
                && idle >= shared.idle_timeout
            {
                let _ = fs::remove_file(&shared.record_path);
                std::process::exit(0);
            }
        }
    });
}
fn handle_connection(mut stream: TcpStream, shared: &Arc<Shared>) -> Result<()> {
    stream.set_read_timeout(Some(REQUEST_TIMEOUT))?;
    let Some(request) = read_request(&mut stream)? else {
        return Ok(());
    };
    *shared
        .last_activity
        .lock()
        .expect("workbench lock poisoned") = Instant::now();

    if !authorized(&request, shared) {
        return respond(
            &mut stream,
            "403 Forbidden",
            "application/json",
            r#"{"error":"forbidden"}"#,
        );
    }

    let path = request
        .target
        .split_once('?')
        .map_or(request.target.as_str(), |(path, _)| path);
    if request.method != "GET" && request.method != "POST" {
        return respond(
            &mut stream,
            "405 Method Not Allowed",
            "application/json",
            r#"{"error":"method_not_allowed"}"#,
        );
    }

    match (request.method.as_str(), path) {
        ("GET", "/") => respond(
            &mut stream,
            "200 OK",
            "text/html; charset=utf-8",
            &workbench_html(&shared.token),
        ),
        ("GET", "/api/v1/health") => {
            let body = serde_json::json!({
                "service": "deltaforge",
                "api": API_VERSION,
                "version": SERVICE_VERSION,
                "pid": std::process::id(),
                "clients": shared.clients.load(Ordering::SeqCst),
            })
            .to_string();
            respond(&mut stream, "200 OK", "application/json", &body)
        }
        ("GET", "/api/v1/project-health") => {
            let health = application::load_project_health(&shared.options)?;
            respond(
                &mut stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&health)?,
            )
        }
        ("GET", "/api/v1/focus") => {
            shared.focus_revision.fetch_add(1, Ordering::SeqCst);
            respond(
                &mut stream,
                "202 Accepted",
                "application/json",
                r#"{"status":"focus_requested"}"#,
            )
        }
        ("GET", "/api/v1/state") => {
            let state =
                application::load_workbench_state_for_session(&shared.options, &shared.session_id)?;
            let body = serde_json::to_string(&state)?;
            respond(&mut stream, "200 OK", "application/json", &body)
        }
        ("GET", "/api/v1/capability") => {
            let content = application::load_capability_content(&shared.options)?;
            let body = serde_json::to_string(&content)?;
            respond(&mut stream, "200 OK", "application/json", &body)
        }
        ("GET", "/api/v1/events") => serve_events(stream, shared, &request),
        ("POST", "/api/v1/runs") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: StartRunBody = match parse_json_body(&request) {
                Ok(body) => body,
                Err(_) => {
                    return respond(
                        &mut stream,
                        "400 Bad Request",
                        "application/json",
                        r#"{"error":"invalid_json"}"#,
                    );
                }
            };
            start_run(&mut stream, Arc::clone(shared), body.filter)
        }
        ("POST", "/api/v1/runs/rerun") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: RerunBody = match parse_json_body(&request) {
                Ok(body) => body,
                Err(_) => {
                    return respond(
                        &mut stream,
                        "400 Bad Request",
                        "application/json",
                        r#"{"error":"invalid_json"}"#,
                    );
                }
            };
            start_run(&mut stream, Arc::clone(shared), Some(body.test))
        }
        ("POST", "/api/v1/runs/cancel") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            cancel_run(&mut stream, shared)
        }
        ("POST", "/api/v1/hints") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            match application::reveal_next_hint(&shared.options) {
                Ok(content) => respond(
                    &mut stream,
                    "200 OK",
                    "application/json",
                    &serde_json::to_string(&content)?,
                ),
                Err(error) => respond(
                    &mut stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("POST", "/api/v1/capabilities/next") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            match application::begin_next_capability(&shared.options) {
                Ok(state) => respond(
                    &mut stream,
                    "200 OK",
                    "application/json",
                    &serde_json::to_string(&state)?,
                ),
                Err(error) => respond(
                    &mut stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("POST", "/api/v1/project/repin-pack") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            if parse_json_body::<EmptyBody>(&request).is_err() {
                return respond(
                    &mut stream,
                    "400 Bad Request",
                    "application/json",
                    r#"{"error":"invalid_json"}"#,
                );
            }
            match application::repin_current_pack(&shared.options) {
                Ok(health) => respond(
                    &mut stream,
                    "200 OK",
                    "application/json",
                    &serde_json::to_string(&health)?,
                ),
                Err(error) => respond(
                    &mut stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("POST", "/api/v1/project/open-editor") => {
            open_project(&mut stream, shared, &request, ProjectOpenKind::Editor)
        }
        ("POST", "/api/v1/project/open-folder") => {
            open_project(&mut stream, shared, &request, ProjectOpenKind::Folder)
        }
        ("POST", "/api/v1/service/shutdown") => shutdown_service(&mut stream, shared, &request),
        ("POST", _) | ("GET", _) => respond(
            &mut stream,
            "404 Not Found",
            "application/json",
            r#"{"error":"not_found"}"#,
        ),
        _ => respond(
            &mut stream,
            "404 Not Found",
            "application/json",
            r#"{"error":"not_found"}"#,
        ),
    }
}

fn read_request(stream: &mut TcpStream) -> Result<Option<HttpRequest>> {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 4096];
    let header_end = loop {
        if let Some(index) = bytes.windows(4).position(|window| window == b"\r\n\r\n") {
            let end = index + 4;
            if end > MAX_HEADER_BYTES {
                bail!("request headers exceed the workbench limit");
            }
            break end;
        }
        if bytes.len() >= MAX_HEADER_BYTES {
            bail!("request headers exceed the workbench limit");
        }
        match stream.read(&mut chunk) {
            Ok(0) if bytes.is_empty() => return Ok(None),
            Ok(0) => bail!("request headers ended before the header terminator"),
            Ok(read) => bytes.extend_from_slice(&chunk[..read]),
            Err(error) if bytes.is_empty() => return Err(error.into()),
            Err(_) => return Ok(None),
        }
    };
    let text =
        std::str::from_utf8(&bytes[..header_end]).context("request headers are not UTF-8")?;
    let mut lines = text.split("\r\n");
    let request_line = lines.next().context("request line is missing")?;
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .context("request method is missing")?
        .to_string();
    let target = parts
        .next()
        .context("request target is missing")?
        .to_string();
    if parts.next() != Some("HTTP/1.1") || parts.next().is_some() {
        bail!("unsupported HTTP request line");
    }

    let mut headers = BTreeMap::new();
    for line in lines.take_while(|line| !line.is_empty()) {
        let (name, value) = line.split_once(':').context("malformed HTTP header")?;
        let name = name.trim().to_ascii_lowercase();
        if headers.insert(name, value.trim().to_string()).is_some() {
            bail!("duplicate HTTP headers are not accepted");
        }
    }
    if headers.contains_key("transfer-encoding") {
        bail!("transfer-encoded request bodies are not accepted");
    }
    let content_length = headers
        .get("content-length")
        .map(|value| value.parse::<usize>().context("invalid Content-Length"))
        .transpose()?
        .unwrap_or_default();
    if content_length > MAX_BODY_BYTES {
        bail!("request body exceeds the workbench limit");
    }
    while bytes.len() < header_end + content_length {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            bail!("request body ended before Content-Length");
        }
        bytes.extend_from_slice(&chunk[..read]);
    }
    Ok(Some(HttpRequest {
        method,
        target,
        headers,
        body: bytes[header_end..header_end + content_length].to_vec(),
    }))
}

fn authorized(request: &HttpRequest, shared: &Shared) -> bool {
    let expected_host = format!("127.0.0.1:{}", shared.port);
    if request.headers.get("host") != Some(&expected_host) {
        return false;
    }
    let token = request
        .target
        .split_once('?')
        .and_then(|(_, query)| {
            query
                .split('&')
                .find_map(|pair| pair.strip_prefix("token="))
        })
        .unwrap_or_default();
    if token != shared.token {
        return false;
    }
    let expected_origin = format!("http://{expected_host}");
    request
        .headers
        .get("origin")
        .is_none_or(|origin| origin == &expected_origin)
}

fn authorized_mutation(request: &HttpRequest, shared: &Shared) -> bool {
    let expected_origin = format!("http://127.0.0.1:{}", shared.port);
    authorized(request, shared)
        && request.headers.get("origin") == Some(&expected_origin)
        && request
            .headers
            .get("content-type")
            .is_some_and(|value| value.eq_ignore_ascii_case("application/json"))
}

fn parse_json_body<T>(request: &HttpRequest) -> Result<T>
where
    T: for<'de> Deserialize<'de> + Default,
{
    if request.body.is_empty() {
        return Ok(T::default());
    }
    serde_json::from_slice(&request.body).context("invalid JSON request body")
}

fn start_run(stream: &mut TcpStream, shared: Arc<Shared>, filter: Option<String>) -> Result<()> {
    if filter
        .as_ref()
        .is_some_and(|value| value.trim().is_empty() || value.len() > 200)
    {
        return respond(
            stream,
            "400 Bad Request",
            "application/json",
            r#"{"error":"invalid_test_filter"}"#,
        );
    }
    let mut starting = shared.run_starting.lock().expect("workbench lock poisoned");
    if shared.shutting_down.load(Ordering::SeqCst)
        || *starting
        || application::run_is_active(&shared.options)?
    {
        return respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"run_already_active"}"#,
        );
    }
    *starting = true;
    drop(starting);

    let worker = Arc::clone(&shared);
    std::thread::spawn(move || {
        let request = application::TestRunRequest {
            stage: None,
            all: false,
            filter,
            list_tests: false,
            fail_fast: false,
            no_build: false,
            keep_temp: false,
            capture_details: true,
            trigger: application::RunTrigger::Workbench,
        };
        let mut sink = application::NullEventSink;
        if let Err(error) = application::run_tests(&worker.options, request, &mut sink)
            && !format!("{error:#}").contains("already active")
        {
            let _ = application::publish_event(
                &worker.options,
                &application::RunEvent::JobInterrupted {
                    job_id: "pending".to_string(),
                    reason: format!("{error:#}"),
                },
            );
        }
        *worker.run_starting.lock().expect("workbench lock poisoned") = false;
        *worker
            .last_activity
            .lock()
            .expect("workbench lock poisoned") = Instant::now();
    });
    respond(
        stream,
        "202 Accepted",
        "application/json",
        r#"{"status":"accepted"}"#,
    )
}

fn cancel_run(stream: &mut TcpStream, shared: &Shared) -> Result<()> {
    match application::cancel_active_run(&shared.options) {
        Ok(job_id) => respond(
            stream,
            "202 Accepted",
            "application/json",
            &serde_json::json!({"status": "cancelling", "job_id": job_id}).to_string(),
        ),
        Err(_) => respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"no_active_run"}"#,
        ),
    }
}

fn shutdown_service(stream: &mut TcpStream, shared: &Shared, request: &HttpRequest) -> Result<()> {
    if !authorized_mutation(request, shared) {
        return respond(
            stream,
            "403 Forbidden",
            "application/json",
            r#"{"error":"forbidden"}"#,
        );
    }
    if parse_json_body::<EmptyBody>(request).is_err() {
        return respond(
            stream,
            "400 Bad Request",
            "application/json",
            r#"{"error":"invalid_json"}"#,
        );
    }
    let run_starting = shared.run_starting.lock().expect("workbench lock poisoned");
    if *run_starting || application::run_is_active(&shared.options).unwrap_or(false) {
        return respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"run_active"}"#,
        );
    }
    shared.shutting_down.store(true, Ordering::SeqCst);
    drop(run_starting);

    let _ = fs::remove_file(&shared.record_path);
    let response = respond(
        stream,
        "202 Accepted",
        "application/json",
        r#"{"status":"stopping"}"#,
    );
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(50));
        std::process::exit(0);
    });
    response
}

fn open_project(
    stream: &mut TcpStream,
    shared: &Shared,
    request: &HttpRequest,
    kind: ProjectOpenKind,
) -> Result<()> {
    if !authorized_mutation(request, shared) {
        return respond(
            stream,
            "403 Forbidden",
            "application/json",
            r#"{"error":"forbidden"}"#,
        );
    }
    if parse_json_body::<EmptyBody>(request).is_err() {
        return respond(
            stream,
            "400 Bad Request",
            "application/json",
            r#"{"error":"invalid_json"}"#,
        );
    }
    let target = application::project_open_target(&shared.options)?;
    let editor = std::env::var("VISUAL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| std::env::var("EDITOR").ok());
    let mut command = project_open_command(kind, &target, editor.as_deref())?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    match command.spawn() {
        Ok(_) => respond(
            stream,
            "202 Accepted",
            "application/json",
            r#"{"status":"opened"}"#,
        ),
        Err(error) => respond(
            stream,
            "409 Conflict",
            "application/json",
            &serde_json::json!({"error": format!("could not open project: {error}")}).to_string(),
        ),
    }
}

fn project_open_command(
    kind: ProjectOpenKind,
    target: &Path,
    editor: Option<&str>,
) -> Result<Command> {
    if matches!(kind, ProjectOpenKind::Editor)
        && let Some(editor) = editor
    {
        let mut parts = editor.split_whitespace();
        let program = parts
            .next()
            .filter(|part| !part.is_empty())
            .context("the configured VISUAL or EDITOR command is empty")?;
        let mut command = Command::new(program);
        command.args(parts).arg(target);
        return Ok(command);
    }

    #[cfg(target_os = "macos")]
    {
        let mut command = Command::new("open");
        if matches!(kind, ProjectOpenKind::Editor) {
            command.arg("-a").arg("Visual Studio Code");
        }
        command.arg(target);
        Ok(command)
    }
    #[cfg(target_os = "linux")]
    {
        let mut command = if matches!(kind, ProjectOpenKind::Editor) {
            Command::new("code")
        } else {
            Command::new("xdg-open")
        };
        command.arg(target);
        Ok(command)
    }
    #[cfg(windows)]
    {
        let mut command = if matches!(kind, ProjectOpenKind::Editor) {
            Command::new("code.cmd")
        } else {
            Command::new("explorer")
        };
        command.arg(target);
        Ok(command)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    {
        let _ = kind;
        let _ = target;
        bail!("opening a project is unsupported on this platform")
    }
}

fn serve_events(mut stream: TcpStream, shared: &Shared, request: &HttpRequest) -> Result<()> {
    stream.set_read_timeout(None)?;
    let headers = concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Type: text/event-stream\r\n",
        "Cache-Control: no-cache\r\n",
        "X-Content-Type-Options: nosniff\r\n",
        "Referrer-Policy: no-referrer\r\n",
        "Connection: keep-alive\r\n\r\n"
    );
    stream.write_all(headers.as_bytes())?;

    struct ClientGuard<'a>(&'a AtomicUsize);
    impl Drop for ClientGuard<'_> {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::SeqCst);
        }
    }
    shared.clients.fetch_add(1, Ordering::SeqCst);
    let _guard = ClientGuard(&shared.clients);
    let mut cursor = query_value(&request.target, "after")
        .and_then(|value| value.parse::<u64>().ok())
        .or_else(|| {
            request
                .headers
                .get("last-event-id")
                .and_then(|value| value.parse::<u64>().ok())
        })
        .unwrap_or(crate::run_journal::cursor(project_root(&shared.options)?)?);
    let mut previous = String::new();
    let mut focus_revision = shared.focus_revision.load(Ordering::SeqCst);

    loop {
        let current_focus_revision = shared.focus_revision.load(Ordering::SeqCst);
        if current_focus_revision != focus_revision {
            focus_revision = current_focus_revision;
            if stream.write_all(b"event: focus\ndata: {}\n\n").is_err() {
                return Ok(());
            }
        }
        for entry in crate::run_journal::entries_after(project_root(&shared.options)?, cursor)? {
            let serialized = serde_json::to_string(&entry.event)?;
            let payload = format!("id: {}\nevent: run\ndata: {serialized}\n\n", entry.id);
            if stream.write_all(payload.as_bytes()).is_err() {
                return Ok(());
            }
            cursor = entry.id;
        }
        let state =
            application::load_workbench_state_for_session(&shared.options, &shared.session_id)?;
        let serialized = serde_json::to_string(&state)?;
        let payload = if serialized != previous {
            previous = serialized.clone();
            format!("event: state\ndata: {serialized}\n\n")
        } else {
            ": keep-alive\n\n".to_string()
        };
        if stream.write_all(payload.as_bytes()).is_err() {
            return Ok(());
        }
        *shared
            .last_activity
            .lock()
            .expect("workbench lock poisoned") = Instant::now();
        std::thread::sleep(EVENT_POLL_INTERVAL);
    }
}

fn project_root(options: &GlobalOptions) -> Result<&Path> {
    options
        .project_dir
        .as_deref()
        .context("workbench project root is missing")
}

fn query_value<'a>(target: &'a str, name: &str) -> Option<&'a str> {
    target.split_once('?').and_then(|(_, query)| {
        query.split('&').find_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            (key == name).then_some(value)
        })
    })
}

fn respond(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) -> Result<()> {
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nContent-Security-Policy: default-src 'self'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; connect-src 'self'; img-src 'self' data:\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nX-Frame-Options: DENY\r\nConnection: close\r\n\r\n{body}",
        body.len(),
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn http_get(port: u16, path: &str) -> Option<String> {
    let response = http_get_response(port, path)?;
    if !response.starts_with("HTTP/1.1 200") {
        return None;
    }
    response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body.to_string())
}

fn http_get_response(port: u16, path: &str) -> Option<String> {
    let request =
        format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n");
    http_exchange(port, &request)
}

fn http_exchange(port: u16, request: &str) -> Option<String> {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);
    let mut stream = TcpStream::connect_timeout(&address, PROBE_TIMEOUT).ok()?;
    stream.set_read_timeout(Some(PROBE_TIMEOUT)).ok()?;
    stream.set_write_timeout(Some(PROBE_TIMEOUT)).ok()?;
    stream.write_all(request.as_bytes()).ok()?;
    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;
    Some(response)
}

fn capability_token(root: &Path) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in root
        .to_string_lossy()
        .as_bytes()
        .iter()
        .chain(nanos.to_le_bytes().iter())
        .chain(std::process::id().to_le_bytes().iter())
    {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}{nanos:x}")
}
fn workbench_html(token: &str) -> String {
    let token_json = serde_json::to_string(token).unwrap_or_else(|_| "\"\"".to_string());
    let html = r###"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="color-scheme" content="dark">
<title>DeltaForge</title>
<style>
:root {
  --bg: #090b10; --panel: #10141d; --panel-2: #151b27; --line: #252d3c;
  --ink: #f2f5fb; --muted: #8e99aa; --blue: #75a7ff; --cyan: #6de3d1;
  --amber: #f4c26b; --radius: 18px;
  font-family: Inter, ui-sans-serif, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}
* { box-sizing: border-box; }
body {
  margin: 0; min-height: 100vh; color: var(--ink);
  background: radial-gradient(circle at 70% -10%, rgba(70,113,196,.18), transparent 38rem), var(--bg);
}
button { font: inherit; }
button:focus-visible, summary:focus-visible { outline: 3px solid var(--cyan); outline-offset: 3px; }
.shell { min-height: 100vh; display: grid; grid-template-columns: 250px minmax(420px,1fr) 330px; }
.rail {
  border-right: 1px solid var(--line); padding: 30px 24px;
  display: flex; flex-direction: column; gap: 34px;
}
.brand { font-weight: 720; letter-spacing: -.03em; font-size: 1.08rem; }
.brand span { color: var(--blue); }
.eyebrow {
  color: var(--muted); font: 650 .68rem/1 ui-monospace, SFMono-Regular, Consolas, monospace;
  letter-spacing: .14em; text-transform: uppercase;
}
.project-name { margin-top: 9px; font-weight: 650; }
.cap-list { display: grid; gap: 8px; }
.cap {
  padding: 11px 12px; border: 1px solid transparent; border-radius: 11px;
  color: var(--muted); font-size: .86rem;
}
.cap.current { color: var(--ink); background: var(--panel); border-color: var(--line); }
.rail-foot { margin-top: auto; color: var(--muted); font-size: .76rem; line-height: 1.55; }
.repo-actions { margin-top: 14px; display: grid; gap: 7px; }
.repo-action { border: 1px solid var(--line); border-radius: 9px; padding: 8px 10px; text-align: left; color: #c8d0dc; background: transparent; cursor: pointer; font-size: .74rem; }
.local-note { display: inline; }
main { padding: 46px clamp(34px,5vw,76px); }
.health-screen { max-width: 720px; margin: 12vh auto 0; padding: 28px; border: 1px solid var(--line); border-radius: var(--radius); background: var(--panel); }
.health-screen[hidden] { display: none; }
.health-screen h1 { margin: 14px 0; font-size: clamp(2rem,4vw,3.6rem); }
.health-screen p { color: var(--muted); line-height: 1.6; }
.health-detail { padding: 13px; border-radius: 10px; background: #090c12; color: #c8d0dc; font: .76rem/1.5 ui-monospace, monospace; white-space: pre-wrap; overflow-wrap: anywhere; }
.health-actions { display: flex; flex-wrap: wrap; gap: 9px; margin-top: 22px; }
.health-actions button { border: 1px solid var(--line); border-radius: 999px; padding: 10px 14px; color: var(--ink); background: transparent; cursor: pointer; font-weight: 650; }
.health-actions button.health-primary { color: #07101a; background: var(--blue); border-color: var(--blue); }
.shell.project-unhealthy .mission, .shell.project-unhealthy > .evidence { display: none; }
.mission { max-width: 820px; margin: 0 auto; }
.kicker { color: var(--cyan); font: 650 .72rem/1 ui-monospace, monospace; letter-spacing: .12em; }
h1 { margin: 18px 0 14px; font-size: clamp(2.35rem,5vw,4.5rem); line-height: .98; letter-spacing: -.055em; }
.lede { max-width: 640px; color: #b7c0ce; font-size: 1.06rem; line-height: 1.65; }
.action-row { display: flex; align-items: center; gap: 14px; margin: 30px 0 36px; }
.primary {
  border: 0; border-radius: 999px; padding: 13px 22px; color: #07101a;
  background: var(--blue); font-weight: 720; cursor: pointer;
}
.primary:disabled { cursor: default; opacity: .55; }
.secondary {
  display: none; margin-top: 14px; border: 1px solid var(--line); border-radius: 999px;
  padding: 8px 12px; color: var(--ink); background: transparent; cursor: pointer;
  font-size: .78rem; font-weight: 650;
}
.secondary.visible { display: inline-flex; }
.status { color: var(--muted); font-size: .84rem; }
.card-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 14px; }
.card {
  min-height: 150px; padding: 20px; border: 1px solid var(--line);
  border-radius: var(--radius); background: rgba(16,20,29,.78);
}
.card h2 { margin: 9px 0 8px; font-size: 1rem; }
.card p { margin: 0; color: var(--muted); line-height: 1.55; font-size: .9rem; }
.spec { margin-top: 16px; border: 1px solid var(--line); border-radius: var(--radius); overflow: hidden; background: rgba(16,20,29,.55); }
.spec details + details { border-top: 1px solid var(--line); }
.spec summary { padding: 16px 20px; cursor: pointer; color: #c8d0dc; font-size: .86rem; font-weight: 650; }
.spec-body { padding: 0 20px 18px; color: var(--muted); font-size: .86rem; line-height: 1.55; }
.bullet-list { margin: 0; padding-left: 1.15rem; display: grid; gap: 8px; }
.example { margin: 0; padding: 14px; overflow: auto; border-radius: 11px; background: #090c12; color: #c8d8f4; font: .76rem/1.55 ui-monospace, monospace; white-space: pre-wrap; }
.evidence { border-left: 1px solid var(--line); padding: 30px 24px; background: rgba(9,11,16,.5); }
.evidence h2 { font-size: .94rem; margin: 9px 0 26px; }
.signal { padding: 16px; border-radius: 14px; background: var(--panel); border: 1px solid var(--line); }
.signal strong { display: block; font-size: .9rem; margin-bottom: 6px; }
.signal p { color: var(--muted); margin: 0; font-size: .82rem; line-height: 1.5; }
.signal.fresh strong { color: var(--cyan); }
.signal.stale strong { color: var(--amber); }
.run-meter { margin-top: 16px; color: var(--muted); font: .76rem/1.5 ui-monospace, monospace; }
.contract { margin-top: 28px; display: grid; gap: 13px; }
.contract-row { display: flex; gap: 10px; color: #b7c0ce; font-size: .82rem; line-height: 1.45; }
.dot { width: 6px; height: 6px; margin-top: 6px; border-radius: 50%; background: var(--blue); flex: 0 0 auto; }
.diagnosis { display: none; margin-top: 16px; padding-top: 16px; border-top: 1px solid var(--line); }
.diagnosis.visible { display: grid; gap: 13px; }
.diagnosis-item span { display: block; margin-bottom: 4px; color: var(--muted); font: 650 .63rem/1 ui-monospace, monospace; letter-spacing: .1em; text-transform: uppercase; }
.diagnosis-item p, .diagnosis-item pre { margin: 0; color: #c8d0dc; font-size: .78rem; line-height: 1.5; white-space: pre-wrap; overflow-wrap: anywhere; }
.diagnosis-item pre { max-height: 150px; overflow: auto; padding: 10px; border-radius: 9px; background: #090c12; }
.other-failures { display: none; margin-top: 15px; border-top: 1px solid var(--line); padding-top: 13px; }
.other-failures.visible { display: block; }
.other-failures summary { cursor: pointer; color: var(--muted); font-size: .76rem; }
.other-failures ul { margin: 11px 0 0; padding-left: 1rem; color: #b7c0ce; font-size: .76rem; line-height: 1.5; }
.other-failures li { margin: 7px 0; padding-left: 2px; }
.other-failure-row { display: grid; grid-template-columns: minmax(0,1fr) auto; align-items: start; gap: 8px; }
.secondary-rerun { border: 1px solid var(--line); border-radius: 999px; padding: 5px 8px; color: #c8d0dc; background: transparent; cursor: pointer; font-size: .68rem; }
.help { margin-top: 28px; padding-top: 24px; border-top: 1px solid var(--line); }
.help-head { display: flex; align-items: center; justify-content: space-between; gap: 10px; }
.help button { border: 1px solid var(--line); border-radius: 999px; padding: 7px 11px; color: var(--ink); background: transparent; cursor: pointer; font-size: .74rem; font-weight: 650; }
.help button:disabled { opacity: .5; cursor: default; }
.help-levels { margin-top: 13px; display: grid; gap: 10px; }
.help-level { padding: 13px; border: 1px solid var(--line); border-radius: 12px; background: var(--panel); }
.help-level strong { color: var(--cyan); font-size: .76rem; }
.help-level p { margin: 7px 0 0; color: var(--muted); font-size: .79rem; line-height: 1.5; white-space: pre-wrap; }
@media (max-width: 1040px) {
  .shell { grid-template-columns: 210px 1fr; }
  .evidence { grid-column: 2; border-top: 1px solid var(--line); border-left: 0; }
}
@media (max-width: 720px) {
  .shell { display: block; }
  .rail { border-right: 0; border-bottom: 1px solid var(--line); padding: 20px 22px; display: grid; grid-template-columns: 1fr auto; align-items: center; gap: 16px; }
  .rail-capabilities { display: none; }
  .rail .eyebrow { display: none; }
  .project-name { margin-top: 0; font-size: .86rem; }
  .rail-foot { grid-column: 1 / -1; margin-top: 0; }
  .local-note { display: none; }
  .repo-actions { margin-top: 0; display: flex; }
  .repo-action { min-height: 44px; flex: 1; text-align: center; }
  main { padding: 34px 22px; }
  .action-row { align-items: stretch; flex-wrap: wrap; }
  .primary { min-height: 48px; }
  .status { flex: 1; min-width: 140px; align-self: center; line-height: 1.4; }
  .card-grid { grid-template-columns: 1fr; }
}
</style>
</head>
<body>
<div class="shell" id="app-shell">
  <aside class="rail">
    <div class="brand">Delta<span>Forge</span></div>
    <div><div class="eyebrow">Project</div><div class="project-name" id="project">Loading?</div></div>
    <div class="rail-capabilities">
      <div class="eyebrow">Capabilities</div>
      <div class="cap-list">
        <div class="cap current" id="current-cap">Current capability</div>
        <div class="cap" id="next-cap">Next capability locked</div>
      </div>
    </div>
    <div class="rail-foot"><span class="local-note">Local workbench<br>No account · Works offline</span>
      <div class="repo-actions"><button class="repo-action" id="open-editor" type="button">Open editor</button><button class="repo-action" id="open-folder" type="button">Open folder</button></div>
    </div>
  </aside>
  <main>
    <section class="health-screen" id="health-screen" hidden>
      <div class="eyebrow">Project recovery</div>
      <h1 id="health-title">DeltaForge cannot load this project</h1>
      <p id="health-guidance"></p>
      <pre class="health-detail" id="health-detail"></pre>
      <div class="health-actions"><button class="health-primary" id="health-recheck" type="button">Check again</button><button id="health-repin" type="button" hidden>Adopt current pack</button><button id="health-editor" type="button">Open editor</button><button id="health-folder" type="button">Open folder</button></div>
    </section>
    <section class="mission">
      <div class="kicker">CURRENT MISSION</div>
      <h1 id="mission-title">Loading your mission</h1>
      <p class="lede" id="mission-copy">Loading the capability contract…</p>
      <div class="action-row">
        <button class="primary" id="primary-action" type="button">Run checks</button>
        <span class="status" id="activity" aria-live="polite">Connecting to the local workbench…</span>
      </div>
      <div class="card-grid">
        <article class="card">
          <div class="eyebrow">Why it matters</div>
          <h2>Capability context</h2>
          <p id="why-copy">Loading…</p>
        </article>
        <article class="card">
          <div class="eyebrow">Definition of done</div>
          <h2>Observable outcomes</h2>
          <ul class="bullet-list" id="success-list"></ul>
        </article>
      </div>
      <div class="spec">
        <details open><summary>Requirements</summary><div class="spec-body"><ul class="bullet-list" id="requirements-list"></ul></div></details>
        <details><summary>Example</summary><div class="spec-body"><pre class="example" id="example-copy"></pre></div></details>
        <details><summary>Edge cases</summary><div class="spec-body"><ul class="bullet-list" id="edge-list"></ul></div></details>
        <details><summary>Non-goals</summary><div class="spec-body"><ul class="bullet-list" id="non-goals-list"></ul></div></details>
      </div>
    </section>
  </main>
  <aside class="evidence">
    <div class="eyebrow">Evidence</div>
    <h2>Your latest proof</h2>
    <div class="signal" id="signal">
      <strong id="signal-title">No run yet</strong>
      <p id="signal-copy">Run checks when you are ready. DeltaForge will preserve the result for your next session.</p>
      <button class="secondary" id="focused-rerun" type="button">Rerun this check</button>
      <div class="run-meter" id="run-meter" aria-live="polite"></div>
      <div class="diagnosis" id="diagnosis">
        <div class="diagnosis-item"><span>Contract</span><p id="diagnosis-contract"></p></div>
        <div class="diagnosis-item"><span>Expected</span><pre id="diagnosis-expected"></pre></div>
        <div class="diagnosis-item"><span>Observed</span><pre id="diagnosis-actual"></pre></div>
        <div class="diagnosis-item" id="diagnosis-fixture-row"><span>Fixture</span><p id="diagnosis-fixture"></p></div>
      </div>
      <details class="other-failures" id="other-failures"><summary id="other-failures-title">Other contradictions</summary><ul id="other-failures-list"></ul></details>
    </div>
    <div class="contract" id="contract">
      <div class="eyebrow">Contract</div>
      <div id="contract-rows"></div>
    </div>
    <div class="help">
      <div class="help-head"><div class="eyebrow">Progressive help</div><button id="reveal-help" type="button">Reveal observation</button></div>
      <div class="help-levels" id="help-levels"></div>
    </div>
  </aside>
</div>
<script>
const token = __TOKEN_JSON__;
const stateUrl = "/api/v1/state?token=" + encodeURIComponent(token);
const api = path => path + "?token=" + encodeURIComponent(token);
const capabilityUrl = api("/api/v1/capability");
const healthUrl = api("/api/v1/project-health");
const action = document.querySelector("#primary-action");
const rerun = document.querySelector("#focused-rerun");
const help = document.querySelector("#reveal-help");
const meter = document.querySelector("#run-meter");
let canonical = null;
let content = null;
let events = null;
let busy = false;
let currentHealth = null;
let live = { active: false, phase: "", passed: 0, failed: 0, current: 0, total: 0, started: 0 };

async function loadHealth() {
  const response = await fetch(healthUrl);
  if (!response.ok) throw new Error("project health unavailable");
  const health = await response.json();
  renderHealth(health);
  return health;
}

function renderHealth(health) {
  currentHealth = health;
  const unhealthy = health.status === "unhealthy";
  document.querySelector("#app-shell").classList.toggle("project-unhealthy", unhealthy);
  document.querySelector("#health-screen").hidden = !unhealthy;
  if (!unhealthy) return;
  const issue = health.issue || {};
  document.querySelector("#health-title").textContent = issue.title || "DeltaForge cannot load this project";
  document.querySelector("#health-guidance").textContent = issue.guidance || "Resolve the reported problem, then check again.";
  document.querySelector("#health-detail").textContent = issue.detail || "Project health check failed.";
  document.querySelector("#health-repin").hidden = !health.actions.some(item => item.kind === "repin_pack");
}

function firstFailure(state) {
  return state.primary_failure || null;
}

function fillList(selector, items) {
  const list = document.querySelector(selector);
  list.replaceChildren(...(items || []).map(item => {
    const row = document.createElement("li"); row.textContent = item; return row;
  }));
}

function renderContent(next) {
  content = next;
  document.querySelector("#mission-title").textContent = next.title;
  document.querySelector("#mission-copy").textContent = next.mission;
  document.querySelector("#why-copy").textContent = next.why;
  fillList("#success-list", next.success_conditions);
  fillList("#requirements-list", next.requirements);
  fillList("#edge-list", next.edge_cases);
  fillList("#non-goals-list", next.non_goals);
  document.querySelector("#example-copy").textContent = next.example;
  document.querySelector("#current-cap").textContent = next.title;
  document.querySelector("#next-cap").textContent = next.next ? "Next · " + next.next.title : "Final capability";

  const contractRows = document.querySelector("#contract-rows");
  contractRows.replaceChildren(...next.requirements.slice(0, 4).map(requirement => {
    const row = document.createElement("div"); row.className = "contract-row";
    const dot = document.createElement("span"); dot.className = "dot";
    const copy = document.createElement("span"); copy.textContent = requirement;
    row.append(dot, copy); return row;
  }));

  const helpLevels = document.querySelector("#help-levels");
  helpLevels.replaceChildren(...next.revealed_help.map(level => {
    const panel = document.createElement("div"); panel.className = "help-level";
    const title = document.createElement("strong"); title.textContent = `Level ${level.level} · ${level.label}`;
    const copy = document.createElement("p"); copy.textContent = level.content;
    panel.append(title, copy); return panel;
  }));
  const revealed = next.revealed_help.length;
  const completed = Boolean(canonical && canonical.capability.completed);
  const available = completed ? next.help_levels : Math.min(next.help_levels, 4);
  help.disabled = busy || revealed >= available;
  help.textContent = revealed >= available
    ? (completed || revealed === next.help_levels ? "All help revealed" : "Retrospective locked")
    : `Reveal level ${revealed + 1}`;
}

function showEvidence(titleText, copyText, tone = "") {
  const signal = document.querySelector("#signal");
  signal.className = "signal" + (tone ? " " + tone : "");
  document.querySelector("#signal-title").textContent = titleText;
  document.querySelector("#signal-copy").textContent = copyText;
}

function renderDiagnosis(failure) {
  const panel = document.querySelector("#diagnosis");
  const diagnosis = failure && failure.diagnosis;
  panel.classList.toggle("visible", Boolean(diagnosis));
  if (!diagnosis) return;
  document.querySelector("#diagnosis-contract").textContent = diagnosis.contract || diagnosis.summary;
  document.querySelector("#diagnosis-expected").textContent = diagnosis.expected || "The capability contract should hold.";
  document.querySelector("#diagnosis-actual").textContent = diagnosis.actual || failure.failures.join("\n") || "The check failed.";
  const fixture = [diagnosis.fixture, ...(diagnosis.fixture_entries || [])].filter(Boolean).join(" · ");
  document.querySelector("#diagnosis-fixture").textContent = fixture;
  document.querySelector("#diagnosis-fixture-row").style.display = fixture ? "block" : "none";
}

function renderSecondaryFailures(state) {
  const failures = state.latest_run && state.latest_run.failed_tests ? state.latest_run.failed_tests.slice(1) : [];
  const panel = document.querySelector("#other-failures");
  panel.classList.toggle("visible", failures.length > 0 && state.freshness === "fresh" && !state.active_job);
  document.querySelector("#other-failures-title").textContent = `${failures.length} other contradiction${failures.length === 1 ? "" : "s"}`;
  const list = document.querySelector("#other-failures-list");
  list.replaceChildren(...failures.map(failure => {
    const row = document.createElement("li");
    const content = document.createElement("div");
    content.className = "other-failure-row";
    const label = document.createElement("span");
    label.textContent = failure.diagnosis ? failure.diagnosis.headline : failure.name;
    const button = document.createElement("button");
    button.type = "button";
    button.className = "secondary-rerun";
    button.dataset.test = failure.name;
    button.textContent = "Rerun";
    button.setAttribute("aria-label", "Rerun " + failure.name);
    content.append(label, button);
    row.append(content);
    return row;
  }));
}

function renderAction(state) {
  const running = live.active || Boolean(state.active_job);
  action.disabled = busy || (!running && !state.primary_action.enabled);
  action.textContent = running ? "Cancel run" : state.primary_action.label;
  if (content) renderContent(content);
}

function formatTimestamp(value) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium", timeStyle: "short"
  }).format(date);
}

function render(state) {
  canonical = state;
  const returning = state.resumption && state.resumption.action_pending ? state.resumption : null;
  document.querySelector("#project").textContent = state.project + " · " + state.language;
  if (!content) {
    document.querySelector("#mission-title").textContent = state.capability.title;
    document.querySelector("#current-cap").textContent = state.capability.title;
  }
  document.querySelector("#activity").textContent =
    returning && returning.previous_session_started_at ? "Returned · " + formatTimestamp(returning.previous_session_started_at) :
    state.recovered_interrupted_job ? "Previous run was safely recovered" :
    state.freshness === "stale" && state.last_source_change ? "Source changed · " + formatTimestamp(state.last_source_change.observed_at) :
    "Last activity · " + formatTimestamp(state.last_activity_at);
  const failure = firstFailure(state);
  const fatalFailure = Boolean(failure && failure.diagnosis && failure.diagnosis.kind === "build");
  renderSecondaryFailures(state);
  rerun.classList.toggle("visible", Boolean(failure) && state.freshness === "fresh" && !state.active_job);
  rerun.dataset.test = failure ? failure.name : "";
  if (live.active || state.active_job) {
    showEvidence("Checks are running", live.phase || "Preparing the project…");
    renderDiagnosis(null);
  } else if (returning) {
    const tone = returning.kind === "interrupted" || returning.kind === "source_changed" ? "stale" : "fresh";
    showEvidence("Welcome back · " + returning.title, returning.detail, tone);
    renderDiagnosis(state.freshness === "fresh" ? failure : null);
  } else if (fatalFailure) {
    showEvidence("Start here · " + failure.diagnosis.headline, failure.diagnosis.summary);
    renderDiagnosis(failure);
  } else if (state.freshness === "fresh") {
    if (state.capability.completed) {
      showEvidence("Capability acquired", content ? content.capability_statement : "This evidence matches the source currently on disk.", "fresh");
      renderDiagnosis(null);
    } else if (failure) {
      const diagnosis = failure.diagnosis;
      showEvidence("Start here · " + (diagnosis ? diagnosis.headline : failure.name),
        diagnosis ? diagnosis.summary : (failure.failures && failure.failures[0] ? failure.failures[0] : "This check contradicted the contract."));
      renderDiagnosis(failure);
    } else {
      showEvidence("Result is current", "This evidence matches the source currently on disk.", "fresh");
      renderDiagnosis(null);
    }
  } else if (state.freshness === "stale") {
    showEvidence("Source changed", "Your previous result is preserved, but it no longer proves the current code.", "stale");
    renderDiagnosis(null);
  } else {
    showEvidence("No run yet", "Run checks when you are ready. DeltaForge will preserve the result for your next session.");
    renderDiagnosis(null);
  }
  renderAction(state);
  renderMeter();
}

function renderMeter() {
  if (!live.active) {
    const run = canonical && canonical.freshness === "fresh" ? canonical.latest_run : null;
    meter.textContent = run ? `${run.passed} passed · ${run.failed} failed` : "";
    return;
  }
  const seconds = live.started ? Math.max(0, Math.floor((performance.now() - live.started) / 1000)) : 0;
  const count = live.total ? ` · ${live.current}/${live.total}` : "";
  meter.textContent = `${live.phase || "Running"}${count} · ${seconds}s · ${live.passed} passed · ${live.failed} failed`;
}

async function loadState() {
  const response = await fetch(stateUrl);
  if (!response.ok) throw new Error("state unavailable");
  const state = await response.json();
  render(state);
  return state;
}

async function loadContent() {
  const response = await fetch(capabilityUrl);
  if (!response.ok) throw new Error("capability unavailable");
  const next = await response.json();
  renderContent(next);
  return next;
}

async function post(path, body = {}) {
  const response = await fetch(api(path), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body)
  });
  if (!response.ok) {
    const detail = await response.json().catch(() => ({}));
    throw new Error(detail.error || "operation failed");
  }
  return response.json();
}

function handleRun(event) {
  switch (event.type) {
    case "job_started":
      live = { active: true, phase: "Preparing checks", passed: 0, failed: 0, current: 0, total: 0, started: performance.now(), buildOutput: "" };
      break;
    case "build_started": live.active = true; live.phase = "Building"; live.buildOutput = ""; break;
    case "build_output":
      live.active = true; live.phase = "Building";
      live.buildOutput = ((live.buildOutput || "") + (event.text || "")).slice(-4000);
      showEvidence("Building project · " + event.stream, live.buildOutput.trim() || "Build output received.");
      break;
    case "build_completed": live.phase = event.passed ? "Build complete" : "Build failed"; break;
    case "test_started":
      live.active = true; live.phase = event.name; live.current = event.index; live.total = event.total; break;
    case "test_passed": live.passed += 1; break;
    case "test_failed":
      live.failed += 1;
      showEvidence("Contradiction found · " + event.result.name,
        event.result.failures && event.result.failures[0] ? event.result.failures[0] : "This check contradicted the contract.");
      rerun.dataset.test = event.result.name;
      break;
    case "run_completed":
      live.active = false; live.passed = event.passed_tests; live.failed = event.failed_tests;
      loadState().catch(showConnectionError); break;
    case "job_interrupted":
      live.active = false; showEvidence("Run interrupted", event.reason, "stale");
      loadState().catch(showConnectionError); break;
    case "source_changed": loadState().catch(showConnectionError); break;
    case "project_state_changed": Promise.all([loadState(), loadContent()]).catch(showConnectionError); break;
  }
  if (canonical) { renderAction(canonical); renderMeter(); }
}

function showConnectionError() {
  document.querySelector("#activity").textContent = "Reconnecting to the local workbench…";
}

async function beginHealthy() {
  try {
    const [state] = await Promise.all([loadState(), loadContent()]);
    events = new EventSource(api("/api/v1/events") + "&after=" + state.event_cursor);
    events.addEventListener("state", event => render(JSON.parse(event.data)));
    events.addEventListener("run", event => handleRun(JSON.parse(event.data)));
    events.addEventListener("focus", () => window.focus());
    events.onerror = showConnectionError;
  } catch (_) { showConnectionError(); }
}

async function begin() {
  try {
    const health = await loadHealth();
    if (health.status === "healthy") await beginHealthy();
  } catch (_) { showConnectionError(); }
}

action.addEventListener("click", async () => {
  if (!canonical || busy) return;
  const restoreFocus = document.activeElement === action;
  busy = true; renderAction(canonical);
  try {
    if (live.active || canonical.active_job) await post("/api/v1/runs/cancel");
    else if (canonical.primary_action.kind === "begin_next_capability") {
      const state = await post("/api/v1/capabilities/next");
      render(state); await loadContent();
    } else if (["run_checks", "resume_checks"].includes(canonical.primary_action.kind)) await post("/api/v1/runs");
  } catch (error) {
    showEvidence("Action could not start", error.message);
  } finally {
    busy = false; renderAction(canonical);
    if (restoreFocus && !action.disabled) action.focus();
  }
});

rerun.addEventListener("click", async () => {
  const test = rerun.dataset.test;
  if (!test || busy) return;
  busy = true; renderAction(canonical);
  try { await post("/api/v1/runs/rerun", { test }); }
  catch (error) { showEvidence("Focused rerun could not start", error.message); }
  finally { busy = false; renderAction(canonical); }
});

document.querySelector("#other-failures-list").addEventListener("click", async event => {
  const button = event.target.closest("button[data-test]");
  if (!button || busy) return;
  busy = true; renderAction(canonical);
  try { await post("/api/v1/runs/rerun", { test: button.dataset.test }); }
  catch (error) { showEvidence("Focused rerun could not start", error.message); }
  finally { busy = false; renderAction(canonical); }
});

help.addEventListener("click", async () => {
  if (busy || !content) return;
  const restoreFocus = document.activeElement === help;
  busy = true; renderContent(content);
  try {
    const next = await post("/api/v1/hints");
    renderContent(next);
  } catch (error) {
    document.querySelector("#activity").textContent = error.message;
  } finally {
    busy = false; renderContent(content);
    if (restoreFocus && !help.disabled) help.focus();
  }
});

async function openProject(path) {
  try { await post(path); }
  catch (error) {
    if (currentHealth && currentHealth.status === "unhealthy") document.querySelector("#health-detail").textContent = error.message;
    else showEvidence("Project could not be opened", error.message);
  }
}

document.querySelector("#open-editor").addEventListener("click", () => openProject("/api/v1/project/open-editor"));
document.querySelector("#open-folder").addEventListener("click", () => openProject("/api/v1/project/open-folder"));
document.querySelector("#health-editor").addEventListener("click", () => openProject("/api/v1/project/open-editor"));
document.querySelector("#health-folder").addEventListener("click", () => openProject("/api/v1/project/open-folder"));
document.querySelector("#health-recheck").addEventListener("click", async () => {
  const health = await loadHealth().catch(() => null);
  if (health && health.status === "healthy") location.reload();
});
document.querySelector("#health-repin").addEventListener("click", async () => {
  try {
    const health = await post("/api/v1/project/repin-pack");
    renderHealth(health);
    if (health.status === "healthy") location.reload();
  } catch (error) { document.querySelector("#health-detail").textContent = error.message; }
});

setInterval(renderMeter, 1000);
setInterval(async () => {
  const previous = currentHealth && currentHealth.status;
  const health = await loadHealth().catch(() => null);
  if (!health) return;
  if (previous === "unhealthy" && health.status === "healthy") location.reload();
  if (health.status === "unhealthy" && events) { events.close(); events = null; }
}, 2000);
begin();
</script>
</body>
</html>"###;
    html.replace("__TOKEN_JSON__", &token_json)
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_shared(port: u16) -> Shared {
        Shared {
            options: GlobalOptions::default(),
            token: "secret-token".to_string(),
            session_id: "test-session".to_string(),
            port,
            clients: AtomicUsize::new(0),
            last_activity: Mutex::new(Instant::now()),
            record_path: PathBuf::from("unused-workbench-record.json"),
            run_starting: Mutex::new(false),
            shutting_down: AtomicBool::new(false),
            idle_timeout: IDLE_TIMEOUT,
            focus_revision: AtomicUsize::new(0),
        }
    }

    fn raw_request(target: &str, host: &str, origin: Option<&str>) -> String {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let shared = Arc::new(test_shared(port));
        let shared_for_server = Arc::clone(&shared);
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_connection(stream, &shared_for_server).unwrap();
        });

        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
        let mut request = format!("GET {target} HTTP/1.1\r\nHost: {host}\r\n");
        if let Some(origin) = origin {
            request.push_str(&format!("Origin: {origin}\r\n"));
        }
        request.push_str("Connection: close\r\n\r\n");
        stream.write_all(request.as_bytes()).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        server.join().unwrap();
        response
    }

    #[test]
    fn rejects_missing_token_wrong_host_and_hostile_origin() {
        let port = 43123;
        let shared = test_shared(port);
        let request = |target: &str, host: &str, origin: Option<&str>| HttpRequest {
            method: "GET".to_string(),
            target: target.to_string(),
            headers: [
                ("host".to_string(), host.to_string()),
                ("origin".to_string(), origin.unwrap_or_default().to_string()),
            ]
            .into_iter()
            .filter(|(name, _)| name != "origin" || origin.is_some())
            .collect(),
            body: Vec::new(),
        };

        assert!(!authorized(&request("/", "127.0.0.1:43123", None), &shared));
        assert!(!authorized(
            &request("/?token=secret-token", "localhost:43123", None),
            &shared
        ));
        assert!(!authorized(
            &request(
                "/?token=secret-token",
                "127.0.0.1:43123",
                Some("https://attacker.example")
            ),
            &shared
        ));
        assert!(authorized(
            &request(
                "/?token=secret-token",
                "127.0.0.1:43123",
                Some("http://127.0.0.1:43123")
            ),
            &shared
        ));
    }

    #[test]
    fn mutations_require_exact_origin_and_json_content_type() {
        let port = 43123;
        let shared = test_shared(port);
        let request = |origin: Option<&str>, content_type: Option<&str>| HttpRequest {
            method: "POST".to_string(),
            target: "/api/v1/runs?token=secret-token".to_string(),
            headers: [
                ("host", Some("127.0.0.1:43123")),
                ("origin", origin),
                ("content-type", content_type),
            ]
            .into_iter()
            .filter_map(|(name, value)| value.map(|value| (name.to_string(), value.to_string())))
            .collect(),
            body: b"{}".to_vec(),
        };
        assert!(!authorized_mutation(
            &request(None, Some("application/json")),
            &shared
        ));
        assert!(!authorized_mutation(
            &request(Some("https://attacker.example"), Some("application/json")),
            &shared
        ));
        assert!(!authorized_mutation(
            &request(Some("http://127.0.0.1:43123"), Some("text/plain")),
            &shared
        ));
        assert!(authorized_mutation(
            &request(Some("http://127.0.0.1:43123"), Some("application/json")),
            &shared
        ));
    }

    #[test]
    fn request_reader_collects_a_bounded_json_body() {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            read_request(&mut stream).unwrap().unwrap()
        });
        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
        stream
            .write_all(
                b"POST /api/v1/runs?token=x HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: 16\r\n\r\n{\"filter\":\"one\"}",
            )
            .unwrap();
        let request = server.join().unwrap();
        assert_eq!(request.method, "POST");
        assert_eq!(request.body, br#"{"filter":"one"}"#);
    }

    #[test]
    fn service_never_serves_guessed_project_paths() {
        let probe = raw_request("/../Cargo.toml?token=secret-token", "127.0.0.1:0", None);
        assert!(probe.starts_with("HTTP/1.1 403"));

        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let shared = Arc::new(test_shared(port));
        let shared_for_server = Arc::clone(&shared);
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_connection(stream, &shared_for_server).unwrap();
        });
        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
        let request = format!(
            "GET /../Cargo.toml?token=secret-token HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
        );
        stream.write_all(request.as_bytes()).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        server.join().unwrap();
        assert!(response.starts_with("HTTP/1.1 404"));
        assert!(!response.contains("[package]"));
    }

    #[test]
    fn shell_uses_the_new_workbench_surface() {
        let html = workbench_html("secret-token");
        assert!(html.contains("CURRENT MISSION"));
        assert!(html.contains("/api/v1/state?token="));
        assert!(html.contains("/api/v1/capability"));
        assert!(html.contains("/api/v1/runs"));
        assert!(html.contains("/api/v1/hints"));
        assert!(html.contains("/api/v1/capabilities/next"));
        assert!(html.contains("/api/v1/project-health"));
        assert!(html.contains("/api/v1/project/open-editor"));
        assert!(html.contains("/api/v1/project/open-folder"));
        assert!(html.contains("id=\"health-screen\""));
        assert!(html.contains("id=\"primary-action\""));
        assert!(html.contains("id=\"diagnosis-expected\""));
        assert!(html.contains("id=\"help-levels\""));
        assert!(
            html.contains("Boolean(failure) && state.freshness === \"fresh\" && !state.active_job")
        );
        assert!(html.contains("if (restoreFocus && !action.disabled) action.focus()"));
        assert!(html.contains("if (restoreFocus && !help.disabled) help.focus()"));
        assert!(!html.contains("class=\"primary\" disabled"));
        assert!(html.contains("Local workbench"));
        assert!(!html.contains("Make file discovery deterministic:"));
        assert!(!html.contains("warm"));
    }

    #[test]
    fn configured_editor_is_passed_the_project_without_a_shell() {
        let target = Path::new("/tmp/learner-project");
        let command =
            project_open_command(ProjectOpenKind::Editor, target, Some("code --reuse-window"))
                .unwrap();

        assert_eq!(command.get_program(), "code");
        assert_eq!(
            command
                .get_args()
                .map(|value| value.to_string_lossy().to_string())
                .collect::<Vec<_>>(),
            ["--reuse-window", "/tmp/learner-project"]
        );
    }
}
