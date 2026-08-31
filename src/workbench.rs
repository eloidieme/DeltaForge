use std::collections::{BTreeMap, BTreeSet};
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

const API_VERSION: &str = "v1";
const SERVICE_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-app2");
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
    fn acquire(path: PathBuf) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
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
    default_project_id: Option<String>,
    token: String,
    session_id: String,
    port: u16,
    clients: AtomicUsize,
    last_activity: Mutex<Instant>,
    record_path: PathBuf,
    run_starting: Mutex<BTreeSet<String>>,
    shutting_down: AtomicBool,
    idle_timeout: Duration,
    focus_revision: AtomicUsize,
    focus_target: Mutex<String>,
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
struct StartBenchmarkBody {
    /// Save the measurement to the project's benchmark history. The browser
    /// always saves: a measurement the learner cannot compare later is of no
    /// use to them.
    #[serde(default = "default_true")]
    save: bool,
}

fn default_true() -> bool {
    true
}

/// Creation request from the browser. `parent_directory` is the one path a
/// browser request may supply; `crate::creation::resolve_target` decides
/// whether it is acceptable.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateProjectBody {
    #[serde(default)]
    pack: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    parent_directory: Option<String>,
    #[serde(default)]
    name: String,
    #[serde(default = "default_true")]
    git: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct LearnerNoteBody {
    #[serde(default)]
    text: String,
    #[serde(default)]
    skipped: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct EmptyBody {}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExportReportBody {
    #[serde(default)]
    format: ExportFormat,
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ExportFormat {
    #[default]
    Markdown,
    Html,
    Json,
}

impl From<ExportFormat> for crate::reporting::ReportFormat {
    fn from(format: ExportFormat) -> Self {
        match format {
            ExportFormat::Markdown => Self::Markdown,
            ExportFormat::Html => Self::Html,
            ExportFormat::Json => Self::Json,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ProjectOpenKind {
    Editor,
    Folder,
}

struct ProjectOpenCommand {
    command: Command,
    application: String,
}

#[derive(Debug, Serialize)]
struct ProjectSummary {
    id: String,
    name: String,
    description: String,
    language: String,
    path: PathBuf,
    current_step: String,
    current_step_number: usize,
    total_steps: usize,
    completed_steps: usize,
    last_opened_at: u64,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue: Option<String>,
}

pub fn launch(options: &GlobalOptions) -> Result<()> {
    let root = if options.project_dir.is_some() {
        Some(crate::context::locate_project_root(options)?)
    } else {
        crate::context::locate_project_root(options).ok()
    };
    let startup_lease = StartupLease::acquire(crate::project_registry::startup_lock_path()?)?;
    let registered = root
        .as_deref()
        .map(|root| crate::project_registry::register(root, options.packs_dir.as_deref()))
        .transpose()?;
    let record_path = crate::project_registry::service_record_path()?;
    let mut service = read_compatible_record(&record_path)?;

    if service.is_none() {
        let token = capability_token(root.as_deref().unwrap_or_else(|| Path::new("deltaforge")));
        spawn_service(root.as_deref(), options, &token)?;
        service = wait_for_service(&record_path)?;
    }

    let (record, status) = service.context("the DeltaForge workbench did not start in time")?;
    drop(startup_lease);
    let route = registered.as_ref().map_or_else(
        || "/projects".to_string(),
        |project| format!("/projects/{}/overview", project.id),
    );
    let url = format!(
        "http://127.0.0.1:{}{}?token={}",
        record.port, route, record.token
    );
    if status.clients > 0 && request_focus(&record, &route) {
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

fn spawn_service(root: Option<&Path>, options: &GlobalOptions, token: &str) -> Result<()> {
    let executable = std::env::current_exe().context("failed to locate the deltaforge binary")?;
    let mut command = Command::new(executable);
    if let Some(root) = root {
        command.arg("--project-dir").arg(root);
    }
    command
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

fn request_focus(record: &ServiceRecord, route: &str) -> bool {
    let path = format!(
        "/api/{API_VERSION}/focus?token={}&route={route}",
        record.token
    );
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
    let root = if options.project_dir.is_some() {
        Some(crate::context::locate_project_root(options)?)
    } else {
        None
    };
    let registered = root
        .as_deref()
        .map(|root| crate::project_registry::register(root, options.packs_dir.as_deref()))
        .transpose()?;
    let session_id = format!(
        "{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    if root.is_some() {
        let initial_session_id = registered.as_ref().map_or_else(
            || session_id.clone(),
            |project| format!("{}-{}", session_id, project.id),
        );
        let _ = application::load_workbench_state_for_session(options, &initial_session_id);
        let _ = application::observe_source_changes(options);
    }
    let record_path = crate::project_registry::service_record_path()?;
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
        default_project_id: registered.map(|project| project.id),
        token,
        session_id,
        port,
        clients: AtomicUsize::new(0),
        last_activity: Mutex::new(Instant::now()),
        record_path,
        run_starting: Mutex::new(BTreeSet::new()),
        shutting_down: AtomicBool::new(false),
        idle_timeout,
        focus_revision: AtomicUsize::new(0),
        focus_target: Mutex::new("/projects".to_string()),
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
            let changed = crate::project_registry::list()
                .unwrap_or_default()
                .into_iter()
                .any(|project| {
                    application::observe_source_changes(&options_for_entry(&project))
                        .ok()
                        .flatten()
                        .is_some()
                });
            if changed {
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
            let run_starting = !shared
                .run_starting
                .lock()
                .expect("workbench lock poisoned")
                .is_empty();
            let run_active = run_starting
                || crate::project_registry::list()
                    .unwrap_or_default()
                    .into_iter()
                    .any(|project| {
                        application::run_is_active(&options_for_entry(&project)).unwrap_or(false)
                    });
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
    let project_api = matches!(
        path,
        "/api/v1/project-health"
            | "/api/v1/state"
            | "/api/v1/capability"
            | "/api/v1/events"
            | "/api/v1/runs"
            | "/api/v1/runs/rerun"
            | "/api/v1/runs/cancel"
            | "/api/v1/benchmarks"
            | "/api/v1/predictions"
            | "/api/v1/reflections"
            | "/api/v1/snapshots"
            | "/api/v1/snapshots/preview"
            | "/api/v1/reports"
            | "/api/v1/hints"
            | "/api/v1/capabilities/next"
            | "/api/v1/project/repin-pack"
            | "/api/v1/project/open-editor"
            | "/api/v1/project/open-folder"
    );
    if project_api && project_request(shared, &request).is_err() {
        return respond(
            &mut stream,
            "404 Not Found",
            "application/json",
            r#"{"error":"project_not_found"}"#,
        );
    }

    match (request.method.as_str(), path) {
        ("GET", "/") | ("GET", "/projects") | ("GET", "/catalog") | ("GET", "/create") => respond(
            &mut stream,
            "200 OK",
            "text/html; charset=utf-8",
            &workbench_html(&shared.token),
        ),
        ("GET", route)
            if route.starts_with("/projects/")
                && (route.ends_with("/overview")
                    || route.ends_with("/build")
                    || route.ends_with("/runs")) =>
        {
            let project_id = route.split('/').nth(2).unwrap_or_default();
            if crate::project_registry::resolve(project_id).is_err() {
                return respond(
                    &mut stream,
                    "404 Not Found",
                    "text/plain; charset=utf-8",
                    "Project not found",
                );
            }
            respond(
                &mut stream,
                "200 OK",
                "text/html; charset=utf-8",
                &workbench_html(&shared.token),
            )
        }
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
            let (_, options) = project_request(shared, &request)?;
            let health = application::load_project_health(&options)?;
            respond(
                &mut stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&health)?,
            )
        }
        ("GET", "/api/v1/catalog") => {
            let catalog = application::load_catalog(&GlobalOptions::default())?;
            respond(
                &mut stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&catalog)?,
            )
        }
        ("GET", "/api/v1/workspace") => {
            let body = match crate::creation::default_workspace() {
                Ok(path) => serde_json::json!({"default_directory": path.display().to_string()}),
                Err(error) => serde_json::json!({"error": format!("{error:#}")}),
            };
            respond(&mut stream, "200 OK", "application/json", &body.to_string())
        }
        ("POST", "/api/v1/projects/preflight") | ("POST", "/api/v1/projects") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: CreateProjectBody = match parse_json_body(&request) {
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
            create_project(&mut stream, shared, path, body)
        }
        ("GET", "/api/v1/projects") => {
            let projects = load_project_summaries()?;
            respond(
                &mut stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&projects)?,
            )
        }
        ("GET", "/api/v1/focus") => {
            let route = query_value(&request.target, "route")
                .filter(|route| route.starts_with("/projects"))
                .unwrap_or("/projects")
                .to_string();
            *shared.focus_target.lock().expect("workbench lock poisoned") = route;
            shared.focus_revision.fetch_add(1, Ordering::SeqCst);
            respond(
                &mut stream,
                "202 Accepted",
                "application/json",
                r#"{"status":"focus_requested"}"#,
            )
        }
        ("GET", "/api/v1/state") => {
            let (project_id, options) = project_request(shared, &request)?;
            let state = application::load_workbench_state_for_session(
                &options,
                &project_session_id(shared, &project_id),
            )?;
            let body = serde_json::to_string(&state)?;
            respond(&mut stream, "200 OK", "application/json", &body)
        }
        ("GET", "/api/v1/capability") => {
            let (_, options) = project_request(shared, &request)?;
            let content = application::load_capability_content(&options)?;
            let body = serde_json::to_string(&content)?;
            respond(&mut stream, "200 OK", "application/json", &body)
        }
        ("GET", "/api/v1/app-events") => serve_app_events(stream, shared),
        ("GET", "/api/v1/events") => {
            let (project_id, options) = project_request(shared, &request)?;
            serve_events(stream, shared, &request, project_id, options)
        }
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
            let (project_id, options) = project_request(shared, &request)?;
            start_run(
                &mut stream,
                Arc::clone(shared),
                project_id,
                options,
                body.filter,
            )
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
            let (project_id, options) = project_request(shared, &request)?;
            start_run(
                &mut stream,
                Arc::clone(shared),
                project_id,
                options,
                Some(body.test),
            )
        }
        ("POST", "/api/v1/benchmarks") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: StartBenchmarkBody = match parse_json_body(&request) {
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
            let (project_id, options) = project_request(shared, &request)?;
            start_benchmark_run(
                &mut stream,
                Arc::clone(shared),
                project_id,
                options,
                body.save,
            )
        }
        ("POST", "/api/v1/predictions") | ("POST", "/api/v1/reflections") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: LearnerNoteBody = match parse_json_body(&request) {
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
            let (_, options) = project_request(shared, &request)?;
            let recorded = if path == "/api/v1/predictions" {
                application::record_prediction(&options, body.text, body.skipped)
            } else {
                application::record_reflection(&options, body.text, body.skipped)
            };
            match recorded {
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
        ("GET", "/api/v1/snapshots/preview") => {
            let (_, options) = project_request(shared, &request)?;
            let preview = application::preview_stage_snapshot(&options)?;
            respond(
                &mut stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&preview)?,
            )
        }
        ("POST", "/api/v1/snapshots") => {
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
            let (_, options) = project_request(shared, &request)?;
            match application::create_stage_snapshot(&options, false) {
                Ok(outcome) => respond(
                    &mut stream,
                    "201 Created",
                    "application/json",
                    &serde_json::to_string(&outcome)?,
                ),
                Err(error) => respond(
                    &mut stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
        }
        ("POST", "/api/v1/reports") => {
            if !authorized_mutation(&request, shared) {
                return respond(
                    &mut stream,
                    "403 Forbidden",
                    "application/json",
                    r#"{"error":"forbidden"}"#,
                );
            }
            let body: ExportReportBody = match parse_json_body(&request) {
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
            let (_, options) = project_request(shared, &request)?;
            match application::export_report(&options, body.format.into()) {
                Ok(exported) => respond(
                    &mut stream,
                    "200 OK",
                    "application/json",
                    &serde_json::to_string(&exported)?,
                ),
                Err(error) => respond(
                    &mut stream,
                    "409 Conflict",
                    "application/json",
                    &serde_json::json!({"error": format!("{error:#}")}).to_string(),
                ),
            }
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
            let (_, options) = project_request(shared, &request)?;
            cancel_run(&mut stream, &options)
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
            let (_, options) = project_request(shared, &request)?;
            match application::reveal_next_hint(&options) {
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
            let (_, options) = project_request(shared, &request)?;
            match application::begin_next_capability(&options) {
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
            let (_, options) = project_request(shared, &request)?;
            match application::repin_current_pack(&options) {
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
            let (_, options) = project_request(shared, &request)?;
            open_project(
                &mut stream,
                shared,
                &request,
                &options,
                ProjectOpenKind::Editor,
            )
        }
        ("POST", "/api/v1/project/open-folder") => {
            let (_, options) = project_request(shared, &request)?;
            open_project(
                &mut stream,
                shared,
                &request,
                &options,
                ProjectOpenKind::Folder,
            )
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

fn start_run(
    stream: &mut TcpStream,
    shared: Arc<Shared>,
    project_id: String,
    options: GlobalOptions,
    filter: Option<String>,
) -> Result<()> {
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
        || starting.contains(&project_id)
        || application::run_is_active(&options)?
    {
        return respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"run_already_active"}"#,
        );
    }
    starting.insert(project_id.clone());
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
        if let Err(error) = application::run_tests(&options, request, &mut sink)
            && !format!("{error:#}").contains("already active")
        {
            let _ = application::publish_event(
                &options,
                &application::RunEvent::JobInterrupted {
                    job_id: "pending".to_string(),
                    reason: format!("{error:#}"),
                },
            );
        }
        worker
            .run_starting
            .lock()
            .expect("workbench lock poisoned")
            .remove(&project_id);
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

/// Serve both halves of the creation flow. `preflight` reports what is wrong
/// without changing anything; `create` writes the project and registers it.
/// Both resolve the target through the same guarded function, so the browser's
/// earlier preflight is a convenience and never the authority.
fn create_project(
    stream: &mut TcpStream,
    shared: &Shared,
    path: &str,
    body: CreateProjectBody,
) -> Result<()> {
    // Creation is the only browser operation that names a filesystem location.
    // Refuse an oversized field before it reaches path resolution.
    if body.pack.len() > 128
        || body.language.len() > 64
        || body.name.len() > 128
        || body
            .parent_directory
            .as_ref()
            .is_some_and(|parent| parent.len() > 4096)
    {
        return respond(
            stream,
            "400 Bad Request",
            "application/json",
            r#"{"error":"invalid_request"}"#,
        );
    }
    let parent = body.parent_directory.as_deref().map(Path::new);
    let options = GlobalOptions::default();

    if path.ends_with("/preflight") {
        return match application::preflight_project(
            &options,
            &body.pack,
            &body.language,
            parent,
            &body.name,
        ) {
            Ok(preflight) => respond(
                stream,
                "200 OK",
                "application/json",
                &serde_json::to_string(&preflight)?,
            ),
            Err(error) => respond(
                stream,
                "400 Bad Request",
                "application/json",
                &serde_json::json!({"error": format!("{error:#}")}).to_string(),
            ),
        };
    }

    if shared.shutting_down.load(Ordering::SeqCst) {
        return respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"service_stopping"}"#,
        );
    }
    let request = application::CreateProjectRequest {
        pack: body.pack,
        language: body.language,
        parent_directory: parent.map(Path::to_path_buf),
        name: body.name,
        git: body.git,
        stage: None,
    };
    match application::create_project(&options, request) {
        Ok(created) => respond(
            stream,
            "201 Created",
            "application/json",
            &serde_json::to_string(&created)?,
        ),
        Err(error) => respond(
            stream,
            "409 Conflict",
            "application/json",
            &serde_json::json!({"error": format!("{error:#}")}).to_string(),
        ),
    }
}

/// Start a benchmark job on the same guard rails as a test run: one job per
/// project at a time, run on a worker thread, progress delivered through the
/// project's event stream rather than this response.
fn start_benchmark_run(
    stream: &mut TcpStream,
    shared: Arc<Shared>,
    project_id: String,
    options: GlobalOptions,
    save: bool,
) -> Result<()> {
    let mut starting = shared.run_starting.lock().expect("workbench lock poisoned");
    if shared.shutting_down.load(Ordering::SeqCst)
        || starting.contains(&project_id)
        || application::run_is_active(&options)?
    {
        return respond(
            stream,
            "409 Conflict",
            "application/json",
            r#"{"error":"run_already_active"}"#,
        );
    }
    starting.insert(project_id.clone());
    drop(starting);

    let worker = Arc::clone(&shared);
    std::thread::spawn(move || {
        let request = application::BenchmarkRunRequest {
            stage: None,
            all: false,
            iterations: None,
            warmup: None,
            save,
            compare: true,
            trigger: application::RunTrigger::Workbench,
        };
        let mut sink = application::NullEventSink;
        if let Err(error) = application::run_benchmarks(&options, request, &mut sink)
            && !format!("{error:#}").contains("already active")
        {
            let _ = application::publish_event(
                &options,
                &application::RunEvent::JobInterrupted {
                    job_id: "pending".to_string(),
                    reason: format!("{error:#}"),
                },
            );
        }
        worker
            .run_starting
            .lock()
            .expect("workbench lock poisoned")
            .remove(&project_id);
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

fn cancel_run(stream: &mut TcpStream, options: &GlobalOptions) -> Result<()> {
    match application::cancel_active_run(options) {
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
    let run_active = !run_starting.is_empty()
        || crate::project_registry::list()
            .unwrap_or_default()
            .into_iter()
            .any(|project| {
                application::run_is_active(&options_for_entry(&project)).unwrap_or(false)
            });
    if run_active {
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
    options: &GlobalOptions,
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
    let target = match application::project_open_target(options) {
        Ok(target) => target,
        Err(error) => {
            return respond(
                stream,
                "409 Conflict",
                "application/json",
                &serde_json::json!({"error": format!("could not locate project: {error}")})
                    .to_string(),
            );
        }
    };
    let editor = std::env::var("VISUAL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| std::env::var("EDITOR").ok());
    let resolved = match project_open_command(kind, &target, editor.as_deref()) {
        Ok(resolved) => resolved,
        Err(error) => {
            return respond(
                stream,
                "409 Conflict",
                "application/json",
                &serde_json::json!({"error": format!("could not open project: {error}")})
                    .to_string(),
            );
        }
    };
    match launch_project_command(resolved.command, &resolved.application) {
        Ok(()) => respond(
            stream,
            "202 Accepted",
            "application/json",
            &serde_json::json!({
                "status": "opened",
                "application": resolved.application,
            })
            .to_string(),
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
) -> Result<ProjectOpenCommand> {
    if matches!(kind, ProjectOpenKind::Editor)
        && let Some(editor) = editor
    {
        let mut parts = editor.split_whitespace();
        let program = parts
            .next()
            .filter(|part| !part.is_empty())
            .context("the configured VISUAL or EDITOR command is empty")?;
        if !terminal_only_editor(program) {
            let mut command = Command::new(program);
            command.args(parts).arg(target);
            return Ok(ProjectOpenCommand {
                command,
                application: program.to_string(),
            });
        }
    }

    #[cfg(target_os = "macos")]
    {
        let mut command = Command::new("open");
        if matches!(kind, ProjectOpenKind::Editor) {
            let application = [
                "Cursor",
                "Visual Studio Code",
                "Zed",
                "Sublime Text",
                "Nova",
                "RustRover",
            ]
            .into_iter()
            .find(|application| macos_application_exists(application))
            .context(
                "no supported graphical editor was found; set VISUAL or EDITOR to a GUI editor command such as 'cursor', 'code', or 'zed'",
            )?;
            command.arg("-a").arg(application).arg(target);
            return Ok(ProjectOpenCommand {
                command,
                application: application.to_string(),
            });
        }
        command.arg(target);
        Ok(ProjectOpenCommand {
            command,
            application: "Finder".to_string(),
        })
    }
    #[cfg(target_os = "linux")]
    {
        let (program, application) = if matches!(kind, ProjectOpenKind::Editor) {
            [
                ("cursor", "Cursor"),
                ("code", "Visual Studio Code"),
                ("zed", "Zed"),
                ("codium", "VSCodium"),
                ("subl", "Sublime Text"),
            ]
            .into_iter()
            .find(|(program, _)| command_exists(program))
            .context(
                "no supported graphical editor was found; set VISUAL or EDITOR to a GUI editor command",
            )?
        } else {
            ("xdg-open", "file manager")
        };
        let mut command = Command::new(program);
        command.arg(target);
        Ok(ProjectOpenCommand {
            command,
            application: application.to_string(),
        })
    }
    #[cfg(windows)]
    {
        let (program, application) = if matches!(kind, ProjectOpenKind::Editor) {
            [
                ("cursor.cmd", "Cursor"),
                ("code.cmd", "Visual Studio Code"),
                ("zed.exe", "Zed"),
                ("codium.cmd", "VSCodium"),
                ("subl.exe", "Sublime Text"),
            ]
            .into_iter()
            .find(|(program, _)| command_exists(program))
            .context(
                "no supported graphical editor was found; set VISUAL or EDITOR to a GUI editor command",
            )?
        } else {
            ("explorer", "File Explorer")
        };
        let mut command = Command::new(program);
        command.arg(target);
        Ok(ProjectOpenCommand {
            command,
            application: application.to_string(),
        })
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    {
        let _ = kind;
        let _ = target;
        bail!("opening a project is unsupported on this platform")
    }
}

fn terminal_only_editor(program: &str) -> bool {
    Path::new(program)
        .file_stem()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "vi" | "vim" | "nvim" | "nano" | "emacs"))
}

fn launch_project_command(mut command: Command, application: &str) -> Result<()> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut child = command
        .spawn()
        .with_context(|| format!("failed to start {application}"))?;
    let deadline = Instant::now() + Duration::from_millis(750);
    loop {
        match child.try_wait()? {
            Some(status) if status.success() => return Ok(()),
            Some(status) => bail!("{application} exited with status {status}"),
            None if Instant::now() >= deadline => {
                std::thread::spawn(move || {
                    let _ = child.wait();
                });
                return Ok(());
            }
            None => std::thread::sleep(Duration::from_millis(20)),
        }
    }
}

#[cfg(target_os = "macos")]
fn macos_application_exists(application: &str) -> bool {
    Command::new("open")
        .arg("-Ra")
        .arg(application)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

#[cfg(any(target_os = "linux", windows))]
fn command_exists(program: &str) -> bool {
    std::env::var_os("PATH").is_some_and(|paths| {
        std::env::split_paths(&paths).any(|directory| directory.join(program).is_file())
    })
}

fn serve_events(
    mut stream: TcpStream,
    shared: &Shared,
    request: &HttpRequest,
    project_id: String,
    options: GlobalOptions,
) -> Result<()> {
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
        .unwrap_or(crate::run_journal::cursor(project_root(&options)?)?);
    let mut previous = String::new();
    let mut focus_revision = shared.focus_revision.load(Ordering::SeqCst);

    loop {
        let current_focus_revision = shared.focus_revision.load(Ordering::SeqCst);
        if current_focus_revision != focus_revision {
            focus_revision = current_focus_revision;
            let target = shared
                .focus_target
                .lock()
                .expect("workbench lock poisoned")
                .clone();
            let payload = format!(
                "event: focus\ndata: {}\n\n",
                serde_json::json!({"route": target})
            );
            if stream.write_all(payload.as_bytes()).is_err() {
                return Ok(());
            }
        }
        for entry in crate::run_journal::entries_after(project_root(&options)?, cursor)? {
            let serialized = serde_json::to_string(&entry.event)?;
            let payload = format!("id: {}\nevent: run\ndata: {serialized}\n\n", entry.id);
            if stream.write_all(payload.as_bytes()).is_err() {
                return Ok(());
            }
            cursor = entry.id;
        }
        let state = application::load_workbench_state_for_session(
            &options,
            &project_session_id(shared, &project_id),
        )?;
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

fn serve_app_events(mut stream: TcpStream, shared: &Shared) -> Result<()> {
    stream.set_read_timeout(None)?;
    stream.write_all(
        concat!(
            "HTTP/1.1 200 OK\r\n",
            "Content-Type: text/event-stream\r\n",
            "Cache-Control: no-cache\r\n",
            "X-Content-Type-Options: nosniff\r\n",
            "Referrer-Policy: no-referrer\r\n",
            "Connection: keep-alive\r\n\r\n"
        )
        .as_bytes(),
    )?;
    struct ClientGuard<'a>(&'a AtomicUsize);
    impl Drop for ClientGuard<'_> {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::SeqCst);
        }
    }
    shared.clients.fetch_add(1, Ordering::SeqCst);
    let _guard = ClientGuard(&shared.clients);
    let mut focus_revision = shared.focus_revision.load(Ordering::SeqCst);
    loop {
        let current = shared.focus_revision.load(Ordering::SeqCst);
        let payload = if current != focus_revision {
            focus_revision = current;
            let target = shared
                .focus_target
                .lock()
                .expect("workbench lock poisoned")
                .clone();
            format!(
                "event: focus\ndata: {}\n\n",
                serde_json::json!({"route": target})
            )
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

fn options_for_entry(project: &crate::project_registry::RegisteredProject) -> GlobalOptions {
    GlobalOptions {
        project_dir: Some(project.path.clone()),
        packs_dir: project.packs_dir.clone(),
    }
}

fn project_request(shared: &Shared, request: &HttpRequest) -> Result<(String, GlobalOptions)> {
    let id = query_value(&request.target, "project")
        .map(str::to_string)
        .or_else(|| shared.default_project_id.clone())
        .context("select a registered project first")?;
    let project = crate::project_registry::resolve(&id)?;
    Ok((id, options_for_entry(&project)))
}

fn project_session_id(shared: &Shared, project_id: &str) -> String {
    format!("{}-{project_id}", shared.session_id)
}

fn load_project_summaries() -> Result<Vec<ProjectSummary>> {
    crate::project_registry::list()?
        .into_iter()
        .map(|project| {
            let options = options_for_entry(&project);
            let health = application::load_project_health(&options)?;
            let state_path = project.path.join(".deltaforge").join("state.json");
            let state = crate::state::ProjectState::read_from(&state_path).ok();
            let content = application::load_capability_content(&options).ok();
            let current_position = content
                .as_ref()
                .and_then(|content| {
                    content.roadmap.iter().find(|step| {
                        matches!(step.status, crate::capability::RoadmapStatus::Current)
                    })
                })
                .map_or(0, |step| step.position);
            let healthy = matches!(health.status, application::ProjectHealthStatus::Healthy);
            Ok(ProjectSummary {
                id: project.id,
                name: content
                    .as_ref()
                    .map(|content| content.project_overview.name.clone())
                    .or_else(|| state.as_ref().map(|state| state.project.clone()))
                    .unwrap_or_else(|| "Unavailable project".to_string()),
                description: content
                    .as_ref()
                    .map(|content| content.project_overview.description.clone())
                    .unwrap_or_else(|| {
                        "This project needs attention before it can continue.".to_string()
                    }),
                language: state
                    .as_ref()
                    .map(|state| state.language.clone())
                    .unwrap_or_default(),
                path: project.path,
                current_step: content
                    .as_ref()
                    .map(|content| content.title.clone())
                    .unwrap_or_else(|| "Unavailable".to_string()),
                current_step_number: current_position,
                total_steps: content.as_ref().map_or(0, |content| content.roadmap.len()),
                completed_steps: state
                    .as_ref()
                    .map_or(0, |state| state.completed_stages.len()),
                last_opened_at: project.last_opened_at,
                status: if healthy {
                    "healthy"
                } else {
                    "needs_attention"
                },
                issue: health.issue.map(|issue| issue.title),
            })
        })
        .collect()
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
/// The single-page workbench, assembled at compile time from three source
/// files. It is served inline rather than as separate assets so the page
/// carries no subresource requests and the capability token stays in exactly
/// one place.
fn workbench_html(token: &str) -> String {
    include_str!("ui/index.html")
        .replace("__STYLE__", include_str!("ui/app.css"))
        .replace("__SCRIPT__", include_str!("ui/app.js"))
        .replace("__TOKEN_JSON__", &serde_json::json!(token).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_shared(port: u16) -> Shared {
        Shared {
            default_project_id: None,
            token: "secret-token".to_string(),
            session_id: "test-session".to_string(),
            port,
            clients: AtomicUsize::new(0),
            last_activity: Mutex::new(Instant::now()),
            record_path: PathBuf::from("unused-workbench-record.json"),
            run_starting: Mutex::new(BTreeSet::new()),
            shutting_down: AtomicBool::new(false),
            idle_timeout: IDLE_TIMEOUT,
            focus_revision: AtomicUsize::new(0),
            focus_target: Mutex::new("/projects".to_string()),
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
    fn the_shell_carries_every_surface_and_its_api() {
        let html = workbench_html("secret-token");
        // Composition: all three source files reached the page.
        assert!(html.contains("--accent"), "stylesheet is missing");
        assert!(html.contains("renderRoute()"), "script is missing");
        assert!(html.contains(r#"const token = "secret-token";"#));
        assert!(!html.contains("__TOKEN_JSON__"));
        assert!(!html.contains("__STYLE__"));
        assert!(!html.contains("__SCRIPT__"));

        // Every screen the 1.0 journey needs.
        for id in [
            "projects-screen",
            "catalog-screen",
            "create-screen",
            "overview-screen",
            "build-screen",
            "performance-screen",
            "runs-screen",
            "health-screen",
        ] {
            assert!(
                html.contains(&format!("id=\"{id}\"")),
                "missing screen {id}"
            );
        }

        // Every route the page calls must exist in the router above.
        for route in [
            "/api/v1/projects",
            "/api/v1/catalog",
            "/api/v1/workspace",
            "/api/v1/projects/preflight",
            "/api/v1/state",
            "/api/v1/capability",
            "/api/v1/project-health",
            "/api/v1/runs",
            "/api/v1/runs/cancel",
            "/api/v1/runs/rerun",
            "/api/v1/benchmarks",
            "/api/v1/predictions",
            "/api/v1/reflections",
            "/api/v1/snapshots",
            "/api/v1/snapshots/preview",
            "/api/v1/reports",
            "/api/v1/hints",
            "/api/v1/capabilities/next",
            "/api/v1/project/open-editor",
            "/api/v1/project/open-folder",
        ] {
            assert!(html.contains(route), "page never calls {route}");
        }

        // Light and dark are both first-class, and motion respects the
        // learner's preference.
        assert!(html.contains("prefers-color-scheme: dark"));
        assert!(html.contains("[data-theme=\"dark\"]"));
        assert!(html.contains("prefers-reduced-motion: reduce"));
        assert!(html.contains("skip-link"));

        // No surface tells the learner to open a terminal.
        assert!(!html.contains("deltaforge init"));
        assert!(!html.contains("deltaforge bench"));
        assert!(!html.contains("deltaforge test"));
    }

    #[test]
    fn configured_editor_is_passed_the_project_without_a_shell() {
        let target = Path::new("/tmp/learner-project");
        let resolved =
            project_open_command(ProjectOpenKind::Editor, target, Some("code --reuse-window"))
                .unwrap();

        assert_eq!(resolved.application, "code");
        assert_eq!(resolved.command.get_program(), "code");
        assert_eq!(
            resolved
                .command
                .get_args()
                .map(|value| value.to_string_lossy().to_string())
                .collect::<Vec<_>>(),
            ["--reuse-window", "/tmp/learner-project"]
        );
        assert!(terminal_only_editor("/usr/bin/nvim"));
        assert!(!terminal_only_editor("cursor"));
    }
}
